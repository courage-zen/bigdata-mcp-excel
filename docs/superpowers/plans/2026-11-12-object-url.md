# Write Tools Return object_url Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Both write tools (`create_xlsx`, `sheet_apply`) return the output object's canonical object-store URL as `object_url`, built from the resolved S3 config; `resolve_input` learns to percent-decode URL paths so emitted (and platform) URLs round-trip back to keys.

**Architecture:** One new pure module `src/objecturl.rs` (URL construction from `S3Config` + key), a decoding pass inside `s3keys::resolve_input` before host/bucket/key validation, and two one-line payload additions in `tools.rs`. No I/O, no client changes — `object_url` is string assembly over already-validated keys.

**Tech Stack:** Rust 2024, percent-encoding 2.3.2 (already in Cargo.lock as object_store's codec; becomes a direct dependency), existing testcontainers integration harness.

**Spec:** `docs/superpowers/specs/2026-11-12-object-url-design.md`

---

### Task 1: objecturl module — URL construction

**Files:**
- Create: `src/objecturl.rs`
- Modify: `src/main.rs` (module registration), `Cargo.toml` (dependency)

- [ ] **Step 1: Write the failing tests**

Create `src/objecturl.rs` with tests only (module + tests compile against an unimplemented `object_url`, which fails to build — TDD red):

```rust
//! Canonical object-store URL construction for write-tool outputs: pure
//! string assembly over validated keys, mirroring how the object_store
//! client addresses the same object.

use crate::config::S3Config;

/// Build the canonical reference URL of `key` under `s3`: no credentials,
/// no presigning — readers need their own access. The shape follows the
/// resolved addressing style (endpoint/path-style/region), so the URL is
/// the same address the object_store client would use.
pub fn object_url(s3: &S3Config, key: &str) -> String {
    unimplemented!()
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
        let url = object_url(&s3("https://s3.example.com", false), "workspaces/ws-1/a.xlsx");
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
        let url = object_url(&s3("http://minio:9000", true), "workspaces/ws-1/a-b_c~d.xlsx");
        assert_eq!(url, "http://minio:9000/uploads/workspaces/ws-1/a-b_c~d.xlsx");
    }
}
```

- [ ] **Step 2: Register the module and dependency, verify red**

In `src/main.rs`, add `mod objecturl;` next to the existing module declarations (match their order: after `mod config;`, before `mod s3keys;` — alphabetical: auth, config, engine, objecturl, s3keys, server, storage, toolerr, tools, util).

In `Cargo.toml`, add under `[dependencies]` (keep alphabetical order, after `object_store`):

```toml
percent-encoding = "2"
```

Run: `cargo test objecturl 2>&1 | tail -5`
Expected: compile succeeds (stub body `unimplemented!()`), the 7 tests FAIL with panic `not implemented` — the red state.

- [ ] **Step 3: Implement `object_url`**

Replace the `unimplemented!()` body in `src/objecturl.rs`:

```rust
/// Build the canonical reference URL of `key` under `s3`: no credentials,
/// no presigning — readers need their own access. The shape follows the
/// resolved addressing style (endpoint/path-style/region), so the URL is
/// the same address the object_store client would use.
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
    let encode_segment = |seg: &str| -> String {
        percent_encoding::utf8_percent_encode(seg, SEGMENT).to_string()
    };
    let encoded_key = key
        .split('/')
        .map(encode_segment)
        .collect::<Vec<_>>()
        .join("/");

    let trim_endpoint = |ep: &str| -> String {
        ep.trim().trim_end_matches('/').to_string()
    };

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
            let scheme = if endpoint.starts_with("https://") { "https" } else { "http" };
            format!("{scheme}://{}.{}/{encoded_key}", s3.bucket, host)
        }
    } else if s3.use_path_style {
        format!("https://s3.{}.amazonaws.com/{}/{encoded_key}", s3.region, s3.bucket)
    } else {
        format!("https://{}.s3.{}.amazonaws.com/{encoded_key}", s3.bucket, s3.region)
    }
}
```

- [ ] **Step 4: Verify green**

Run: `cargo test objecturl 2>&1 | tail -5`
Expected: all 7 tests PASS.

Run: `cargo fmt --check && cargo clippy -- -D warnings 2>&1 | tail -3`
Expected: clean (the `percent_encoding` paths in the SEGMENT const must resolve; fix any clippy nits).

- [ ] **Step 5: Commit**

```bash
git add src/objecturl.rs src/main.rs Cargo.toml Cargo.lock
git commit -m "feat: objecturl module builds canonical object-store URLs"
```

---

### Task 2: resolve_input decodes percent-escapes

**Files:**
- Modify: `src/s3keys.rs` (resolve_input)

- [ ] **Step 1: Write the failing tests**

Append to `src/s3keys.rs` tests module (after `resolve_url_first_segment_not_bucket`):

```rust
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
        assert!(resolve_input("http://minio:9000/uploads/a/%2e%2e/b.xlsx", &cfg).is_err());
    }

    #[test]
    fn resolve_encoded_query_separator_is_not_query() {
        // %3F decodes to '?' INSIDE the key; query stripping happens before
        // decoding, so this stays a key character, not a query separator.
        let cfg = url_s3();
        assert_eq!(
            resolve_input("http://minio:9000/uploads/what%3F.xlsx", &cfg).unwrap(),
            "what?.xlsx"
        );
    }

    #[test]
    fn resolve_invalid_utf8_after_decode_is_rejected() {
        // %FF is invalid UTF-8 as a standalone byte.
        let cfg = url_s3();
        assert!(resolve_input("http://minio:9000/uploads/%FF.xlsx", &cfg).is_err());
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
            resolve_input("http://minio:9000/uploads/100%25.xlsx", &cfg).unwrap(),
            "100%.xlsx"
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
```

Note: the URL-side lone-`%` case (`100%.xlsx` in a URL, no valid escape) also passes through literally per percent-encoding's lenient semantics — add it as a second assert inside the malformed test: `resolve_input("http://minio:9000/uploads/100%.xlsx", &cfg)` → `"100%.xlsx"`.

- [ ] **Step 2: Verify red**

Run: `cargo test s3keys 2>&1 | tail -8`
Expected: NEW tests FAIL (resolve_encoded_path_style_url, resolve_encoded_traversal_is_rejected passes already? — no: current code compares the literal path `workspaces/ws%201/...` which does not match any allowlist issue but `validate_input_key` on the raw path... run and see: encoded tests fail because no decoding happens today; traversal-encoded may accidentally pass validation today since `%2e%2e` is not literally `..` — that's the bug, assert err → currently FAILS. Red state confirmed by at least the encoded-path and traversal tests failing).

- [ ] **Step 3: Implement decoding in resolve_input**

In `src/s3keys.rs`, replace the body of `resolve_input` between the `http://`/`https://` strip and the host/bucket matching. Current code:

```rust
pub fn resolve_input(raw: &str, s3: &S3Config) -> Result<String, InvalidKey> {
    let raw = raw.trim();
    let Some(rest) = raw
        .strip_prefix("http://")
        .or_else(|| raw.strip_prefix("https://"))
    else {
        // Not a URL: plain key, validated as-is.
        return validate_input_key(raw, s3);
    };
    // Drop the query string (presign signatures) before splitting.
    let rest = rest.split('?').next().unwrap_or(rest);
    let (host, path) = match rest.split_once('/') {
        Some(split) => split,
        None => {
            return Err(InvalidKey(...));
        }
    };
    ...
```

Change: after the query-strip line and BEFORE `let (host, path) = ...`, decode the path portion. Since the host portion contains no percent-escapes in practice (and stripping `http://` already removed the scheme), decode the whole `rest` after query-strip, then split:

```rust
    // Drop the query string (presign signatures) before splitting — an
    // encoded '?' (%3F) in a key must not be mistaken for the separator.
    let rest = rest.split('?').next().unwrap_or(rest);
    // Percent-decode the path so emitted/platform URLs with encoded
    // segments resolve to the raw key. Lenient: malformed escapes pass
    // through literally; validation below runs on the decoded key, so
    // traversal smuggled through escapes is still caught. Invalid UTF-8
    // after decode rejects the input.
    let rest = percent_decode_lenient(rest);
```

Then the existing host/bucket matching operates on the decoded `rest` — the `path` extracted there is already decoded, flows into `validate_input_key` unchanged. Add the helper at module level (above `resolve_input`):

```rust
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
```

And in `resolve_input`, after the query-strip line:

```rust
    let rest = percent_decode_lenient(rest)?;
```

- [ ] **Step 4: Verify green + full suite**

Run: `cargo test 2>&1 | tail -5`
Expected: ALL tests pass (new s3keys tests + existing suite; the round-trip test needs `crate::objecturl` from Task 1 — already merged).

Run: `cargo fmt --check && cargo clippy -- -D warnings 2>&1 | tail -3`
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add src/s3keys.rs
git commit -m "feat: resolve_input percent-decodes URL paths"
```

---

### Task 3: write tools return object_url

**Files:**
- Modify: `src/tools.rs` (payloads, tool descriptions, instructions)

- [ ] **Step 1: Write the failing tests**

Append to `src/tools.rs` tests module. Two unit tests on the payload shape with a fake-but-valid S3 config (endpoint unreachable — upload fails, so payload-level tests can't run the full path...). Instead: integration-level assertions live in Task 4; here assert on the JSON payload structure via the render path — the simplest honest unit test is on `create_xlsx_inner`/`sheet_apply_inner`'s success payload with a REAL local S3, which we don't have. So: unit tests assert the tool DESCRIPTIONS mention object_url (cheap, meaningful for the agent-facing contract) and the payload assembly stays a pure function of (s3, key) — extract payload building? No — keep it inline; the description tests:

```rust
    #[test]
    fn tool_descriptions_mention_object_url() {
        // The agent-facing contract: write tools advertise object_url.
        let desc = ExcelMcp::tool_create_xlsx_description();
        assert!(desc.contains("object_url"));
        let desc = ExcelMcp::tool_sheet_apply_description();
        assert!(desc.contains("object_url"));
    }
```

Note: rmcp's `#[tool]` macro generates description accessors with names like `tool_create_xlsx_description`? — it does not expose them. Instead, test via the router: `ExcelMcp::new(cfg).get_info()` lists tools? — ServerConfig carries instructions, not tool descriptors. The pragmatic unit test: assert the string literal used in the description contains "object_url" — a compile-time-ish pin. Implement as:

```rust
    #[test]
    fn tool_descriptions_mention_object_url() {
        // The agent-facing contract: write tools advertise object_url.
        let cfg = test_cfg();
        let server = ExcelMcp::new(cfg);
        let router = server.tool_router();
        let tools = router.as_ref().tools();
        let by_name = |n: &str| {
            tools
                .iter()
                .find(|t| t.name == n)
                .unwrap_or_else(|| panic!("tool {n} missing"))
                .clone()
        };
        let create = by_name("create_xlsx");
        assert!(create.description.unwrap().contains("object_url"));
        let apply = by_name("sheet_apply");
        assert!(apply.description.unwrap().contains("object_url"));
    }
```

The router API surface: rmcp `#[tool_router]` generates a `tool_router()` method returning something with `.tools()`. EXACT API may differ — if `tool_router()`/`.tools()` doesn't compile, adapt to the actual rmcp 3 API (check `rmcp::handler::server::router::tool_router` docs or the macro expansion in target/). The assertion target stays: descriptions of both write tools mention `object_url`.

- [ ] **Step 2: Verify red**

Run: `cargo test tool_descriptions 2>&1 | tail -3`
Expected: FAIL (no tool description mentions object_url yet).

- [ ] **Step 3: Implement the payload + description changes**

`src/tools.rs`, in `create_xlsx_inner`'s success payload:

```rust
    let inline_limit = server.cfg.inline_max_bytes as usize;
    let obj = serde_json::json!({
        "s3_key": output_key,
        "object_url": crate::objecturl::object_url(&s3, &output_key),
        "cells_written": cells,
    });
    if bytes.len() <= inline_limit {
        obj["content_b64"] = serde_json::Value::String(crate::util::base64_encode(&bytes));
    }
    Ok(obj)
```

(`output_key` is moved into the JSON — build the URL BEFORE the json! macro moves it, or clone: build `let url = crate::objecturl::object_url(&s3, &output_key);` before, reference it in the macro. Check current code: `output_key` is used by upload_bytes first (borrowed), then moved into json!. Insert the URL line between upload and json!.)

In `sheet_apply_inner`'s success payload:

```rust
    let object_url = crate::objecturl::object_url(&s3, &output_key);
    Ok(serde_json::json!({
        "s3_key": output_key,
        "object_url": object_url,
        "cells_edited": applied,
        "ops_applied": ops_count,
    }))
```

Tool descriptions — `create_xlsx`: change `Returns the output s3_key and the file bytes base64-encoded when small.` to `Returns the output s3_key, its canonical object-store URL (object_url), and the file bytes base64-encoded when small.`

`sheet_apply`: append after `an existing object there is overwritten.` → `an existing object there is overwritten. Returns the output s3_key and its canonical object-store URL (object_url).`

Server instructions (`get_info().with_instructions(...)`): after `Every output goes to an explicit output_key...` add `Write tools return object_url, the canonical object-store URL of the output — pass it around as the object reference.`

- [ ] **Step 4: Verify green + suite**

Run: `cargo test 2>&1 | tail -5`
Expected: all pass.

Run: `cargo fmt --check && cargo clippy -- -D warnings 2>&1 | tail -3`
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add src/tools.rs
git commit -m "feat: write tools return object_url in their payloads"
```

---

### Task 4: integration tests + docs + version

**Files:**
- Modify: `tests/integration.rs`, `README.md`, `Cargo.toml` (version), `.github/workflows/release.yml` (smoke)

- [ ] **Step 1: Extend the round-trip integration test**

In `tests/integration.rs`, `create_read_apply_roundtrip` — after the create assertions, add:

```rust
    // The emitted object_url points at the created object and is accepted
    // back by sheet_read (round-trip).
    let object_url = created["object_url"].as_str().unwrap().to_string();
    assert_eq!(
        object_url,
        format!("{}/uploads/workspaces/ws-42/inventory.xlsx", stack.s3_endpoint)
    );
    let read_via_url = stack
        .tool_result_json("sheet_read", serde_json::json!({"s3_key": object_url}), 15)
        .unwrap();
    assert_eq!(read_via_url["cells"]["A2"]["value"].as_str().unwrap(), "apple");
```

And after the sheet_apply assertions:

```rust
    let apply_url = applied["object_url"].as_str().unwrap().to_string();
    assert_eq!(
        apply_url,
        format!("{}/uploads/workspaces/ws-42/edited.xlsx", stack.s3_endpoint)
    );
    let read2_via_url = stack
        .tool_result_json("sheet_read", serde_json::json!({"s3_key": apply_url}), 16)
        .unwrap();
    assert_eq!(read2_via_url["cells"]["C2"]["value"].as_str().unwrap(), "fresh");
```

Add a new CJK round-trip test (after `seeded_object_reads_back_via_url`):

```rust
#[test]
#[ignore = "needs Docker + integration feature"]
fn create_with_cjk_key_returns_readable_object_url() {
    // object_url percent-encodes non-ASCII key segments; sheet_read
    // accepts the encoded URL back and resolves the same object.
    let stack = Stack::start().expect("stack");
    let created = stack
        .tool_result_json(
            "create_xlsx",
            serde_json::json!({
                "data": [["k", "v"], ["name", "苹果"]],
                "output_key": "workspaces/ws-42/报表.xlsx"
            }),
            50,
        )
        .unwrap();
    let url = created["object_url"].as_str().unwrap();
    assert!(url.ends_with("/uploads/workspaces/ws-42/%E6%8A%A5%E8%A1%A8.xlsx"), "got: {url}");
    let read = stack
        .tool_result_json("sheet_read", serde_json::json!({"s3_key": url}), 51)
        .unwrap();
    assert_eq!(read["cells"]["B2"]["value"].as_str().unwrap(), "苹果");
}
```

- [ ] **Step 1b: Run the integration tests (needs Docker)**

Run: `cargo test --features integration --test integration -- --ignored 2>&1 | tail -8`
Expected: all green (existing + new). If Docker is unavailable, report it — do not skip silently.

- [ ] **Step 2: README updates**

`README.md`:

1. `create_xlsx` bullet (line ~10-12): after `Returns the output \`s3_key\` plus the file bytes base64-inline when small.` → `Returns the output \`s3_key\`, its canonical object-store URL (\`object_url\`), plus the file bytes base64-inline when small.`

2. Response example (line ~46-48): `Response: `{"s3_key": "workspaces/ws-42/reports/fruit.xlsx", "cells_written": 6, "content_b64": "..."}`` → add `"object_url": "http://minio:9000/uploads/workspaces/ws-42/reports/fruit.xlsx"` between s3_key and cells_written in the example JSON.

3. Chinese section URL paragraph (line ~205-207): append one sentence: `写入工具（create_xlsx / sheet_apply）的返回值带 \`object_url\` 字段——输出对象的完整对象存储 URL，可直接作为引用传递；该 URL 也能直接传回 sheet_read 读取。`

- [ ] **Step 3: Version bump**

`Cargo.toml`: `version = "0.3.0"` → `version = "0.4.0"`.

Run: `cargo build 2>&1 | tail -2` — Expected: `Finished` (lock updates).

- [ ] **Step 4: Release smoke round-trips via object_url**

`.github/workflows/release.yml`, in the smoke step: replace the KEY extraction + read step with object_url-based extraction (keep the s3_key grep as a sanity check):

```bash
          # create -> read round-trip through the returned object_url
          # (exercises URL emission AND URL acceptance in one pass)
          created="$(curl -sf -X POST "$BASE" -H "$AUTH" -H "Mcp-Session-Id: $SID" -H "$CT" -H "$ACCEPT" \
            -d '{"jsonrpc":"2.0","method":"tools/call","params":{"name":"create_xlsx","arguments":{"data":[["name","qty"],["apple",3]],"output_key":"workspaces/ws-1/smoke.xlsx"}},"id":3}')"
          echo "$created" | grep -q 's3_key' || { echo 'create_xlsx returned no s3_key'; echo "$created"; exit 1; }
          echo "$created" | grep -q 'object_url' || { echo 'create_xlsx returned no object_url'; echo "$created"; exit 1; }

          # sheet_read on the created object, addressing it BY URL
          KEY="$(echo "$created" | sed -n 's/^data://p' \
            | python3 -c 'import json,sys; print(json.load(sys.stdin)["result"]["content"][0]["text"])' \
            | python3 -c 'import json,sys; print(json.load(sys.stdin)["s3_key"])')"
          URL="$(echo "$created" | sed -n 's/^data://p' \
            | python3 -c 'import json,sys; print(json.load(sys.stdin)["result"]["content"][0]["text"])' \
            | python3 -c 'import json,sys; print(json.load(sys.stdin)["object_url"])')"
          readback="$(curl -sf -X POST "$BASE" -H "$AUTH" -H "Mcp-Session-Id: $SID" -H "$CT" -H "$ACCEPT" \
            -d "{\"jsonrpc\":\"2.0\",\"method\":\"tools/call\",\"params\":{\"name\":\"sheet_read\",\"arguments\":{\"s3_key\":\"${URL}\"}},\"id\":4}")"
          echo "$readback" | grep -q 'apple' || { echo 'sheet_read via object_url round-trip failed'; echo "$readback"; exit 1; }
```

The sheet_apply step below it still uses `${KEY}` (unchanged — key form works and is already covered).

- [ ] **Step 5: Commit**

```bash
git add tests/integration.rs README.md Cargo.toml Cargo.lock .github/workflows/release.yml
git commit -m "feat: integration tests, docs, smoke URL round-trip; 0.4.0"
```

---

### Task 5: release v0.4.0

- [ ] **Step 1: Final full check**

Run: `cargo fmt --check && cargo clippy -- -D warnings && cargo test 2>&1 | tail -3`
Expected: all clean/green.

- [ ] **Step 2: Tag and push**

```bash
git tag v0.4.0 && git push origin main v0.4.0
```

- [ ] **Step 3: Watch the release workflow**

Run: `gh run watch --repo courage-zen/bigdata-mcp-excel $(gh run list --repo courage-zen/bigdata-mcp-excel --limit 1 --json databaseId --jq '.[0].databaseId')`
Expected: green; release v0.4.0 with 5 assets. (Verify the picked run is the Release workflow, not Build.)

- [ ] **Step 4: Verify the release**

Run: `gh release view v0.4.0 --repo courage-zen/bigdata-mcp-excel --json tagName,assets --jq '.tagName, (.assets | map(.name) | join("\n"))'`
Expected: v0.4.0 + the 5 usual assets.
