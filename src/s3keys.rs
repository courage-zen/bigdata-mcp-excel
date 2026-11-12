//! S3 key validation: the security boundary every tool input and output
//! destination passes first. Input-key rules port markitdown-mcp's
//! `validate_s3_key` semantics; output keys mirror the same rule set plus
//! the `.xlsx` suffix requirement.

use crate::config::S3Config;

/// Extensions never worth fetching: cheap block on smuggling executables.
const DANGEROUS_EXTENSIONS: [&str; 16] = [
    ".exe", ".msi", ".bat", ".cmd", ".sh", ".ps1", ".scr", ".vbs", ".jar", ".app", ".dmg", ".pkg",
    ".deb", ".rpm", ".so", ".dll",
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

/// Percent-decode a URL path string. Lenient like the percent-encoding
/// crate: a '%' not followed by two hex digits passes through literally.
/// Invalid UTF-8 after decoding is rejected — a URL whose decoded path is
/// not UTF-8 cannot address an object key (keys are UTF-8 strings).
fn percent_decode_lenient(input: &str) -> Result<String, InvalidKey> {
    use percent_encoding::percent_decode_str;
    percent_decode_str(input)
        .decode_utf8()
        .map(|s| s.into_owned())
        .map_err(|_| InvalidKey("url path is not valid UTF-8".into()))
}

/// Resolve a tool's input key argument: a plain S3 key passes through; a
/// full object-store URL (as found in platform attachment metadata) is
/// converted to its key first. Either way the result is then validated by
/// `validate_input_key`, so the security rules are identical for both
/// forms.
pub fn resolve_input(raw: &str, s3: &S3Config) -> Result<String, InvalidKey> {
    let raw = raw.trim();
    let Some(rest) = raw
        .strip_prefix("http://")
        .or_else(|| raw.strip_prefix("https://"))
    else {
        // Not a URL: plain key, validated as-is.
        return validate_input_key(raw, s3);
    };
    // Drop the query string (presign signatures) before splitting — an
    // encoded '?' (%3F) in a key must not be mistaken for the separator.
    let rest = rest.split('?').next().unwrap_or(rest);
    // Percent-decode the path so emitted/platform URLs with encoded
    // segments resolve to the raw key. Lenient: malformed escapes pass
    // through literally; validation below runs on the decoded key, so
    // traversal smuggled through escapes is still caught. Invalid UTF-8
    // after decode rejects the input.
    let rest = percent_decode_lenient(rest)?;
    let (host, path) = match rest.split_once('/') {
        Some(split) => split,
        None => {
            return Err(InvalidKey(
                "URL contains no object key; pass 's3_key' as a plain key or a full object-store URL of an object in the configured bucket".into(),
            ));
        }
    };
    // Virtual-host style: host is "<bucket>.<endpoint-domain>".
    if let Some(domain) = host.strip_prefix(&format!("{}.", s3.bucket))
        && !domain.is_empty()
    {
        return validate_input_key(path, s3);
    }
    // Bucket-only or trailing-slash URL: segment matches, but no key.
    if path == s3.bucket || path == format!("{}/", s3.bucket) {
        return Err(InvalidKey(
            "URL contains no object key; pass 's3_key' as a plain key or a full object-store URL of an object in the configured bucket".into(),
        ));
    }
    // Path style: first path segment is the bucket.
    let key = match path.strip_prefix(&format!("{}/", s3.bucket)) {
        Some(key) => key,
        None => {
            return Err(InvalidKey(
                "URL host or first path segment does not match the configured bucket; expected an object-store URL of an object in the configured bucket".into(),
            ));
        }
    };
    validate_input_key(key, s3)
}

/// Normalize an output key: the full input rule set — format, traversal,
/// dangerous extensions, segment-aware allowlist — plus the .xlsx suffix
/// rule. Validation is symmetric: writes can never reach where reads
/// cannot.
pub fn normalize_output_key(key: &str, s3: &S3Config) -> Result<String, InvalidKey> {
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
        // The full S3 usage standard blocklist, both sides of the IO.
        for ext in [
            ".exe", ".msi", ".bat", ".cmd", ".sh", ".ps1", ".scr", ".vbs", ".jar", ".app", ".dmg",
            ".pkg", ".deb", ".rpm", ".so", ".dll",
        ] {
            assert!(
                validate_input_key(&format!("payload{ext}"), &s3(&[])).is_err(),
                "input side must reject {ext}"
            );
            assert!(
                normalize_output_key(&format!("payload{ext}.xlsx").replace(".xlsx", ""), &s3(&[]))
                    .is_err(),
                "output side must reject {ext}"
            );
        }
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
        assert!(normalize_output_key("out/a.xlsx", &s3(&[])).is_ok());
        assert!(normalize_output_key("out/a.csv", &s3(&[])).is_err());
        assert!(normalize_output_key("out/a.XLSX", &s3(&[])).is_ok());
    }

    #[test]
    fn output_key_rejects_traversal() {
        assert!(normalize_output_key("out/../etc/passwd.xlsx", &s3(&[])).is_err());
        assert!(normalize_output_key("/abs.xlsx", &s3(&[])).is_err());
    }

    #[test]
    fn output_key_allowlist_symmetric_with_input() {
        // Writes obey the same segment-aware allowlist as reads: keys
        // outside every configured prefix are refused.
        let cfg = s3(&["workspaces/ws-1"]);
        assert!(normalize_output_key("workspaces/ws-1/rep.xlsx", &cfg).is_ok());
        assert!(normalize_output_key("workspaces/ws-1", &cfg).is_err()); // not .xlsx
        assert!(normalize_output_key("workspaces/ws-11/rep.xlsx", &cfg).is_err());
        assert!(normalize_output_key("excel-mcp-out/a.xlsx", &cfg).is_err());
        assert!(normalize_output_key("other/a.xlsx", &cfg).is_err());
    }

    #[test]
    fn output_key_allowlist_prefix_with_trailing_slash() {
        let cfg = s3(&["workspaces/"]);
        assert!(normalize_output_key("workspaces/a.xlsx", &cfg).is_ok());
        assert!(normalize_output_key("other/a.xlsx", &cfg).is_err());
    }

    #[test]
    fn output_key_unrestricted_without_allowlist() {
        let cfg = s3(&[]);
        assert!(normalize_output_key("any/where/a.xlsx", &cfg).is_ok());
    }

    // --- resolve_input: URL forms ---

    fn url_s3() -> S3Config {
        S3Config {
            bucket: "uploads".into(),
            region: "r".into(),
            access_key_id: "ak".into(),
            secret_access_key: "sk".into(),
            endpoint_url: "http://minio:9000".into(),
            use_path_style: true,
            allowed_key_prefixes: vec!["workspaces/".into()],
        }
    }

    #[test]
    fn resolve_plain_key_passthrough() {
        let cfg = s3(&[]);
        assert_eq!(
            resolve_input("workspaces/a/b.xlsx", &cfg).unwrap(),
            "workspaces/a/b.xlsx"
        );
    }

    #[test]
    fn resolve_path_style_url() {
        // MinIO in-cluster form: endpoint + bucket + key in the path.
        let cfg = url_s3();
        assert_eq!(
            resolve_input("http://minio:9000/uploads/workspaces/a/b.xlsx", &cfg).unwrap(),
            "workspaces/a/b.xlsx"
        );
    }

    #[test]
    fn resolve_path_style_url_with_query() {
        // Presigned URLs carry the signature in the query string.
        let cfg = url_s3();
        assert_eq!(
            resolve_input(
                "http://minio:9000/uploads/workspaces/a/b.xlsx?X-Amz-Signature=abc&X-Amz-Expires=600",
                &cfg
            )
            .unwrap(),
            "workspaces/a/b.xlsx"
        );
    }

    #[test]
    fn resolve_virtual_host_style_url() {
        // bucket.endpoint host: path starts directly at the key.
        let cfg = url_s3();
        assert_eq!(
            resolve_input("https://uploads.s3.amazonaws.com/workspaces/a/b.xlsx", &cfg).unwrap(),
            "workspaces/a/b.xlsx"
        );
    }

    #[test]
    fn resolve_url_still_enforces_key_rules() {
        // Traversal smuggled in the URL path is caught by validate_input_key.
        let cfg = url_s3();
        assert!(resolve_input("http://minio:9000/uploads/../etc/passwd", &cfg).is_err());
    }

    #[test]
    fn resolve_virtual_host_url_still_enforces_key_rules() {
        // A traversal smuggled in the virtual-host URL path must be caught
        // by validate_input_key, same as the path-style branch.
        let cfg = url_s3();
        assert!(resolve_input("https://uploads.s3.amazonaws.com/../etc/passwd", &cfg).is_err());
    }

    #[test]
    fn resolve_bucket_only_url_has_no_key() {
        let cfg = url_s3();
        for url in ["http://minio:9000/uploads", "http://minio:9000/uploads/"] {
            let err = resolve_input(url, &cfg).unwrap_err().to_string();
            assert!(err.contains("no object key"), "for {url} got: {err}");
            assert!(!err.contains("uploads"), "must not name the bucket: {err}");
        }
    }

    #[test]
    fn resolve_bare_host_url_has_no_key() {
        let cfg = url_s3();
        let err = resolve_input("http://minio:9000", &cfg)
            .unwrap_err()
            .to_string();
        assert!(err.contains("no object key"), "message: {err}");
        assert!(
            !err.contains("minio"),
            "must not echo the rejected URL: {err}"
        );
    }

    #[test]
    fn resolve_url_first_segment_not_bucket() {
        // A URL pointing elsewhere is rejected without naming the bucket or
        // echoing the rejected key (error-message sanitization).
        let cfg = url_s3();
        let err = resolve_input("http://other-host:9000/otherbucket/workspaces/a.xlsx", &cfg)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("does not match the configured bucket"),
            "message: {err}"
        );
        assert!(!err.contains("uploads"), "must not name the bucket: {err}");
        assert!(
            !err.contains("other-host"),
            "must not echo the rejected URL: {err}"
        );
    }

    // --- resolve_input: percent-encoded URL paths ---

    #[test]
    fn resolve_encoded_path_style_url() {
        // A URL with percent-encoded segments (as our own object_url emits,
        // and as browsers/SDKs normalize) decodes back to the raw key.
        let cfg = url_s3();
        assert_eq!(
            resolve_input(
                "http://minio:9000/uploads/workspaces/ws%201/%E6%8A%A5%E8%A1%A8.xlsx",
                &cfg
            )
            .unwrap(),
            "workspaces/ws 1/报表.xlsx"
        );
    }

    #[test]
    fn resolve_encoded_traversal_is_rejected() {
        // %2e%2e decodes to '..' — the segment rule still catches it.
        let cfg = url_s3();
        assert!(
            resolve_input("http://minio:9000/uploads/workspaces/a/%2e%2e/b.xlsx", &cfg).is_err()
        );
    }

    #[test]
    fn resolve_encoded_query_separator_is_not_query() {
        // %3F decodes to '?' INSIDE the key; query stripping happens before
        // decoding, so this stays a key character, not a query separator.
        let cfg = url_s3();
        assert_eq!(
            resolve_input("http://minio:9000/uploads/workspaces/what%3F.xlsx", &cfg).unwrap(),
            "workspaces/what?.xlsx"
        );
    }

    #[test]
    fn resolve_invalid_utf8_after_decode_is_rejected() {
        // %FF is invalid UTF-8 as a standalone byte.
        let cfg = url_s3();
        assert!(resolve_input("http://minio:9000/uploads/workspaces/%FF.xlsx", &cfg).is_err());
    }

    #[test]
    fn resolve_plain_key_with_percent_passes_through() {
        // A bare key is not a URL: no decoding, '%' stays literal.
        let cfg = s3(&[]);
        assert_eq!(
            resolve_input("100% done/a.xlsx", &cfg).unwrap(),
            "100% done/a.xlsx"
        );
    }

    #[test]
    fn resolve_unencoded_cjk_path_still_works() {
        // IRIs (raw UTF-8 in the path, no percent-escapes) are still
        // accepted: decode is a no-op on un-escaped bytes.
        let cfg = url_s3();
        assert_eq!(
            resolve_input("http://minio:9000/uploads/workspaces/报表.xlsx", &cfg).unwrap(),
            "workspaces/报表.xlsx"
        );
    }

    #[test]
    fn resolve_malformed_escape_passes_through_literally() {
        // percent_decode's documented lenient semantics: a lone '%' not
        // followed by hex passes through as a literal '%'.
        let cfg = url_s3();
        assert_eq!(
            resolve_input("http://minio:9000/uploads/workspaces/100%25.xlsx", &cfg).unwrap(),
            "workspaces/100%.xlsx"
        );
        // URL-side lone '%' (no valid escape) also passes through literally:
        assert_eq!(
            resolve_input("http://minio:9000/uploads/workspaces/100%.xlsx", &cfg).unwrap(),
            "workspaces/100%.xlsx"
        );
    }

    #[test]
    fn object_url_round_trips_through_resolve_input() {
        // The invariant this feature exists for: every configuration shape,
        // every key — resolve_input(object_url(cfg, key)) == key.
        for (endpoint, path_style) in [
            ("http://minio:9000", true),
            ("https://s3.example.com", false),
            ("", true),
            ("", false),
        ] {
            let cfg = S3Config {
                bucket: "uploads".into(),
                region: "us-east-1".into(),
                access_key_id: "ak".into(),
                secret_access_key: "sk".into(),
                endpoint_url: endpoint.into(),
                use_path_style: path_style,
                allowed_key_prefixes: vec![],
            };
            for key in [
                "workspaces/ws-1/a.xlsx",
                "workspaces/ws 1/报表.xlsx",
                "100% done/a.xlsx",
                "deep/nested/key ~v2.xlsx",
            ] {
                let url = crate::objecturl::object_url(&cfg, key);
                let back = resolve_input(&url, &cfg)
                    .unwrap_or_else(|e| panic!("round-trip failed for {url}: {e}"));
                assert_eq!(back, key, "round-trip mismatch for {url}");
            }
        }
    }
}
