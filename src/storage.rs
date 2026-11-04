//! object_store-backed MinIO/S3 access: bounded fetch to tempfile, upload
//! from bytes. All tool IO flows through these two functions.

use std::path::Path;

use object_store::aws::AmazonS3Builder;
use object_store::path::Path as StorePath;
use object_store::{ObjectStore, PutPayload};

use crate::config::S3Config;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("storage is not configured on this server; ask the operator to set S3_BUCKET and credentials")]
    NotConfigured,
    #[error("file not found or not accessible")]
    NotFound,
    #[error("download exceeded the {0} byte limit")]
    TooLarge(u64),
    #[error("storage operation failed")]
    Failed,
}

/// A local tempfile guard: the file is deleted when the guard drops. The
/// engine reads through `as_path()`; ownership keeps the file alive.
pub struct TempFile {
    inner: tempfile::NamedTempFile,
}

impl TempFile {
    pub fn as_path(&self) -> &Path {
        self.inner.path()
    }
}

/// Build the object_store client from the resolved S3 config.
pub fn client(s3: &S3Config) -> Result<impl ObjectStore, StorageError> {
    let mut builder = AmazonS3Builder::new()
        .with_bucket_name(&s3.bucket)
        .with_access_key_id(&s3.access_key_id)
        .with_secret_access_key(&s3.secret_access_key)
        .with_region(&s3.region)
        .with_virtual_hosted_style_request(!s3.use_path_style);
    if !s3.endpoint_url.is_empty() {
        builder = builder
            .with_endpoint(&s3.endpoint_url)
            // MinIO in-cluster is http://service:9000
            .with_allow_http(true);
    }
    builder.build().map_err(|_| StorageError::Failed)
}

fn store_path(key: &str) -> StorePath {
    StorePath::from(key)
}

/// Download one object into a fresh local tempfile. Fails NotFound for a
/// missing key, TooLarge once more than `max_bytes` have been read.
pub async fn fetch_to_tempfile(
    store: &dyn ObjectStore,
    key: &str,
    max_bytes: u64,
) -> Result<TempFile, StorageError> {
    use futures::StreamExt;
    use tokio::io::AsyncWriteExt;

    let location = store_path(key);
    let get = store.get(&location).await.map_err(|e| match e {
        object_store::Error::NotFound { .. } => StorageError::NotFound,
        _ => StorageError::Failed,
    })?;
    let size = get.meta().size;
    if size > max_bytes {
        return Err(StorageError::TooLarge(max_bytes));
    }
    let mut reader = get.into_stream();

    let inner = tempfile::Builder::new()
        .prefix("excel-mcp-in-")
        .suffix(".xlsx")
        .tempfile()
        .map_err(|_| StorageError::Failed)?;
    let file = inner.reopen().map_err(|_| StorageError::Failed)?;
    let mut writer = tokio::io::BufWriter::new(tokio::fs::File::from_std(file));

    let mut total: u64 = 0;
    while let Some(chunk) = reader.next().await {
        let bytes = chunk.map_err(|_| StorageError::Failed)?;
        total += bytes.len() as u64;
        if total > max_bytes {
            return Err(StorageError::TooLarge(max_bytes));
        }
        writer
            .write_all(&bytes)
            .await
            .map_err(|_| StorageError::Failed)?;
    }
    writer.flush().await.map_err(|_| StorageError::Failed)?;

    Ok(TempFile { inner })
}

/// Generate a fresh output key under `prefix`: `<prefix><uuid>.xlsx`.
/// Collision-free by construction.
pub fn generated_output_key(prefix: &str) -> String {
    let p = if prefix.ends_with('/') {
        prefix.to_string()
    } else {
        format!("{prefix}/")
    };
    format!("{}{}.xlsx", p, uuid::Uuid::new_v4())
}

/// Upload bytes to `key`.
pub async fn upload_bytes(
    store: &dyn ObjectStore,
    key: &str,
    bytes: Vec<u8>,
) -> Result<(), StorageError> {
    store
        .put(&store_path(key), PutPayload::from_bytes(bytes.into()))
        .await
        .map_err(|_| StorageError::Failed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s3() -> S3Config {
        S3Config {
            bucket: "b".into(),
            region: "us-east-1".into(),
            access_key_id: "ak".into(),
            secret_access_key: "sk".into(),
            endpoint_url: "http://minio:9000".into(),
            use_path_style: true,
            allowed_key_prefixes: vec![],
        }
    }

    #[test]
    fn client_builds_with_minio_endpoint() {
        // Build only — no network. Proves path-style + allow_http are accepted.
        assert!(client(&s3()).is_ok());
    }

    #[test]
    fn output_key_generation() {
        let generated = generated_output_key("excel-mcp-out/");
        assert!(generated.starts_with("excel-mcp-out/"));
        assert!(generated.ends_with(".xlsx"));
    }

    #[test]
    fn output_key_generation_adds_slash() {
        let generated = generated_output_key("excel-mcp-out");
        assert!(generated.starts_with("excel-mcp-out/"));
    }
}
