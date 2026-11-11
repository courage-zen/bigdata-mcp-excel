//! object_store-backed MinIO/S3 access: bounded fetch to tempfile, upload
//! from bytes. All tool IO flows through these two functions.

use std::path::Path;
use std::time::Duration;

use object_store::aws::AmazonS3Builder;
use object_store::path::Path as StorePath;
use object_store::{ClientOptions, ObjectStore, ObjectStoreExt, PutPayload};

use crate::config::S3Config;

/// Standard §4: whole-download monotonic wall clock.
const DOWNLOAD_WALL: Duration = Duration::from_secs(60);
/// Standard §4: connection establishment timeout.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// Standard §4: per-request read timeout.
const READ_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("file not found or not accessible")]
    NotFound,
    #[error("download exceeded the {0} byte limit")]
    TooLarge(u64),
    #[error("download timed out")]
    Timeout,
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
        .with_virtual_hosted_style_request(!s3.use_path_style)
        .with_client_options(
            ClientOptions::default()
                .with_connect_timeout(CONNECT_TIMEOUT)
                .with_timeout(READ_TIMEOUT),
        );
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
/// missing key, TooLarge once more than `max_bytes` have been read, Timeout
/// once the whole download exceeds the standard §4 wall clock.
pub async fn fetch_to_tempfile(
    store: &dyn ObjectStore,
    key: &str,
    max_bytes: u64,
) -> Result<TempFile, StorageError> {
    use futures::StreamExt;
    use tokio::io::AsyncWriteExt;

    let location = store_path(key);
    let inner = tempfile::Builder::new()
        .prefix("excel-mcp-in-")
        .suffix(".xlsx")
        .tempfile()
        .map_err(|_| StorageError::Failed)?;
    let file = inner.reopen().map_err(|_| StorageError::Failed)?;
    let mut writer = tokio::io::BufWriter::new(tokio::fs::File::from_std(file));

    let mut total: u64 = 0;
    // The whole download — metadata get, size check, and the read loop —
    // is bounded by the standard §4 wall clock. The tempfile guard is
    // dropped (file deleted) on every early return.
    let result = tokio::time::timeout(DOWNLOAD_WALL, async {
        let get = store.get(&location).await.map_err(|e| match e {
            object_store::Error::NotFound { .. } => StorageError::NotFound,
            _ => StorageError::Failed,
        })?;
        let size = get.meta.size;
        if size > max_bytes {
            return Err(StorageError::TooLarge(max_bytes));
        }
        let mut reader = get.into_stream();
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
        writer.flush().await.map_err(|_| StorageError::Failed)
    })
    .await;

    match result {
        Ok(Ok(())) => Ok(TempFile { inner }),
        Ok(Err(e)) => Err(e),
        Err(_elapsed) => Err(StorageError::Timeout),
    }
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
        .map(|_| ())
        .map_err(|_| StorageError::Failed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream::{self, BoxStream};

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
    fn client_carries_layered_timeouts() {
        // We cannot read ClientOptions back out of a built store, so the
        // test pins the constants and the builder wiring side by side:
        // constants are the standard's numbers, and client() builds.
        use std::time::Duration;
        assert_eq!(super::CONNECT_TIMEOUT, Duration::from_secs(10));
        assert_eq!(super::READ_TIMEOUT, Duration::from_secs(60));
        assert!(client(&s3()).is_ok());
    }

    #[tokio::test]
    async fn fetch_roundtrips_in_memory_object() {
        use object_store::memory::InMemory;

        let store = InMemory::new();
        let key = "workspaces/ws-1/report.xlsx";
        let payload: object_store::PutPayload = b"PK\x03\x04fake-xlsx-bytes".to_vec().into();
        store
            .put(&object_store::path::Path::from(key), payload)
            .await
            .expect("seed object");

        let tmp = fetch_to_tempfile(&store, key, 1024 * 1024)
            .await
            .expect("fetch succeeds");
        let fetched = std::fs::read(tmp.as_path()).expect("read tempfile");
        assert_eq!(fetched, b"PK\x03\x04fake-xlsx-bytes");
    }

    /// A store whose GET stream never yields: fetch must hit the wall-clock.
    #[derive(Debug)]
    struct HangingStore;

    impl std::fmt::Display for HangingStore {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "HangingStore")
        }
    }

    #[async_trait::async_trait]
    impl object_store::ObjectStore for HangingStore {
        async fn put_opts(
            &self,
            _location: &object_store::path::Path,
            _payload: object_store::PutPayload,
            _opts: object_store::PutOptions,
        ) -> object_store::Result<object_store::PutResult> {
            unreachable!("not exercised")
        }

        async fn put_multipart_opts(
            &self,
            _location: &object_store::path::Path,
            _opts: object_store::PutMultipartOptions,
        ) -> object_store::Result<Box<dyn object_store::MultipartUpload>> {
            unreachable!("not exercised")
        }

        async fn get_opts(
            &self,
            _location: &object_store::path::Path,
            _options: object_store::GetOptions,
        ) -> object_store::Result<object_store::GetResult> {
            // Stream that never produces an item and never ends.
            Ok(object_store::GetResult {
                payload: object_store::GetResultPayload::Stream(Box::pin(stream::pending())),
                meta: object_store::ObjectMeta {
                    location: object_store::path::Path::default(),
                    last_modified: Default::default(),
                    size: 1,
                    e_tag: None,
                    version: None,
                },
                range: Default::default(),
                attributes: Default::default(),
                extensions: Default::default(),
            })
        }

        fn delete_stream(
            &self,
            _locations: BoxStream<'static, object_store::Result<object_store::path::Path>>,
        ) -> BoxStream<'static, object_store::Result<object_store::path::Path>> {
            unreachable!("not exercised")
        }

        fn list(
            &self,
            _prefix: Option<&object_store::path::Path>,
        ) -> BoxStream<'static, object_store::Result<object_store::ObjectMeta>> {
            unreachable!("not exercised")
        }

        async fn list_with_delimiter(
            &self,
            _prefix: Option<&object_store::path::Path>,
        ) -> object_store::Result<object_store::ListResult> {
            unreachable!("not exercised")
        }

        async fn copy_opts(
            &self,
            _from: &object_store::path::Path,
            _to: &object_store::path::Path,
            _options: object_store::CopyOptions,
        ) -> object_store::Result<()> {
            unreachable!("not exercised")
        }
    }

    #[tokio::test(start_paused = true)]
    async fn fetch_times_out_on_hanging_stream() {
        // Tokio time is frozen: the 60s wall clock fires without waiting.
        // (Match on the Result directly: TempFile is not Debug, so no
        // unwrap_err — {err:?} still works, StorageError is Debug.)
        let result = fetch_to_tempfile(&HangingStore, "a/b.xlsx", 1024).await;
        let err = match result {
            Ok(_) => panic!("hanging stream must time out, not complete"),
            Err(e) => e,
        };
        assert!(matches!(err, StorageError::Timeout), "got: {err:?}");
    }
}
