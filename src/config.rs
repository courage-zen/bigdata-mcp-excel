//! Environment-driven configuration, mirroring markitdown-mcp's variable
//! names so one ops runbook covers both servers.

use std::env;

/// Resolved server + S3 configuration. `s3` is None when the bucket or
/// credentials are missing, which means "S3 not configured" (distinct from
/// configured-but-failing).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub auth_keys: Vec<String>,
    pub s3: Option<S3Config>,
    pub inline_max_bytes: u64,
    pub max_file_bytes: u64,
}

/// Resolved S3 source configuration. Endpoint and path-style handling
/// follow markitdown-mcp's semantics: a custom endpoint (MinIO / R2)
/// implies path-style addressing unless explicitly overridden.
#[derive(Debug, Clone, PartialEq)]
pub struct S3Config {
    pub bucket: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub endpoint_url: String,
    /// Path-style addressing; default: true iff a custom endpoint is set
    /// (MinIO and friends cannot do virtual-host style).
    pub use_path_style: bool,
    /// Segment-aware input key allowlist; empty = no restriction.
    pub allowed_key_prefixes: Vec<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self::from_map(|k| env::var(k).ok())
    }

    /// Testable core: reads from a closure instead of the process env.
    pub fn from_map(get: impl Fn(&str) -> Option<String>) -> Self {
        let host = get("MCP_HOST")
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "0.0.0.0".into());
        let port = get("MCP_PORT")
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(8080);
        let auth_keys = get("MCP_AUTH_KEY")
            .map(|v| {
                v.split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();
        let s3 = S3Config::from_map(&get);
        // Standard names (S3 usage standard); the private EXCEL_* names
        // stay readable for one version cycle as a migration courtesy.
        let inline_max_bytes = parse_bytes(
            get("S3_INLINE_MAX_BYTES").or_else(|| get("EXCEL_INLINE_MAX_BYTES")),
            2 * 1024 * 1024,
        );
        let max_file_bytes = parse_bytes(
            get("S3_MAX_FILE_BYTES").or_else(|| get("EXCEL_MAX_FILE_BYTES")),
            100 * 1024 * 1024,
        );
        Config {
            host,
            port,
            auth_keys,
            s3,
            inline_max_bytes,
            max_file_bytes,
        }
    }
}

fn parse_bytes(raw: Option<String>, default: u64) -> u64 {
    raw.and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(default)
}

impl S3Config {
    pub fn from_map(get: &impl Fn(&str) -> Option<String>) -> Option<Self> {
        let bucket = get("S3_BUCKET")?.trim().to_string();
        let access_key_id = get("AWS_ACCESS_KEY_ID")?.trim().to_string();
        let secret_access_key = get("AWS_SECRET_ACCESS_KEY")?.trim().to_string();
        if bucket.is_empty() || access_key_id.is_empty() || secret_access_key.is_empty() {
            return None;
        }
        let endpoint_url = get("AWS_ENDPOINT_URL")
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        let region = get("S3_REGION")
            .or_else(|| get("AWS_REGION"))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "us-east-1".into());
        let raw_path_style = get("S3_USE_PATH_STYLE").map(|s| s.trim().to_lowercase());
        let use_path_style = match raw_path_style.as_deref() {
            Some("1" | "true" | "yes") => true,
            Some("0" | "false" | "no") => false,
            _ => !endpoint_url.is_empty(),
        };
        let allowed_key_prefixes = get("S3_KEY_PREFIXES")
            .map(|v| {
                v.split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();
        Some(S3Config {
            bucket,
            region,
            access_key_id,
            secret_access_key,
            endpoint_url,
            use_path_style,
            allowed_key_prefixes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn map<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + use<'a> {
        let owned: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect();
        move |k| owned.get(k).cloned()
    }

    #[test]
    fn defaults_when_env_empty() {
        let cfg = Config::from_map(|_| None);
        assert_eq!(cfg.host, "0.0.0.0");
        assert_eq!(cfg.port, 8080);
        assert!(cfg.auth_keys.is_empty());
        assert!(cfg.s3.is_none());
        assert_eq!(cfg.inline_max_bytes, 2 * 1024 * 1024);
        assert_eq!(cfg.max_file_bytes, 100 * 1024 * 1024);
    }

    #[test]
    fn s3_unconfigured_without_credentials() {
        let cfg = Config::from_map(map(&[("S3_BUCKET", "uploads")]));
        assert!(cfg.s3.is_none());
    }

    #[test]
    fn s3_prefixed_byte_limits_are_standard_names() {
        // The standard names win; old private names remain readable for one
        // version cycle as a migration courtesy.
        let cfg = Config::from_map(map(&[
            ("S3_INLINE_MAX_BYTES", "4096"),
            ("S3_MAX_FILE_BYTES", "8192"),
        ]));
        assert_eq!(cfg.inline_max_bytes, 4096);
        assert_eq!(cfg.max_file_bytes, 8192);
    }

    #[test]
    fn legacy_excel_byte_limit_names_still_readable() {
        let cfg = Config::from_map(map(&[
            ("EXCEL_INLINE_MAX_BYTES", "4096"),
            ("EXCEL_MAX_FILE_BYTES", "8192"),
        ]));
        assert_eq!(cfg.inline_max_bytes, 4096);
        assert_eq!(cfg.max_file_bytes, 8192);
    }

    #[test]
    fn standard_name_overrides_legacy_when_both_set() {
        let cfg = Config::from_map(map(&[
            ("S3_INLINE_MAX_BYTES", "4096"),
            ("EXCEL_INLINE_MAX_BYTES", "999"),
            ("S3_MAX_FILE_BYTES", "8192"),
            ("EXCEL_MAX_FILE_BYTES", "999"),
        ]));
        assert_eq!(cfg.inline_max_bytes, 4096);
        assert_eq!(cfg.max_file_bytes, 8192);
    }

    #[test]
    fn s3_minio_implies_path_style() {
        let cfg = Config::from_map(map(&[
            ("S3_BUCKET", "uploads"),
            ("AWS_ACCESS_KEY_ID", "ak"),
            ("AWS_SECRET_ACCESS_KEY", "sk"),
            ("AWS_ENDPOINT_URL", "http://minio:9000"),
        ]));
        let s3 = cfg.s3.unwrap();
        assert!(s3.use_path_style);
        assert_eq!(s3.region, "us-east-1");
    }

    #[test]
    fn s3_aws_defaults_to_virtual_host() {
        let cfg = Config::from_map(map(&[
            ("S3_BUCKET", "uploads"),
            ("AWS_ACCESS_KEY_ID", "ak"),
            ("AWS_SECRET_ACCESS_KEY", "sk"),
        ]));
        assert!(!cfg.s3.unwrap().use_path_style);
    }

    #[test]
    fn path_style_env_overrides_endpoint_default() {
        let cfg = Config::from_map(map(&[
            ("S3_BUCKET", "uploads"),
            ("AWS_ACCESS_KEY_ID", "ak"),
            ("AWS_SECRET_ACCESS_KEY", "sk"),
            ("AWS_ENDPOINT_URL", "http://minio:9000"),
            ("S3_USE_PATH_STYLE", "false"),
        ]));
        assert!(!cfg.s3.unwrap().use_path_style);
    }

    #[test]
    fn auth_keys_split_on_comma() {
        let cfg = Config::from_map(map(&[("MCP_AUTH_KEY", "k1, k2,,")]));
        assert_eq!(cfg.auth_keys, vec!["k1".to_string(), "k2".to_string()]);
    }
}
