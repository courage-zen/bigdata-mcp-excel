//! The MCP tool surface: one struct per tool's typed args, #[tool] methods
//! on a stateful `ExcelMcp` handler.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::tool;
use rmcp::{tool_handler, tool_router};
use rmcp::{ErrorData as McpError, ServerHandler};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::Config;

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
    /// Optional explicit output key (must end with .xlsx); default is
    /// generated under S3_OUTPUT_PREFIX.
    #[serde(default)]
    pub output_key: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SheetReadArgs {
    /// S3 key of the workbook to read.
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

#[tool_router]
impl ExcelMcp {
    pub fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    #[tool(
        description = "Create a new Excel (.xlsx) workbook from data: a 2-D array of cell values, or {\"sheets\":[{\"name\",\"rows\"}]} for several sheets. Strings starting with \"=\" are formulas. Returns the output s3_key and the file bytes base64-encoded when small."
    )]
    async fn create_xlsx(
        &self,
        Parameters(args): Parameters<CreateXlsxArgs>,
    ) -> Result<CallToolResult, McpError> {
        let mut sheets = crate::engine::create::parse_data(&args.data)
            .map_err(|e| McpError::invalid_params(e.to_string(), None))?;
        // A bare 2-D array plus sheet_name: name the only sheet.
        if let (Some(name), true) = (args.sheet_name.clone(), sheets.len() == 1) {
            if sheets[0].name.is_none() {
                sheets[0].name = Some(name);
            }
        }
        let cells = crate::engine::create::count_cells(&sheets);
        let bytes = tokio::task::spawn_blocking(move || {
            crate::engine::create::build_xlsx(&sheets)
        })
        .await
        .map_err(|_| McpError::internal_error("engine task failed", None))?
        .map_err(|e| McpError::invalid_params(e.to_string(), None))?;

        // Resolve output key: explicit (validated) or generated.
        let output_key = match args.output_key.as_deref() {
            Some(explicit) => crate::s3keys::normalize_output_key(explicit)
                .map_err(|e| McpError::invalid_params(e.to_string(), None))?,
            None => crate::storage::generated_output_key(&self.cfg.output_prefix),
        };
        let s3 = self.cfg.s3.clone().ok_or_else(|| {
            McpError::internal_error(
                "S3 output is not configured on this server; ask the operator to set S3_BUCKET and credentials",
                None,
            )
        })?;
        let store = crate::storage::client(&s3)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        let store: Box<dyn object_store::ObjectStore> = Box::new(store);
        let max = self.cfg.max_file_bytes;
        if bytes.len() as u64 > max {
            return Err(McpError::internal_error(
                format!("output exceeds the {max} byte limit"),
                None,
            ));
        }
        crate::storage::upload_bytes(store.as_ref(), &output_key, bytes.clone())
            .await
            .map_err(|_| McpError::internal_error("upload failed", None))?;

        let inline_limit = self.cfg.inline_max_bytes as usize;
        let mut obj = serde_json::json!({ "s3_key": output_key, "cells_written": cells });
        if bytes.len() <= inline_limit {
            obj["content_b64"] =
                serde_json::Value::String(crate::util::base64_encode(&bytes));
        }
        Ok(CallToolResult::success(vec![ContentBlock::text(
            obj.to_string(),
        )]))
    }

    #[tool(
        description = "Read one sheet of an .xlsx workbook from S3: returns sheet list, cells (A1-keyed values plus formulas), merged ranges and a truncation flag. Values are recomputed with a formula engine when possible, else cached values are returned (values_are_cached=true)."
    )]
    async fn sheet_read(
        &self,
        Parameters(args): Parameters<SheetReadArgs>,
    ) -> Result<CallToolResult, McpError> {
        let s3 = self.cfg.s3.clone().ok_or_else(|| {
            McpError::internal_error(
                "S3 output is not configured on this server; ask the operator to set S3_BUCKET and credentials",
                None,
            )
        })?;
        crate::s3keys::validate_input_key(&args.s3_key, &s3)
            .map_err(|e| McpError::invalid_params(e.to_string(), None))?;
        let store = crate::storage::client(&s3)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        let store: Box<dyn object_store::ObjectStore> = Box::new(store);
        let max = self.cfg.max_file_bytes;
        let tmp = crate::storage::fetch_to_tempfile(store.as_ref(), &args.s3_key, max)
            .await
            .map_err(|e| McpError::invalid_params(e.to_string(), None))?;

        let sheet = args.sheet.clone();
        let max_rows = args.max_rows;
        let result = tokio::task::spawn_blocking(move || {
            crate::engine::read::read_sheet(tmp.as_path(), sheet.as_deref(), max_rows)
        })
        .await
        .map_err(|_| McpError::internal_error("engine task failed", None))?
        .map_err(|e| McpError::invalid_params(e.to_string(), None))?;

        let text = match args.format.as_deref() {
            Some("markdown") => crate::engine::read::to_markdown(&result),
            _ => serde_json::to_string(&result)
                .map_err(|e| McpError::internal_error(e.to_string(), None))?,
        };
        Ok(CallToolResult::success(vec![ContentBlock::text(text)]))
    }
}

#[tool_handler]
impl ServerHandler for ExcelMcp {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions(
                "Excel workbook tools over S3. create_xlsx builds a new .xlsx from data; \
                 sheet_read returns cells, formulas and recomputed values for one sheet; \
                 sheet_apply edits an existing workbook and writes a new copy.",
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
            output_prefix: "excel-mcp-out/".into(),
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

    #[tokio::test]
    async fn create_xlsx_errors_without_s3() {
        let server = ExcelMcp::new(test_cfg());
        let result = server
            .create_xlsx(Parameters(CreateXlsxArgs {
                data: serde_json::json!([[1, 2], [3, "=SUM(A1:B1)"]]),
                sheet_name: None,
                output_key: None,
            }))
            .await;
        // The not-configured error surfaces as an MCP error.
        let msg = match result {
            Err(e) => e.message.to_string(),
            Ok(r) => {
                if r.is_error.unwrap_or(false) {
                    extract_text(&r).unwrap_or_default()
                } else {
                    String::new()
                }
            }
        };
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
                output_key: Some("not-an-xlsx.csv".into()),
            }))
            .await;
        let msg = match result {
            Err(e) => e.message.to_string(),
            Ok(r) => {
                if r.is_error.unwrap_or(false) {
                    extract_text(&r).unwrap_or_default()
                } else {
                    String::new()
                }
            }
        };
        assert!(
            msg.contains(".xlsx"),
            "expected output-key rejection, got: {msg}"
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
        let msg = match result {
            Err(e) => e.message.to_string(),
            Ok(r) => {
                if r.is_error.unwrap_or(false) {
                    extract_text(&r).unwrap_or_default()
                } else {
                    String::new()
                }
            }
        };
        assert!(
            msg.contains("..") || msg.contains("key"),
            "expected input-key rejection, got: {msg}"
        );
    }
}
