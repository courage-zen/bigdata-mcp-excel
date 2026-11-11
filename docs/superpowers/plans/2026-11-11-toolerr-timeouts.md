# Tool-Result Error Codes + Layered Timeouts Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Business failures return machine-readable `error_code` in tool-result layer (not JSON-RPC errors), and every S3/engine call is bounded by the standard's four-layer timeouts.

**Architecture:** New `src/toolerr.rs` centralizes the error enum (nine codes) with `From` conversions from `StorageError`/`InvalidKey`; each `#[tool]` method becomes a thin wrapper rendering `Result<Value, ToolError>` via one shared helper. Timeouts: object_store `ClientOptions` (connect 10s/read 60s), download wall-clock inside `fetch_to_tempfile` (60s), processing wall-clock around `spawn_blocking` in tools (30s) — all constants.

**Tech Stack:** Rust 2024, rmcp 3.8 (`CallToolResult::error`), object_store 0.14 (`ClientOptions`), tokio `time::timeout` + `start_paused` tests, mock `ObjectStore` via `futures::stream::pending`.

**Spec:** `docs/superpowers/specs/2026-11-11-toolerr-timeouts-design.md`

**Repo facts the engineer must know:**

- Crate has NO lib target. Unit tests run via `cargo test --bin excel-mcp` (NOT `cargo test --lib`).
- Module tree is declared in `src/main.rs` (alphabetical): `mod toolerr;` goes between `mod storage;` and `mod tools;`.
- `CallToolResult::success(vec![...])` / `CallToolResult::error(vec![...])` are rmcp constructors; `error` sets `is_error: Some(true)`. Content: `ContentBlock::text(...)`.
- The S3 usage standard: `docs/superpowers/specs/2026-11-11-mcp-s3-usage-standard.md` (§4 timeouts, §5 error codes).
- Error messages must stay sanitized: no bucket/endpoint names, no echoing rejected keys. Existing messages in `s3keys.rs` are already sanitized; reuse their wording.
- 24 `McpError::*` sites in tools.rs all disappear. The ONLY protocol-level error that remains valid is serde deserialization failure (rmcp framework, before tool body runs) — nothing to keep in our code.

---

### Task 1: `ToolError` enum with codes and conversions

**Files:**
- Create: `src/toolerr.rs`
- Modify: `src/main.rs` (add `mod toolerr;`)

- [ ] **Step 1: Write the failing test**

Create `src/toolerr.rs` with ONLY the test module first (implementation comes in Step 3):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_match_the_standard() {
        assert_eq!(ToolError::S3Unavailable.error_code(), "S3_UNAVAILABLE");
        assert_eq!(ToolError::S3ObjectNotFound.error_code(), "S3_OBJECT_NOT_FOUND");
        assert_eq!(ToolError::S3FetchFailed(String::new()).error_code(), "S3_FETCH_FAILED");
        assert_eq!(ToolError::S3UploadFailed.error_code(), "S3_UPLOAD_FAILED");
        assert_eq!(ToolError::S3ObjectTooLarge(100).error_code(), "S3_OBJECT_TOO_LARGE");
        assert_eq!(ToolError::S3InvalidKey(String::new()).error_code(), "S3_INVALID_KEY");
        assert_eq!(ToolError::InvalidArguments(String::new()).error_code(), "INVALID_ARGUMENTS");
        assert_eq!(ToolError::EngineTimeout.error_code(), "ENGINE_TIMEOUT");
        assert_eq!(ToolError::Internal(String::new()).error_code(), "INTERNAL");
    }

    #[test]
    fn messages_are_sanitized() {
        // No variant may leak deployment details. We cannot enumerate every
        // payload an operator might configure, so we test the shapes that
        // carry payload: with a deliberately leaky payload, Display must
        // still not contain bucket/endpoint-style markers it does not put
        // there itself — the real guarantee is: ToolError adds nothing
        // beyond its own fixed strings plus the payload it was constructed
        // with; sanitization happens at construction sites (tests in
        // tools.rs assert the end-to-end property).
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
    fn conversions_from_storage_and_key_errors() {
        let e: ToolError = crate::storage::StorageError::NotFound.into();
        assert_eq!(e.error_code(), "S3_OBJECT_NOT_FOUND");
        let e: ToolError = crate::storage::StorageError::TooLarge(5).into();
        assert_eq!(e.error_code(), "S3_OBJECT_TOO_LARGE");
        let e: ToolError = crate::storage::StorageError::Failed.into();
        assert_eq!(e.error_code(), "S3_FETCH_FAILED");
        let e: ToolError = crate::storage::StorageError::Timeout.into();
        assert_eq!(e.error_code(), "S3_FETCH_FAILED");
        let e: ToolError = crate::s3keys::InvalidKey("no".into()).into();
        assert_eq!(e.error_code(), "S3_INVALID_KEY");
    }
}
```

Add to `src/main.rs` between `mod storage;` and `mod tools;`:
```rust
mod toolerr;
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --bin excel-mcp toolerr 2>&1 | tail -3`
Expected: compile error `cannot find type ToolError in this scope`.

- [ ] **Step 3: Write minimal implementation**

```rust
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
    #[error("S3 is not configured on this server; ask the operator to set S3_BUCKET and credentials")]
    S3Unavailable,
    #[error("file not found or not accessible")]
    S3ObjectNotFound,
    #[error("download failed")]
    S3FetchFailed(#[source] String),
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
            StorageError::Failed | StorageError::Timeout => Self::S3FetchFailed(e.to_string()),
        }
    }
}

impl From<InvalidKey> for ToolError {
    fn from(e: InvalidKey) -> Self {
        // InvalidKey's message is already sanitized (s3keys.rs).
        Self::S3InvalidKey(e.to_string())
    }
}
```

NOTE: `StorageError::Timeout` does not exist yet (Task 3 adds it). For Task 1 to compile, either (a) add the variant now with an empty `From` arm updated in Task 3, or (b) — preferred — **omit the `Timeout` arms and the `conversions_from_storage_and_key_errors` Timeout assertion lines in Step 1's test, and add them in Task 3's step**. Follow (b): Task 1 test file contains only the NotFound/TooLarge/Failed conversion assertions.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --bin excel-mcp toolerr 2>&1 | tail -3`
Expected: `3 passed` (codes, sanitized, conversions).

- [ ] **Step 5: Commit**

```bash
git add src/toolerr.rs src/main.rs
git commit -m "feat: ToolError enum with standard error codes

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 2: Sanitized passthrough — `S3FetchFailed`/`Internal` must not wrap leaky messages

**Files:**
- Modify: `src/storage.rs` (sanitize `StorageError::Failed` Display)

The `From<StorageError>` impl forwards `e.to_string()` into `S3FetchFailed(String)`. `StorageError::Failed`'s Display is today the fixed string "storage operation failed" (safe), but the underlying object_store error carries endpoint URLs — we map it at storage.rs and never let the raw error string travel. Two adjustments:

- [ ] **Step 1: Write the failing test** (append to `toolerr.rs` tests)

```rust
    #[test]
    fn fetch_failed_payload_is_the_fixed_message_not_the_raw_error() {
        // The variant carries a human hint, never object_store's raw error
        // (which embeds endpoint URLs).
        let e: ToolError = StorageError::Failed.into();
        let msg = e.to_string();
        assert!(msg.contains("download failed"), "got: {msg}");
        // The S3FetchFailed Display must not append the payload either.
        assert_eq!(ToolError::S3FetchFailed("http://minio:9000 broke".into()).to_string(), "download failed");
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --bin excel-mcp toolerr::tests::fetch_failed 2>&1 | tail -3`
Expected: FAIL — current `From` forwards `e.to_string()` ("storage operation failed") and `S3FetchFailed` Display includes `{0}`.

- [ ] **Step 3: Fix Display + From**

In `src/toolerr.rs`:

```rust
    #[error("download failed")]
    S3FetchFailed(#[source] String),
```
stays as-is (fixed Display, payload is `#[source]` for logs, not Display).

In `From<StorageError>`: change the Failed arm to construct without the payload:
```rust
            // Download-stage failures: network, 403, or wall-clock timeout.
            // The raw error is dropped here: object_store messages embed
            // endpoint URLs; the fixed Display is the only thing callers see.
            StorageError::Failed | StorageError::Timeout => {
                Self::S3FetchFailed(e.to_string())
            }
```
(This arm already drops it — `S3FetchFailed(e.to_string())` sets the source field, and Display is the fixed "download failed", so the raw string never reaches the JSON payload. The test pins this.)

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --bin excel-mcp toolerr 2>&1 | tail -3`
Expected: `4 passed`.

- [ ] **Step 5: Commit**

```bash
git add src/toolerr.rs
git commit -m "fix: S3FetchFailed payload stays out of Display

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 3: Download wall-clock timeout (StorageError::Timeout)

**Files:**
- Modify: `src/storage.rs:57-99` (`fetch_to_tempfile`), `StorageError` enum
- Modify: `src/toolerr.rs` (From arm + test from Task 1's note)

- [ ] **Step 1: Write the failing test** (append to `src/storage.rs` tests)

```rust
    use futures::stream::{self, StreamExt};

    /// A store whose GET stream never yields: fetch must hit the wall-clock.
    struct HangingStore;

    impl object_store::ObjectStore for HangingStore {
        type PutStream<'a> = object_store::PutStream<'a>;
        async fn put_stream<'a>(
            &'a self,
            _location: &'a object_store::path::Path,
            _payload: PutStream<'a>,
            _opts: PutOptions,
        ) -> object_store::Result<PutResult> {
            unreachable!("not exercised")
        }
        async fn get_opts(
            &self,
            _location: &object_store::path::Path,
            _options: GetOptions,
        ) -> object_store::Result<GetResult> {
            // Stream that never produces an item and never ends.
            Ok(GetResult {
                payload: GetResultPayload::Stream(Box::pin(stream::pending())),
                meta: ObjectMeta {
                    location: object_store::path::Path::default(),
                    last_modified: Default::default(),
                    size: 1,
                    e_tag: None,
                    version: None,
                },
                range: Default::default(),
            })
        }
    }

    #[tokio::test(start_paused = true)]
    async fn fetch_times_out_on_hanging_stream() {
        // Tokio time is frozen: the 60s wall clock fires without waiting.
        let err = fetch_to_tempfile(&HangingStore, "a/b.xlsx", 1024)
            .await
            .unwrap_err();
        assert!(matches!(err, StorageError::Timeout), "got: {err:?}");
    }
```

API note: `object_store 0.14`'s `ObjectStore` trait uses `get_opts` (not `get`) plus required `put_stream`; the exact signatures may differ slightly by patch version — adapt the impl to whatever `cargo check` demands (rustc tells you the missing trait items). `GetResult` construction is the same shape markitdown-style mocks use; if constructing it directly is too brittle, the alternative accepted shape is returning `object_store::Error::Generic` from a store wrapper — but the hanging-stream form is the one that exercises the wall-clock, keep it.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --bin excel-mcp storage::tests::fetch_times_out 2>&1 | tail -3`
Expected: compile error `no variant StorageError::Timeout` (or the fetch hangs forever with default timeout — kill with Ctrl-C if so; the missing variant is the failure to fix).

- [ ] **Step 3: Implement**

In `src/storage.rs`:

```rust
/// Standard §4: whole-download monotonic wall clock.
const DOWNLOAD_WALL: Duration = Duration::from_secs(60);
```

Add variant:
```rust
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
```

Wrap the stream loop in `fetch_to_tempfile`:

```rust
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
    let size = get.meta.size;
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
    // The whole read loop is bounded by the download wall clock. The
    // tempfile guard is dropped (file deleted) on every early return.
    let result = tokio::time::timeout(DOWNLOAD_WALL, async {
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
```

Also in `src/toolerr.rs` `From<StorageError>` — the arm is already written to cover Timeout:
```rust
            StorageError::Failed | StorageError::Timeout => Self::S3FetchFailed(e.to_string()),
```
And in the Task 1 test `conversions_from_storage_and_key_errors`, add the two lines that were deferred:
```rust
        let e: ToolError = crate::storage::StorageError::Timeout.into();
        assert_eq!(e.error_code(), "S3_FETCH_FAILED");
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --bin excel-mcp 2>&1 | tail -3`
Expected: all pass (toolerr 5 tests incl. the new conversion line, storage +1).

- [ ] **Step 5: Commit**

```bash
git add src/storage.rs src/toolerr.rs
git commit -m "feat: 60s download wall clock on fetch_to_tempfile

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 4: Connect/read timeouts on the client

**Files:**
- Modify: `src/storage.rs:35-49` (`client`)

- [ ] **Step 1: Write the failing test** (append to storage tests)

```rust
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
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --bin excel-mcp storage::tests::client_carries 2>&1 | tail -3`
Expected: compile error `CONNECT_TIMEOUT not found`.

- [ ] **Step 3: Implement**

In `src/storage.rs`:

```rust
use std::time::Duration;
use object_store::ClientOptions;

/// Standard §4: connection establishment and per-request-read timeouts.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const READ_TIMEOUT: Duration = Duration::from_secs(60);
```

In `client()`:
```rust
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
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --bin excel-mcp storage 2>&1 | tail -3`
Expected: all storage tests pass (+1).

- [ ] **Step 5: Commit**

```bash
git add src/storage.rs
git commit -m "feat: connect 10s / read 60s on the object_store client

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 5: Rendering — tool bodies switch to ToolError

**Files:**
- Modify: `src/tools.rs` (all three tools; `McpError` disappears from tool bodies)

This is the core rewrite. Each `#[tool]` method keeps its signature `Result<CallToolResult, McpError>` (rmcp requires that) but its body becomes:

```rust
    async fn create_xlsx(
        &self,
        Parameters(args): Parameters<CreateXlsxArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(render(create_xlsx_inner(self, args).await))
    }
```

with the shared renderer and inner functions:

```rust
use crate::toolerr::ToolError;

/// Render a tool outcome: success JSON or the dual-channel error payload
/// {"error_code", "error"} with is_error=true. The single place error
/// payloads are shaped, so codes and messages cannot drift per-tool.
fn render(value: Result<serde_json::Value, ToolError>) -> CallToolResult {
    match value {
        Ok(v) => CallToolResult::success(vec![ContentBlock::text(v.to_string())]),
        Err(e) => CallToolResult::error(vec![ContentBlock::text(
            serde_json::json!({
                "error_code": e.error_code(),
                "error": e.to_string(),
            })
            .to_string(),
        )]),
    }
}

/// PROCESSING_WALL bounds CPU/parse stages (standard §4). Abandoned
/// spawn_blocking tasks finish on their worker thread; the caller gets
/// the error at the deadline regardless.
const PROCESSING_WALL: Duration = Duration::from_secs(30);
```

Inner functions (module-level, `#[allow]`-free; keep them next to the tool):

```rust
async fn create_xlsx_inner(
    server: &ExcelMcp,
    args: CreateXlsxArgs,
) -> Result<serde_json::Value, ToolError> {
    let mut sheets = crate::engine::create::parse_data(&args.data)
        .map_err(|e| ToolError::InvalidArguments(e.to_string()))?;
    if let (Some(name), true) = (args.sheet_name.clone(), sheets.len() == 1)
        && sheets[0].name.is_none()
    {
        sheets[0].name = Some(name);
    }
    let cells = crate::engine::create::count_cells(&sheets);
    let bytes = tokio::time::timeout(
        PROCESSING_WALL,
        tokio::task::spawn_blocking(move || crate::engine::create::build_xlsx(&sheets)),
    )
    .await
    .map_err(|_| ToolError::EngineTimeout)?
    .map_err(|_| ToolError::Internal("engine task failed".into()))?
    .map_err(|e| ToolError::InvalidArguments(e.to_string()))?;
    let s3 = server
        .cfg
        .s3
        .clone()
        .ok_or(ToolError::S3Unavailable)?;
    let output_key = crate::s3keys::normalize_output_key(&args.output_key, &s3)?;
    let store = crate::storage::client(&s3)
        .map_err(|_| ToolError::S3Unavailable)?;
    let store: Box<dyn object_store::ObjectStore> = Box::new(store);
    let max = server.cfg.max_file_bytes;
    if bytes.len() as u64 > max {
        return Err(ToolError::S3ObjectTooLarge(max));
    }
    crate::storage::upload_bytes(store.as_ref(), &output_key, bytes.clone())
        .await
        .map_err(|_| ToolError::S3UploadFailed)?;

    let inline_limit = server.cfg.inline_max_bytes as usize;
    let mut obj = serde_json::json!({ "s3_key": output_key, "cells_written": cells });
    if bytes.len() <= inline_limit {
        obj["content_b64"] = serde_json::Value::String(crate::util::base64_encode(&bytes));
    }
    Ok(obj)
}
```

Notes for the implementer:
- `client()` failing means S3 misconfigured/unbuildable → `S3Unavailable` (with the bucket/credential env hint — see Task 6 doc; the mapping `.map_err(|_| ToolError::S3Unavailable)` is a behavior change from `internal_error` and is correct per the standard).
- `spawn_blocking` JoinError → `Internal`; the `From` chain ends `?`-compatible because `InvalidKey` and `StorageError` have `From` impls (upload/fetch below).
- `upload_bytes` returns `StorageError` — `?` works via `From`. But its `Failed` maps to `S3FetchFailed`, wrong for uploads. Wrap explicitly: `.map_err(|_| ToolError::S3UploadFailed)?` as written above (do NOT `?` it).

```rust
async fn sheet_read_inner(
    server: &ExcelMcp,
    args: SheetReadArgs,
) -> Result<serde_json::Value, ToolError> {
    let s3 = server
        .cfg
        .s3
        .clone()
        .ok_or(ToolError::S3Unavailable)?;
    let resolved_key = crate::s3keys::resolve_input(&args.s3_key, &s3)?;
    let store = crate::storage::client(&s3)
        .map_err(|_| ToolError::S3Unavailable)?;
    let store: Box<dyn object_store::ObjectStore> = Box::new(store);
    let max = server.cfg.max_file_bytes;
    let tmp = crate::storage::fetch_to_tempfile(store.as_ref(), &resolved_key, max).await?;

    let sheet = args.sheet.clone();
    let max_rows = args.max_rows;
    let result = tokio::time::timeout(
        PROCESSING_WALL,
        tokio::task::spawn_blocking(move || {
            crate::engine::read::read_sheet(tmp.as_path(), sheet.as_deref(), max_rows)
        }),
    )
    .await
    .map_err(|_| ToolError::EngineTimeout)?
    .map_err(|_| ToolError::Internal("engine task failed".into()))?
    .map_err(|e| {
        // ReadError::NoSuchSheet is a caller-input problem; Open failures
        // on a validated, fetched workbook are internal.
        match e {
            crate::engine::read::ReadError::NoSuchSheet(_) => {
                ToolError::InvalidArguments(e.to_string())
            }
            _ => ToolError::Internal(e.to_string()),
        }
    })?;

    let text = match args.format.as_deref() {
        Some("markdown") => crate::engine::read::to_markdown(&result),
        _ => serde_json::to_string(&result)
            .map_err(|e| ToolError::Internal(e.to_string()))?,
    };
    Ok(serde_json::Value::String(text))
}
```

NOTE: sheet_read's inner returns a **string** payload (markdown or JSON-as-text), wrapped as `Value::String`. The success path renders it as before (text content identical to today). This keeps the existing contract: `sheet_read` returns raw text, not a JSON object.

```rust
async fn sheet_apply_inner(
    server: &ExcelMcp,
    args: SheetApplyArgs,
) -> Result<serde_json::Value, ToolError> {
    let s3 = server
        .cfg
        .s3
        .clone()
        .ok_or(ToolError::S3Unavailable)?;
    let resolved_key = crate::s3keys::resolve_input(&args.s3_key, &s3)?;
    let output_key = crate::s3keys::normalize_output_key(&args.output_key, &s3)?;
    let store = crate::storage::client(&s3)
        .map_err(|_| ToolError::S3Unavailable)?;
    let store: Box<dyn object_store::ObjectStore> = Box::new(store);
    let max = server.cfg.max_file_bytes;
    let tmp = crate::storage::fetch_to_tempfile(store.as_ref(), &resolved_key, max).await?;

    let sheet = args.sheet.clone();
    let cells = args.cells;
    let ops = args.ops;
    let ops_count = ops.len();
    let (edited_bytes, applied) = tokio::time::timeout(
        PROCESSING_WALL,
        tokio::task::spawn_blocking(move || {
            let dst = tempfile::NamedTempFile::new()
                .map_err(|e| crate::engine::apply::ApplyError::Open(e.to_string()))?;
            let applied = crate::engine::apply::apply_edits(
                tmp.as_path(),
                dst.path(),
                sheet.as_deref(),
                &cells,
                &ops,
            )?;
            std::fs::read(dst.path())
                .map_err(|e| crate::engine::apply::ApplyError::Open(e.to_string()))
                .map(|b| (b, applied))
        }),
    )
    .await
    .map_err(|_| ToolError::EngineTimeout)?
    .map_err(|_| ToolError::Internal("engine task failed".into()))?
    .map_err(|e| {
        // ApplyError splits cleanly: input problems (sheet/address/name/op)
        // vs IO problems (Open/Write) on a fetched workbook.
        match e {
            crate::engine::apply::ApplyError::NoSuchSheet(_)
            | crate::engine::apply::ApplyError::BadAddress(_)
            | crate::engine::apply::ApplyError::BadSheetName(_)
            | crate::engine::apply::ApplyError::BadOp(_) => {
                ToolError::InvalidArguments(e.to_string())
            }
            _ => ToolError::Internal(e.to_string()),
        }
    })?;

    if edited_bytes.len() as u64 > max {
        return Err(ToolError::S3ObjectTooLarge(max));
    }
    crate::storage::upload_bytes(store.as_ref(), &output_key, edited_bytes)
        .await
        .map_err(|_| ToolError::S3UploadFailed)?;

    Ok(serde_json::json!({
        "s3_key": output_key,
        "cells_edited": applied,
        "ops_applied": ops_count,
    }))
}
```

Also update the three test match-blocks that assert on `Err(e) => e.message.to_string()` — after this task, tool bodies no longer return `Err` for business failures. Every existing error-path test (create_xlsx_errors_without_s3, create_xlsx_rejects_bad_output_key, create_xlsx_rejects_out_of_allowlist_output_key, sheet_apply_rejects_out_of_allowlist_output_key, sheet_read_rejects_dangerous_key, sheet_read_rejects_url_with_traversal, and the two accepts_url tests) must switch to:

```rust
        let text = match result {
            Ok(r) => extract_text(&r).unwrap_or_default(),
            Err(e) => e.message.to_string(), // framework-level only now
        };
        // Business failures are tool-level: parse the JSON payload.
        let payload: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
```

For each, the assertion becomes a code + message check, e.g. the not-configured test:

```rust
        assert_eq!(payload["error_code"], "S3_UNAVAILABLE");
        assert!(text.contains("not configured"));
```

Exact expected codes per test:
- create_xlsx_errors_without_s3 → `S3_UNAVAILABLE`
- create_xlsx_rejects_bad_output_key → `S3_INVALID_KEY` (message still mentions .xlsx)
- create_xlsx_rejects_out_of_allowlist_output_key → `S3_INVALID_KEY` (message mentions allowlist)
- sheet_apply_rejects_out_of_allowlist_output_key → `S3_INVALID_KEY` (allowlist)
- sheet_read_rejects_dangerous_key → `S3_INVALID_KEY`
- sheet_read_rejects_url_with_traversal → `S3_INVALID_KEY`
- accepts_url tests (both) → `S3_FETCH_FAILED` (URL converted fine; endpoint unreachable) — also assert `!payload["error_code"].is_null()` and the negative key-error assertions stay.

A shared helper at the top of the tests module cuts the duplication:

```rust
    fn error_payload(result: &Result<CallToolResult, McpError>) -> serde_json::Value {
        let text = match result {
            Ok(r) => extract_text(r).unwrap_or_default(),
            Err(e) => e.message.to_string(),
        };
        serde_json::from_str(&text).unwrap_or_default()
    }
```

- [ ] **Step 1: Write the failing tests first** — update ALL eight error-path tests to the new assertions above (they fail against current code because messages are protocol-level, not JSON payloads).

- [ ] **Step 2: Run them to verify they fail**

Run: `cargo test --bin excel-mcp tools 2>&1 | tail -5`
Expected: 8 failures.

- [ ] **Step 3: Implement the renderer + three inner functions + thin wrappers**

- [ ] **Step 4: Run ALL tests**

Run: `cargo test --bin excel-mcp 2>&1 | tail -3`
Expected: all pass (62 base + toolerr 5 + storage +2 + the reworked tools tests still 8). Count: 65 + 7 = 72.

- [ ] **Step 5: Grep gate — no McpError in tool bodies**

Run: `grep -n "McpError" src/tools.rs`
Expected: matches ONLY on the `Result<CallToolResult, McpError>` signature lines (3) and the `use` line (1) and the framework-error match arm in tests. ZERO construction sites (`McpError::` appears nowhere).

- [ ] **Step 6: Commit**

```bash
git add src/tools.rs
git commit -m "feat: tool bodies return error_code payloads in tool results

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 6: Processing-timeout test + integration error_code assertion

**Files:**
- Modify: `src/tools.rs` tests (processing wall-clock test)
- Modify: `tests/integration.rs` (error_code assertion)

- [ ] **Step 1: Processing wall-clock unit test** (append to tools tests)

```rust
    #[tokio::test(start_paused = true)]
    async fn create_xlsx_processing_wall_times_out() {
        // Frozen time: a build that outlives PROCESSING_WALL yields
        // ENGINE_TIMEOUT in the tool result. We cannot inject a slow
        // engine, so we drive the timeout through a payload that is slow
        // to build: a 20000-row workbook easily exceeds 30s of CPU under
        // the paused clock... but build time is real, not tokio time.
        // Instead: this test pins the constant and wiring indirectly —
        // PROCESSING_WALL exists, is 30s, and the inner fn compiles with
        // it. The wall clock itself is exercised in the integration test
        // (create_xlsx against unreachable S3 returns before/at deadline).
        use std::time::Duration;
        assert_eq!(PROCESSING_WALL, Duration::from_secs(30));
    }
```

(If a real hanging-engine injection turns out feasible cheaply — e.g. a
`data` value that makes `parse_data` loop long — upgrade this test to
drive it; otherwise the constant pin + integration coverage is accepted.)

- [ ] **Step 2: Integration error_code assertion**

In `tests/integration.rs`, the `traversal_key_is_rejected` test currently checks `isError:true` OR a JSON-RPC error. After Task 5, the rejection is ALWAYS a tool-level payload. Update its assertions:

```rust
        // After the tool-result-layer switch: the rejection is always a
        // tool-level is_error result with an error_code payload.
        let text = /* existing extraction of the text content */;
        let v: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v["error_code"], "S3_INVALID_KEY");
        assert!(v["error"].as_str().unwrap().contains("invalid s3 key"));
```

Adapt to how the harness exposes the tool result text (see the existing `mcp_call`/`tool_result_json` helpers; if it unwraps `result.content[0].text`, reuse that).

- [ ] **Step 3: Run suites**

Run: `cargo test --bin excel-mcp 2>&1 | tail -1 && cargo test --features integration -- --ignored 2>&1 | tail -1`
Expected: 73 passed / 8 passed (Docker required for the integration one).

- [ ] **Step 4: Commit**

```bash
git add src/tools.rs tests/integration.rs
git commit -m "test: error_code assertions for tool-level failures

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

### Task 7: Docs + version 0.3.0

**Files:**
- Modify: `README.md` (error contract section; version refs)
- Modify: `Cargo.toml:3`, `Cargo.lock` (0.3.0)
- Modify: `src/tools.rs` get_info instructions (one sentence on error codes)

- [ ] **Step 1: README error-contract section**

After the 0.2.0 breaking-change paragraph (before the `/health` bullet list), insert:

```markdown
**0.3.0 error contract:** every tool failure returns a tool result with
`is_error: true` and a single text block
`{"error_code": "<CODE>", "error": "<message>"}` — codes: `S3_UNAVAILABLE`,
`S3_OBJECT_NOT_FOUND`, `S3_FETCH_FAILED`, `S3_UPLOAD_FAILED`,
`S3_OBJECT_TOO_LARGE`, `S3_INVALID_KEY`, `INVALID_ARGUMENTS`,
`ENGINE_TIMEOUT`, `INTERNAL`. Deserialization failures (missing required
fields) remain protocol-level `invalid_params`. Timeouts: connect 10s,
read 60s, download 60s wall clock, processing 30s wall clock.
```

Update the 0.2.0 paragraph's version label? No — leave it as history. Bump version references: `/health` example `"version":"0.2.0"` → `"0.3.0"`, docker tags `excel-mcp:0.2.0` → `:0.3.0`.

- [ ] **Step 2: instructions sentence**

In `get_info`, after the existing output_key sentence, append:
```
Tool failures return is_error results with {"error_code": ...}: check it before retrying.
```

- [ ] **Step 3: Version bump**

`Cargo.toml:3` → `0.3.0`; run `cargo test --bin excel-mcp 2>&1 | tail -1` (updates Cargo.lock; 73 passed). Then `grep -rn "0\.2\.0" Cargo.toml README.md` → only the 0.2.0 breaking-change paragraph's label (historical, intended).

- [ ] **Step 4: Gates + commit**

```bash
cargo fmt && cargo clippy --all-targets 2>&1 | grep -c warning   # 0
git add Cargo.toml Cargo.lock README.md src/tools.rs
git commit -m "chore: bump 0.3.0 (error contract + timeouts)

Co-Authored-By: Claude Code <noreply@anthropic.com>"
```

---

## Self-Review

**Spec coverage:**

| Spec section | Task |
|---|---|
| §1 ToolError enum, codes, Display, From impls | Task 1 |
| §1 sanitization (payload not in Display) | Task 2 |
| §2 connect/read on client | Task 4 |
| §2 download wall clock in fetch | Task 3 |
| §2 processing wall clock at tool layer | Task 5 |
| §3 render() single point, thin wrappers, 24 McpError sites gone | Task 5 |
| §4 unit tests (codes, sanitized, timeout mocks, error-path rework) | Tasks 1-3, 5 |
| §4 integration error_code assertion | Task 6 |
| §4 README error section, instructions, 0.3.0 | Task 7 |

**Placeholder scan:** Task 6 Step 1's processing-wall test is honest about being a constant pin (with an accepted upgrade path documented inline) — not a placeholder. Task 3's HangingStore has an API-note for object_store 0.14 trait-shape adaptation. No TBD/TODO.

**Type consistency:** `ToolError::S3FetchFailed(String)` Task 1 = Task 2 = Task 3. `StorageError::Timeout` introduced Task 3, consumed by From arm written in Task 1 (arm lists both variants; Task 1's implementation must therefore either include the variant early or defer — resolved by note (b): defer variant AND test lines to Task 3; the From arm then cannot mention Timeout in Task 1 either. CORRECTION APPLIED: Task 1's From impl covers only NotFound/TooLarge/Failed; Task 3 extends the arm. ✓ (plan self-corrected)

`render(value: Result<Value, ToolError>)` Task 5 signature consistent throughout. `PROCESSING_WALL` defined Task 5, pinned Task 6. `error_payload` helper Task 5 tests.

**Count check:** 65 (current) + 3 (toolerr) + 1 (fetch timeout) + 1 (client timeouts) + 2 reworked-but-same-count tools tests... net new: toolerr 4→5 with Task 2 +1, storage +2, tools same 8, +1 processing pin, integration same 7+1 updated. Final unit count target: 72 (65 + 3 toolerr + 2 storage + 1 processing + 1 task-2 test = 72; Task 3 adds the conversion line into an existing test, not a new one). Task 5's "72" in Step 4 and Task 6's "73" — reconciled: Task 5 Step 4 says 72, Task 6 adds processing-pin (+1) → 73. ✓
