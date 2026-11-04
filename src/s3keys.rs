//! S3 key validation: the security boundary every tool input passes first.
//! Port of markitdown-mcp's `validate_s3_key` semantics.

use crate::config::S3Config;

/// Extensions never worth fetching: cheap block on smuggling executables.
const DANGEROUS_EXTENSIONS: [&str; 12] = [
    ".exe", ".bat", ".cmd", ".sh", ".ps1", ".scr", ".vbs", ".jar", ".app", ".dmg", ".pkg", ".deb",
];

#[derive(Debug, thiserror::Error)]
#[error("invalid s3 key: {0}")]
pub struct InvalidKey(String);

/// Validate an input key against the configured allowlist.
///
/// Rules (markitdown-mcp parity):
/// - non-empty, no leading `/`, no `//`, no `.`/`..` segments, no backslash
/// - extension (if any) not in DANGEROUS_EXTENSIONS
/// - when `S3_KEY_PREFIXES` is set, key must start with one prefix at a
///   segment boundary (`workspaces/ws-1` matches `workspaces/ws-1/a.xlsx`
///   and `workspaces/ws-1`, not `workspaces/ws-11/a.xlsx`)
pub fn validate_input_key(key: &str, s3: &S3Config) -> Result<String, InvalidKey> {
    let key = key.trim();
    if key.is_empty() {
        return Err(InvalidKey("key is empty".into()));
    }
    if key.starts_with('/') {
        return Err(InvalidKey("key must not start with '/'".into()));
    }
    if key.contains('\\') || key.contains("//") {
        return Err(InvalidKey("key must be a plain '/'-separated path".into()));
    }
    if key
        .split('/')
        .any(|seg| seg.is_empty() || seg == "." || seg == "..")
    {
        return Err(InvalidKey(
            "key must not contain '.', '..' or empty segments".into(),
        ));
    }
    let lower = key.to_lowercase();
    if DANGEROUS_EXTENSIONS.iter().any(|ext| lower.ends_with(ext)) {
        return Err(InvalidKey("key extension is refused".into()));
    }
    if !s3.allowed_key_prefixes.is_empty()
        && !s3.allowed_key_prefixes.iter().any(|p| {
            key == p.as_str()
                || key.starts_with(p.as_str())
                    && (p.ends_with('/') || key.as_bytes().get(p.len()) == Some(&b'/'))
        })
    {
        return Err(InvalidKey("key is outside the configured allowlist".into()));
    }
    Ok(key.to_string())
}

/// Normalize an output key: validation minus the input allowlist (outputs
/// go to `S3_OUTPUT_PREFIX`, not the input roots) plus the .xlsx suffix
/// rule.
pub fn normalize_output_key(key: &str) -> Result<String, InvalidKey> {
    let key = key.trim();
    if key.is_empty() {
        return Err(InvalidKey("key is empty".into()));
    }
    if key.starts_with('/')
        || key.contains('\\')
        || key.contains("//")
        || key
            .split('/')
            .any(|seg| seg.is_empty() || seg == "." || seg == "..")
    {
        return Err(InvalidKey("key must be a plain '/'-separated path".into()));
    }
    let lower = key.to_lowercase();
    if DANGEROUS_EXTENSIONS.iter().any(|ext| lower.ends_with(ext)) {
        return Err(InvalidKey("key extension is refused".into()));
    }
    if !lower.ends_with(".xlsx") {
        return Err(InvalidKey("output key must end with .xlsx".into()));
    }
    Ok(key.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s3(prefixes: &[&str]) -> S3Config {
        S3Config {
            bucket: "b".into(),
            region: "r".into(),
            access_key_id: "ak".into(),
            secret_access_key: "sk".into(),
            endpoint_url: "http://minio:9000".into(),
            use_path_style: true,
            allowed_key_prefixes: prefixes.iter().map(|p| p.to_string()).collect(),
        }
    }

    #[test]
    fn rejects_empty_and_absolute() {
        assert!(validate_input_key("", &s3(&[])).is_err());
        assert!(validate_input_key("  ", &s3(&[])).is_err());
        assert!(validate_input_key("/abs/path.xlsx", &s3(&[])).is_err());
    }

    #[test]
    fn rejects_traversal_and_double_slash() {
        assert!(validate_input_key("a/../b.xlsx", &s3(&[])).is_err());
        assert!(validate_input_key("a//b.xlsx", &s3(&[])).is_err());
        assert!(validate_input_key("a/./b.xlsx", &s3(&[])).is_err());
        assert!(validate_input_key(r"a\b.xlsx", &s3(&[])).is_err());
    }

    #[test]
    fn rejects_dangerous_extensions() {
        assert!(validate_input_key("payload.exe", &s3(&[])).is_err());
        assert!(validate_input_key("payload.CMD", &s3(&[])).is_err());
    }

    #[test]
    fn allowlist_matches_segment_aware() {
        let cfg = s3(&["workspaces/ws-1"]);
        assert!(validate_input_key("workspaces/ws-1", &cfg).is_ok());
        assert!(validate_input_key("workspaces/ws-1/a.xlsx", &cfg).is_ok());
        assert!(validate_input_key("workspaces/ws-11/a.xlsx", &cfg).is_err());
        assert!(validate_input_key("other/a.xlsx", &cfg).is_err());
    }

    #[test]
    fn allowlist_prefix_with_trailing_slash() {
        let cfg = s3(&["workspaces/"]);
        assert!(validate_input_key("workspaces/a.xlsx", &cfg).is_ok());
        assert!(validate_input_key("other/a.xlsx", &cfg).is_err());
    }

    #[test]
    fn no_allowlist_accepts_any_valid_key() {
        let cfg = s3(&[]);
        assert!(validate_input_key("any/where/a.xlsx", &cfg).is_ok());
    }

    #[test]
    fn output_key_requires_xlsx_suffix() {
        assert!(normalize_output_key("out/a.xlsx").is_ok());
        assert!(normalize_output_key("out/a.csv").is_err());
        assert!(normalize_output_key("out/a.XLSX").is_ok());
    }

    #[test]
    fn output_key_rejects_traversal() {
        assert!(normalize_output_key("out/../etc/passwd.xlsx").is_err());
        assert!(normalize_output_key("/abs.xlsx").is_err());
    }
}
