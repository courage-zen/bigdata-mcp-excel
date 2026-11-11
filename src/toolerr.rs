//! Tool-result-layer error taxonomy: one enum, one code per failure class,
//! rendered as {"error_code": ..., "error": ...} in tool results. The
//! boundary is "our code vs the framework": serde deserialization failures
//! stay protocol-level (rmcp); everything the tool body can fail on lands
//! here with a machine-readable code.

use crate::s3keys::InvalidKey;
use crate::storage::StorageError;

/// S3 usage standard §5 codes plus service-level codes. Every business
/// failure a tool body can produce maps to exactly one variant.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error(
        "S3 is not configured on this server; ask the operator to set S3_BUCKET and credentials"
    )]
    S3Unavailable,
    #[error("file not found or not accessible")]
    S3ObjectNotFound,
    // Plain payload, not #[source]: String does not implement
    // std::error::Error. Display stays the fixed "download failed" — the
    // payload never reaches the message (pinned by the sanitization tests).
    #[error("download failed")]
    S3FetchFailed(String),
    #[error("upload failed")]
    S3UploadFailed,
    #[error("object exceeds the {0} byte limit")]
    S3ObjectTooLarge(u64),
    #[error("invalid s3 key: {0}")]
    S3InvalidKey(String),
    #[error("invalid arguments: {0}")]
    InvalidArguments(String),
    #[error("processing timed out")]
    EngineTimeout,
    #[error("internal error")]
    Internal(String),
}

impl ToolError {
    /// Machine-readable code (snake/upper case, stable contract).
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::S3Unavailable => "S3_UNAVAILABLE",
            Self::S3ObjectNotFound => "S3_OBJECT_NOT_FOUND",
            Self::S3FetchFailed(_) => "S3_FETCH_FAILED",
            Self::S3UploadFailed => "S3_UPLOAD_FAILED",
            Self::S3ObjectTooLarge(_) => "S3_OBJECT_TOO_LARGE",
            Self::S3InvalidKey(_) => "S3_INVALID_KEY",
            Self::InvalidArguments(_) => "INVALID_ARGUMENTS",
            Self::EngineTimeout => "ENGINE_TIMEOUT",
            Self::Internal(_) => "INTERNAL",
        }
    }
}

impl From<StorageError> for ToolError {
    fn from(e: StorageError) -> Self {
        match e {
            StorageError::NotFound => Self::S3ObjectNotFound,
            StorageError::TooLarge(limit) => Self::S3ObjectTooLarge(limit),
            // Download-stage failures: network, 403, or wall-clock timeout.
            StorageError::Failed => Self::S3FetchFailed(e.to_string()),
        }
    }
}

impl From<InvalidKey> for ToolError {
    fn from(e: InvalidKey) -> Self {
        // InvalidKey's message is already sanitized (s3keys.rs).
        Self::S3InvalidKey(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_match_the_standard() {
        assert_eq!(ToolError::S3Unavailable.error_code(), "S3_UNAVAILABLE");
        assert_eq!(
            ToolError::S3ObjectNotFound.error_code(),
            "S3_OBJECT_NOT_FOUND"
        );
        assert_eq!(
            ToolError::S3FetchFailed(String::new()).error_code(),
            "S3_FETCH_FAILED"
        );
        assert_eq!(ToolError::S3UploadFailed.error_code(), "S3_UPLOAD_FAILED");
        assert_eq!(
            ToolError::S3ObjectTooLarge(100).error_code(),
            "S3_OBJECT_TOO_LARGE"
        );
        assert_eq!(
            ToolError::S3InvalidKey(String::new()).error_code(),
            "S3_INVALID_KEY"
        );
        assert_eq!(
            ToolError::InvalidArguments(String::new()).error_code(),
            "INVALID_ARGUMENTS"
        );
        assert_eq!(ToolError::EngineTimeout.error_code(), "ENGINE_TIMEOUT");
        assert_eq!(ToolError::Internal(String::new()).error_code(), "INTERNAL");
    }

    #[test]
    fn messages_are_sanitized() {
        // ToolError adds nothing beyond its own fixed strings plus the
        // payload it was constructed with; end-to-end sanitization is
        // pinned at construction sites in tools.rs tests.
        let cases = vec![
            ToolError::S3Unavailable,
            ToolError::S3ObjectNotFound,
            ToolError::S3FetchFailed("net".into()),
            ToolError::S3UploadFailed,
            ToolError::S3ObjectTooLarge(100),
            ToolError::S3InvalidKey("bad".into()),
            ToolError::InvalidArguments("bad".into()),
            ToolError::EngineTimeout,
            ToolError::Internal("boom".into()),
        ];
        for e in cases {
            let msg = e.to_string();
            assert!(!msg.contains("minio"), "leak: {msg}");
            assert!(!msg.contains("uploads"), "leak: {msg}");
            assert!(!msg.contains("127.0.0.1"), "leak: {msg}");
            assert!(!msg.is_empty());
        }
    }

    #[test]
    fn fetch_failed_display_excludes_leaky_payload() {
        // Display is the wire message (render uses e.to_string() for the
        // "error" field), so even a directly-constructed leaky payload must
        // never reach it.
        let e = ToolError::S3FetchFailed("http://minio:9000/uploads broke".into());
        assert_eq!(e.to_string(), "download failed");
        // The From<StorageError> conversion must land on the same fixed
        // Display, not on StorageError's message.
        let e: ToolError = StorageError::Failed.into();
        assert_eq!(e.to_string(), "download failed");
    }

    #[test]
    fn conversions_from_storage_and_key_errors() {
        let e: ToolError = crate::storage::StorageError::NotFound.into();
        assert_eq!(e.error_code(), "S3_OBJECT_NOT_FOUND");
        let e: ToolError = crate::storage::StorageError::TooLarge(5).into();
        assert_eq!(e.error_code(), "S3_OBJECT_TOO_LARGE");
        let e: ToolError = crate::storage::StorageError::Failed.into();
        assert_eq!(e.error_code(), "S3_FETCH_FAILED");
        // InvalidKey's tuple constructor is private to s3keys; build one
        // through the real validation path (an empty key fails before the
        // allowlist is consulted, so any config works).
        let s3 = crate::config::S3Config {
            bucket: "b".into(),
            region: "r".into(),
            access_key_id: "ak".into(),
            secret_access_key: "sk".into(),
            endpoint_url: String::new(),
            use_path_style: true,
            allowed_key_prefixes: vec![],
        };
        let e: ToolError = crate::s3keys::validate_input_key("", &s3)
            .unwrap_err()
            .into();
        assert_eq!(e.error_code(), "S3_INVALID_KEY");
    }
}
