//! The MCP tool surface: one struct per tool's typed args, #[tool] methods
//! on a stateful `ExcelMcp` handler.

use std::time::Duration;

use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::tool;
use rmcp::{ErrorData as McpError, ServerHandler};
use rmcp::{tool_handler, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::Config;
use crate::toolerr::ToolError;

/// Render a tool outcome: success payload (raw text or JSON) or the
/// dual-channel error payload {"error_code", "error"} with is_error=true.
/// The single place error payloads are shaped, so codes and messages
/// cannot drift per-tool.
fn render(value: Result<serde_json::Value, ToolError>) -> CallToolResult {
    match value {
        Ok(v) => {
            // Value::String carries a raw text payload (sheet_read returns
            // the sheet JSON or markdown as text, not an object): emit it
            // verbatim. Other values are JSON objects: compact-serialize.
            let text = match v {
                serde_json::Value::String(s) => s,
                other => other.to_string(),
            };
            CallToolResult::success(vec![ContentBlock::text(text)])
        }
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

#[derive(Clone)]
pub struct ExcelMcp {
    pub cfg: Config,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateXlsxArgs {
    /// Rows as a 2-D array of cell values, or {"sheets":[{"name","rows"}]}.
    /// Strings starting with "=" are formulas.
    pub data: serde_json::Value,
    /// Optional sheet name when data is a bare 2-D array.
    #[serde(default)]
    pub sheet_name: Option<String>,
    /// Full S3 key for the new workbook (required, must end with .xlsx);
    /// use the caller's workspace path, e.g. workspaces/<ws-id>/<name>.xlsx.
    pub output_key: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SheetReadArgs {
    /// S3 key of the workbook to read, or the full object-store URL of an
    /// attachment (converted to a key automatically).
    pub s3_key: String,
    /// Sheet name or 0-based index as string; default: first sheet.
    #[serde(default)]
    pub sheet: Option<String>,
    /// Row limit; extra rows are dropped and `truncated` is set.
    #[serde(default = "default_max_rows")]
    pub max_rows: usize,
    /// "json" (default) returns the full struct; "markdown" returns a table.
    #[serde(default)]
    pub format: Option<String>,
}

fn default_max_rows() -> usize {
    500
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SheetApplyArgs {
    /// S3 key of the source workbook, or its full object-store URL
    /// (converted to a key automatically). Never modified.
    pub s3_key: String,
    /// S3 key for the edited copy (required, must end with .xlsx).
    pub output_key: String,
    /// Default sheet for cell edits (name); default: first sheet.
    #[serde(default)]
    pub sheet: Option<String>,
    /// Cell edits: {address: "B3", value: ...}; "=" strings are formulas,
    /// null clears the cell.
    #[serde(default)]
    pub cells: Vec<crate::engine::apply::CellEdit>,
    /// Structural operations, applied after cell edits.
    #[serde(default)]
    pub ops: Vec<crate::engine::apply::WorkbookOp>,
}

#[tool_router]
impl ExcelMcp {
    pub fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    #[tool(
        description = "Create a new Excel (.xlsx) workbook from data: a 2-D array of cell values, or {\"sheets\":[{\"name\",\"rows\"}]} for several sheets. Strings starting with \"=\" are formulas. Prefer this over emitting CSV/markdown tables when the deliverable is a real spreadsheet. output_key is required: give the full destination S3 key ending in .xlsx, using the caller's workspace path (workspaces/<ws-id>/<name>.xlsx). Existing objects at that key are overwritten. Returns the output s3_key and the file bytes base64-encoded when small."
    )]
    async fn create_xlsx(
        &self,
        Parameters(args): Parameters<CreateXlsxArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(render(create_xlsx_inner(self, args).await))
    }

    #[tool(
        description = "Read one sheet of an .xlsx workbook from S3. Use this whenever you need data from an Excel file, including platform attachments whose URL points at the object store: pass the attachment's URL or its S3 key as s3_key (URLs are converted to keys automatically). Do NOT download attachments or parse xlsx files locally. Returns sheet list, cells (A1-keyed values plus formulas), merged ranges and a truncation flag. Values are recomputed with a formula engine when possible, else cached values are returned (values_are_cached=true)."
    )]
    async fn sheet_read(
        &self,
        Parameters(args): Parameters<SheetReadArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(render(sheet_read_inner(self, args).await))
    }

    #[tool(
        description = "Edit an .xlsx workbook from S3 and write a new copy: set cell values/formulas, insert rows/columns, add or rename sheets. The source object is never modified. Pass the source workbook's URL or S3 key as s3_key (URLs are converted to keys automatically); output_key must be the full destination S3 key ending in .xlsx, using the caller's workspace path (workspaces/<ws-id>/<name>.xlsx); an existing object there is overwritten."
    )]
    async fn sheet_apply(
        &self,
        Parameters(args): Parameters<SheetApplyArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(render(sheet_apply_inner(self, args).await))
    }
}

async fn create_xlsx_inner(
    server: &ExcelMcp,
    args: CreateXlsxArgs,
) -> Result<serde_json::Value, ToolError> {
    let mut sheets = crate::engine::create::parse_data(&args.data)
        .map_err(|e| ToolError::InvalidArguments(e.to_string()))?;
    // A bare 2-D array plus sheet_name: name the only sheet.
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
    let s3 = server.cfg.s3.clone().ok_or(ToolError::S3Unavailable)?;
    // Output key: agent-supplied, validated with the full input rule set
    // plus the .xlsx suffix rule.
    let output_key = crate::s3keys::normalize_output_key(&args.output_key, &s3)?;
    let store = crate::storage::client(&s3).map_err(|_| ToolError::S3Unavailable)?;
    let store: Box<dyn object_store::ObjectStore> = Box::new(store);
    let max = server.cfg.max_file_bytes;
    if bytes.len() as u64 > max {
        return Err(ToolError::S3ObjectTooLarge(max));
    }
    // StorageError::Failed would map to S3FetchFailed via `?` — wrong for
    // uploads; wrap explicitly.
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

async fn sheet_read_inner(
    server: &ExcelMcp,
    args: SheetReadArgs,
) -> Result<serde_json::Value, ToolError> {
    let s3 = server.cfg.s3.clone().ok_or(ToolError::S3Unavailable)?;
    let resolved_key = crate::s3keys::resolve_input(&args.s3_key, &s3)?;
    let store = crate::storage::client(&s3).map_err(|_| ToolError::S3Unavailable)?;
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
    .map_err(|e| match e {
        // NoSuchSheet is a caller-input problem; Open failures on a
        // validated, fetched workbook are internal.
        crate::engine::read::ReadError::NoSuchSheet(_) => {
            ToolError::InvalidArguments(e.to_string())
        }
        _ => ToolError::Internal(e.to_string()),
    })?;

    // The wire contract is raw text (markdown or JSON-as-text), not a JSON
    // object; keep it by returning Value::String.
    let text = match args.format.as_deref() {
        Some("markdown") => crate::engine::read::to_markdown(&result),
        _ => serde_json::to_string(&result).map_err(|e| ToolError::Internal(e.to_string()))?,
    };
    Ok(serde_json::Value::String(text))
}

async fn sheet_apply_inner(
    server: &ExcelMcp,
    args: SheetApplyArgs,
) -> Result<serde_json::Value, ToolError> {
    let s3 = server.cfg.s3.clone().ok_or(ToolError::S3Unavailable)?;
    let resolved_key = crate::s3keys::resolve_input(&args.s3_key, &s3)?;
    let output_key = crate::s3keys::normalize_output_key(&args.output_key, &s3)?;
    let store = crate::storage::client(&s3).map_err(|_| ToolError::S3Unavailable)?;
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
    .map_err(|e| match e {
        // Input problems (sheet/address/name/op) vs IO problems (Open/Write)
        // on a fetched workbook.
        crate::engine::apply::ApplyError::NoSuchSheet(_)
        | crate::engine::apply::ApplyError::BadAddress(_)
        | crate::engine::apply::ApplyError::BadSheetName(_)
        | crate::engine::apply::ApplyError::BadOp(_) => ToolError::InvalidArguments(e.to_string()),
        _ => ToolError::Internal(e.to_string()),
    })?;

    if edited_bytes.len() as u64 > max {
        return Err(ToolError::S3ObjectTooLarge(max));
    }
    // StorageError::Failed would map to S3FetchFailed via `?` — wrong for
    // uploads; wrap explicitly.
    crate::storage::upload_bytes(store.as_ref(), &output_key, edited_bytes)
        .await
        .map_err(|_| ToolError::S3UploadFailed)?;

    Ok(serde_json::json!({
        "s3_key": output_key,
        "cells_edited": applied,
        "ops_applied": ops_count,
    }))
}

#[tool_handler]
impl ServerHandler for ExcelMcp {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions(
                "Excel workbook tools over S3 — use them instead of parsing xlsx files yourself. \
                 sheet_read returns cells, formulas and recomputed values for one sheet; \
                 sheet_apply edits an existing workbook and writes a new copy; \
                 create_xlsx builds a new .xlsx from data. All inputs are S3 keys, but full \
                 object-store URLs (e.g. platform attachment URLs) are accepted and converted \
                 automatically — do not download attachments to parse them locally. \
                 Every output goes to an explicit output_key you choose under \
                 the caller's workspace path (workspaces/<ws-id>/…); outputs \
                 never land in a shared default prefix.",
            )
            .with_server_info(Implementation::new(
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cfg() -> Config {
        Config {
            host: "127.0.0.1".into(),
            port: 0,
            auth_keys: vec![],
            s3: None,
            inline_max_bytes: 1024 * 1024,
            max_file_bytes: 1024 * 1024,
        }
    }

    fn extract_text(result: &CallToolResult) -> Option<String> {
        match result.content.first() {
            Some(ContentBlock::Text(t)) => Some(t.text.clone()),
            _ => None,
        }
    }

    /// Assert a business failure and unwrap its payload: business failures
    /// are Ok(CallToolResult) with is_error=true and a JSON text body
    /// {"error_code", "error"}; protocol-level Err (rmcp) cannot carry our
    /// codes, so landing there fails this helper. Asserts the expected
    /// `code` and a non-empty message, and returns the message for the
    /// caller's content and no-leak assertions.
    #[track_caller]
    fn error_payload(result: &Result<CallToolResult, McpError>, code: &str) -> String {
        let r = match result {
            Ok(r) => r,
            Err(e) => panic!(
                "business failures must render as tool results, not protocol errors: {}",
                e.message
            ),
        };
        assert_eq!(r.is_error, Some(true), "business failure sets is_error");
        let text = extract_text(r).unwrap_or_default();
        let payload: serde_json::Value = serde_json::from_str(&text)
            .unwrap_or_else(|e| panic!("error payload must be JSON: {text} ({e})"));
        assert_eq!(
            payload["error_code"].as_str(),
            Some(code),
            "wrong error_code, payload: {text}"
        );
        let msg = payload["error"].as_str().unwrap_or_default().to_string();
        assert!(!msg.is_empty(), "error message non-null, payload: {text}");
        msg
    }

    #[tokio::test]
    async fn create_xlsx_rejects_missing_output_key() {
        // The generated-key fallback is gone: a missing output_key must
        // fail at deserialization, before any engine or S3 work.
        let raw = serde_json::json!({"data": [[1]]});
        let parsed: Result<CreateXlsxArgs, _> = serde_json::from_value(raw);
        let err = parsed.expect_err("missing output_key must fail to deserialize");
        assert!(
            err.to_string().contains("output_key"),
            "error names the field: {err}"
        );
    }

    #[test]
    fn render_string_value_emits_raw_text() {
        // sheet_read's payload is raw text (sheet JSON or markdown), not a
        // JSON string value: it must reach the wire verbatim, un-quoted.
        let r = render(Ok(serde_json::Value::String("{\"sheets\":[]}".into())));
        let text = extract_text(&r).unwrap();
        assert_eq!(text, "{\"sheets\":[]}");

        // Object values serialize compactly, as before.
        let r = render(Ok(serde_json::json!({ "s3_key": "a/b.xlsx" })));
        let text = extract_text(&r).unwrap();
        assert_eq!(text, "{\"s3_key\":\"a/b.xlsx\"}");
    }

    #[tokio::test]
    async fn create_xlsx_errors_without_s3() {
        let server = ExcelMcp::new(test_cfg());
        let result = server
            .create_xlsx(Parameters(CreateXlsxArgs {
                data: serde_json::json!([[1, 2], [3, "=SUM(A1:B1)"]]),
                sheet_name: None,
                output_key: "excel-mcp-out/a.xlsx".into(),
            }))
            .await;
        // The not-configured error is a business failure with a code.
        let msg = error_payload(&result, "S3_UNAVAILABLE");
        assert!(msg.contains("not configured"), "got: {msg}");
    }

    #[tokio::test]
    async fn create_xlsx_rejects_bad_output_key() {
        // With S3 configured (but unreachable), a bad output key must be
        // rejected before any network call.
        let mut cfg = test_cfg();
        cfg.s3 = Some(crate::config::S3Config {
            bucket: "b".into(),
            region: "us-east-1".into(),
            access_key_id: "ak".into(),
            secret_access_key: "sk".into(),
            endpoint_url: "http://127.0.0.1:1".into(),
            use_path_style: true,
            allowed_key_prefixes: vec![],
        });
        let server = ExcelMcp::new(cfg);
        let result = server
            .create_xlsx(Parameters(CreateXlsxArgs {
                data: serde_json::json!([[1]]),
                sheet_name: None,
                output_key: "not-an-xlsx.csv".into(),
            }))
            .await;
        let msg = error_payload(&result, "S3_INVALID_KEY");
        assert!(
            msg.contains(".xlsx"),
            "expected output-key rejection, got: {msg}"
        );
    }

    #[tokio::test]
    async fn create_xlsx_rejects_out_of_allowlist_output_key() {
        // With an allowlist configured, a write outside it is refused
        // before any network call — same guarantee reads have.
        let mut cfg = url_cfg();
        cfg.s3.as_mut().unwrap().allowed_key_prefixes = vec!["workspaces/".into()];
        let server = ExcelMcp::new(cfg);
        let result = server
            .create_xlsx(Parameters(CreateXlsxArgs {
                data: serde_json::json!([[1]]),
                sheet_name: None,
                output_key: "excel-mcp-out/a.xlsx".into(),
            }))
            .await;
        let msg = error_payload(&result, "S3_INVALID_KEY");
        assert!(
            msg.contains("allowlist"),
            "expected allowlist rejection, got: {msg}"
        );
    }

    #[tokio::test]
    async fn sheet_apply_rejects_out_of_allowlist_output_key() {
        // Mirror of the create_xlsx test: the write side of sheet_apply
        // obeys the same allowlist. Input key is inside the list so the
        // rejection proves the OUTPUT key check fires.
        let mut cfg = url_cfg();
        cfg.s3.as_mut().unwrap().allowed_key_prefixes = vec!["workspaces/".into()];
        let server = ExcelMcp::new(cfg);
        let result = server
            .sheet_apply(Parameters(SheetApplyArgs {
                s3_key: "workspaces/ws-1/in.xlsx".into(),
                output_key: "excel-mcp-out/a.xlsx".into(),
                sheet: None,
                cells: vec![],
                ops: vec![],
            }))
            .await;
        let msg = error_payload(&result, "S3_INVALID_KEY");
        assert!(
            msg.contains("allowlist"),
            "expected allowlist rejection, got: {msg}"
        );
    }

    #[tokio::test]
    async fn sheet_read_rejects_dangerous_key() {
        // Path-traversal keys must be rejected before any S3 call.
        let mut cfg = test_cfg();
        cfg.s3 = Some(crate::config::S3Config {
            bucket: "b".into(),
            region: "us-east-1".into(),
            access_key_id: "ak".into(),
            secret_access_key: "sk".into(),
            endpoint_url: "http://127.0.0.1:1".into(),
            use_path_style: true,
            allowed_key_prefixes: vec![],
        });
        let server = ExcelMcp::new(cfg);
        let result = server
            .sheet_read(Parameters(SheetReadArgs {
                s3_key: "../etc/passwd".into(),
                sheet: None,
                max_rows: 500,
                format: None,
            }))
            .await;
        let msg = error_payload(&result, "S3_INVALID_KEY");
        assert!(
            msg.contains("..") || msg.contains("key"),
            "expected input-key rejection, got: {msg}"
        );
    }

    /// S3 config for tools-level URL tests: unreachable endpoint so any
    /// network attempt fails AFTER key validation, proving the URL was
    /// accepted and converted.
    fn url_cfg() -> Config {
        Config {
            host: "127.0.0.1".into(),
            port: 0,
            auth_keys: vec![],
            s3: Some(crate::config::S3Config {
                bucket: "uploads".into(),
                region: "us-east-1".into(),
                access_key_id: "ak".into(),
                secret_access_key: "sk".into(),
                endpoint_url: "http://127.0.0.1:1".into(),
                use_path_style: true,
                allowed_key_prefixes: vec![],
            }),
            inline_max_bytes: 1024,
            max_file_bytes: 1024,
        }
    }

    #[tokio::test]
    async fn sheet_read_accepts_url_and_passes_validation() {
        // The URL converts to key "workspaces/a.xlsx"; validation passes
        // (no allowlist configured); the S3 fetch itself fails because the
        // endpoint is unreachable — but with a fetch error, NOT a key error.
        let server = ExcelMcp::new(url_cfg());
        let result = server
            .sheet_read(Parameters(SheetReadArgs {
                s3_key: "http://127.0.0.1:1/uploads/workspaces/a.xlsx".into(),
                sheet: None,
                max_rows: 500,
                format: None,
            }))
            .await;
        // URL converts fine; the endpoint is unreachable, so the failure is
        // the download — a fetch error, never a key error. The fixed
        // "download failed" message carries no endpoint or bucket.
        let msg = error_payload(&result, "S3_FETCH_FAILED");
        assert!(
            !msg.contains("does not match bucket")
                && !msg.contains("no object key")
                && !msg.contains("invalid s3 key"),
            "URL should have been accepted, got: {msg}"
        );
        assert!(!msg.contains("uploads"), "must not name the bucket: {msg}");
        assert!(
            !msg.contains("127.0.0.1"),
            "must not leak the endpoint: {msg}"
        );
    }

    #[tokio::test]
    async fn sheet_read_rejects_url_with_traversal() {
        let server = ExcelMcp::new(url_cfg());
        let result = server
            .sheet_read(Parameters(SheetReadArgs {
                s3_key: "http://127.0.0.1:1/uploads/../etc/passwd".into(),
                sheet: None,
                max_rows: 500,
                format: None,
            }))
            .await;
        let msg = error_payload(&result, "S3_INVALID_KEY");
        assert!(
            msg.contains("invalid s3 key"),
            "traversal in URL must be rejected as a key error, got: {msg}"
        );
    }

    #[tokio::test]
    async fn sheet_apply_accepts_url_and_passes_validation() {
        // Mirror of the sheet_read accept test: the URL converts to a key,
        // validation passes, and the failure (if any) comes from the
        // unreachable endpoint — not from key rejection.
        let server = ExcelMcp::new(url_cfg());
        let result = server
            .sheet_apply(Parameters(SheetApplyArgs {
                s3_key: "http://127.0.0.1:1/uploads/workspaces/a.xlsx".into(),
                output_key: "out/a.xlsx".into(),
                sheet: None,
                cells: vec![],
                ops: vec![],
            }))
            .await;
        let msg = error_payload(&result, "S3_FETCH_FAILED");
        assert!(
            !msg.contains("does not match bucket")
                && !msg.contains("no object key")
                && !msg.contains("invalid s3 key"),
            "URL should have been accepted, got: {msg}"
        );
        assert!(!msg.contains("uploads"), "must not name the bucket: {msg}");
        assert!(
            !msg.contains("127.0.0.1"),
            "must not leak the endpoint: {msg}"
        );
    }
}
