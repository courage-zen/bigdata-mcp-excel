//! Canonical object-store URL construction for write-tool outputs: pure
//! string assembly over validated keys, mirroring how the object_store
//! client addresses the same object.

use crate::config::S3Config;

/// Build the canonical reference URL of `key` under `s3`: no credentials,
/// no presigning — readers need their own access. The shape follows the
/// resolved addressing style (endpoint/path-style/region), so the URL is
/// the same address the object_store client would use.
// Task 3 (write-tool payloads) is the first caller; until then the binary
// has no reference to this function.
#[expect(dead_code)]
pub fn object_url(s3: &S3Config, key: &str) -> String {
    // Per-segment percent-encoding: '/' stays a literal separator, every
    // other non-unreserved byte of the UTF-8 segment encodes. The set is
    // "encode all non-alphanumerics, then un-reserve -._~" = exactly the
    // non-unreserved complement of RFC 3986.
    const SEGMENT: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'.')
        .remove(b'_')
        .remove(b'~');
    let encode_segment =
        |seg: &str| -> String { percent_encoding::utf8_percent_encode(seg, SEGMENT).to_string() };
    let encoded_key = key
        .split('/')
        .map(encode_segment)
        .collect::<Vec<_>>()
        .join("/");

    let trim_endpoint = |ep: &str| -> String { ep.trim().trim_end_matches('/').to_string() };

    if !s3.endpoint_url.is_empty() {
        let mut endpoint = trim_endpoint(&s3.endpoint_url);
        if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
            endpoint = format!("http://{endpoint}");
        }
        if s3.use_path_style {
            format!("{endpoint}/{}/{encoded_key}", s3.bucket)
        } else {
            // Virtual-host on a custom endpoint: bucket.<host[:port]>.
            let host = endpoint
                .strip_prefix("http://")
                .or_else(|| endpoint.strip_prefix("https://"))
                .unwrap_or(&endpoint);
            let scheme = if endpoint.starts_with("https://") {
                "https"
            } else {
                "http"
            };
            format!("{scheme}://{}.{}/{encoded_key}", s3.bucket, host)
        }
    } else if s3.use_path_style {
        format!(
            "https://s3.{}.amazonaws.com/{}/{encoded_key}",
            s3.region, s3.bucket
        )
    } else {
        format!(
            "https://{}.s3.{}.amazonaws.com/{encoded_key}",
            s3.bucket, s3.region
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s3(endpoint: &str, path_style: bool) -> S3Config {
        S3Config {
            bucket: "uploads".into(),
            region: "us-east-1".into(),
            access_key_id: "ak".into(),
            secret_access_key: "sk".into(),
            endpoint_url: endpoint.into(),
            use_path_style: path_style,
            allowed_key_prefixes: vec![],
        }
    }

    #[test]
    fn minio_endpoint_path_style() {
        let url = object_url(&s3("http://minio:9000", true), "workspaces/ws-1/a.xlsx");
        assert_eq!(url, "http://minio:9000/uploads/workspaces/ws-1/a.xlsx");
    }

    #[test]
    fn custom_endpoint_virtual_host() {
        let url = object_url(
            &s3("https://s3.example.com", false),
            "workspaces/ws-1/a.xlsx",
        );
        assert_eq!(url, "https://uploads.s3.example.com/workspaces/ws-1/a.xlsx");
    }

    #[test]
    fn aws_virtual_host() {
        let url = object_url(&s3("", false), "workspaces/ws-1/a.xlsx");
        assert_eq!(
            url,
            "https://uploads.s3.us-east-1.amazonaws.com/workspaces/ws-1/a.xlsx"
        );
    }

    #[test]
    fn aws_path_style() {
        let url = object_url(&s3("", true), "workspaces/ws-1/a.xlsx");
        assert_eq!(
            url,
            "https://s3.us-east-1.amazonaws.com/uploads/workspaces/ws-1/a.xlsx"
        );
    }

    #[test]
    fn endpoint_trailing_slash_and_bare_host() {
        // Trailing slashes on the endpoint are trimmed; a scheme-less
        // endpoint is treated as http://.
        let url = object_url(&s3("http://minio:9000///", true), "a.xlsx");
        assert_eq!(url, "http://minio:9000/uploads/a.xlsx");
        let url = object_url(&s3("minio:9000", true), "a.xlsx");
        assert_eq!(url, "http://minio:9000/uploads/a.xlsx");
    }

    #[test]
    fn key_segments_are_percent_encoded() {
        // Each segment encodes non-unreserved characters; '/' stays literal.
        let url = object_url(&s3("http://minio:9000", true), "workspaces/ws 1/报表.xlsx");
        assert_eq!(
            url,
            "http://minio:9000/uploads/workspaces/ws%201/%E6%8A%A5%E8%A1%A8.xlsx"
        );
        // '%' itself must encode (a raw '%25' in a key encodes as '%2525').
        let url = object_url(&s3("http://minio:9000", true), "100% done/a.xlsx");
        assert_eq!(url, "http://minio:9000/uploads/100%25%20done/a.xlsx");
    }

    #[test]
    fn plain_key_is_unchanged() {
        // Unreserved characters pass through untouched.
        let url = object_url(
            &s3("http://minio:9000", true),
            "workspaces/ws-1/a-b_c~d.xlsx",
        );
        assert_eq!(
            url,
            "http://minio:9000/uploads/workspaces/ws-1/a-b_c~d.xlsx"
        );
    }
}
