//! Workbook editing (read-modify-write) on umya-spreadsheet. The engine
//! rewrites an existing file; the tool layer never overwrites the source.

use std::path::Path;

use serde::Deserialize;

/// A single cell edit: value or formula. `value` strings starting with '='
/// are formulas. Setting null clears the cell.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CellEdit {
    /// A1 address, e.g. "B3".
    pub address: String,
    /// New cell value; a string starting with '=' writes a formula.
    /// Null clears the cell.
    pub value: serde_json::Value,
}

/// One structural operation on the workbook.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum WorkbookOp {
    /// Insert blank rows before `row` (1-based).
    InsertRows {
        /// Target sheet name (default: first).
        #[serde(default)]
        sheet: Option<String>,
        /// 1-based row number the rows are inserted before.
        row: u32,
        /// How many rows to insert.
        count: u32,
    },
    /// Insert blank columns before `col` ("A" style or 1-based number).
    InsertCols {
        #[serde(default)]
        sheet: Option<String>,
        /// Column letters ("B") or 1-based index as string.
        col: String,
        count: u32,
    },
    /// Append a new sheet.
    AddSheet {
        name: String,
    },
    /// Rename a sheet.
    RenameSheet {
        from: String,
        to: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ApplyError {
    #[error("workbook could not be opened: {0}")]
    Open(String),
    #[error("workbook could not be written")]
    Write,
    #[error("sheet not found: {0}")]
    NoSuchSheet(String),
    #[error("bad cell address: {0}")]
    BadAddress(String),
    #[error("sheet name refused by Excel rules: {0}")]
    BadSheetName(String),
    #[error("invalid op: {0}")]
    BadOp(String),
}

/// Parse "B3" into 1-based (col, row). Column part must be ASCII letters,
/// row part digits.
pub fn parse_a1(address: &str) -> Result<(u32, u32), ApplyError> {
    let addr = address.trim();
    let letters: String = addr.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
    let digits: &str = &addr[letters.len()..];
    if letters.is_empty() || digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
        return Err(ApplyError::BadAddress(address.to_string()));
    }
    let mut col: u32 = 0;
    for b in letters.bytes() {
        let letter = b.to_ascii_uppercase() - b'A' + 1;
        col = col
            .checked_mul(26)
            .and_then(|c| c.checked_add(letter as u32))
            .ok_or_else(|| ApplyError::BadAddress(address.to_string()))?;
    }
    let row: u32 = digits
        .parse()
        .map_err(|_| ApplyError::BadAddress(address.to_string()))?;
    if row == 0 {
        return Err(ApplyError::BadAddress(address.to_string()));
    }
    Ok((col, row))
}

/// Resolve a column spec ("B" or "2") to a 1-based index.
fn parse_col_spec(spec: &str) -> Result<u32, ApplyError> {
    let s = spec.trim();
    if s.chars().all(|c| c.is_ascii_alphabetic()) && !s.is_empty() {
        return parse_a1(&format!("{s}1")).map(|(col, _)| col);
    }
    if let Ok(n) = s.parse::<u32>() {
        if n > 0 {
            return Ok(n);
        }
    }
    Err(ApplyError::BadOp(format!("bad column: {spec}")))
}

/// Find a sheet by name (or default to the first sheet).
fn sheet_index_by_name(book: &umya_spreadsheet::Workbook, sheet: Option<&str>) -> Result<usize, ApplyError> {
    let Some(name) = sheet else {
        return Ok(0);
    };
    for (idx, ws) in book.sheet_collection().iter().enumerate() {
        if ws.name() == name {
            return Ok(idx);
        }
    }
    Err(ApplyError::NoSuchSheet(name.to_string()))
}

/// Apply cell edits and ops to the workbook at `src`, writing to `dst`.
/// Returns the number of cell edits applied.
pub fn apply_edits(
    src: &Path,
    dst: &Path,
    sheet: Option<&str>,
    cells: &[CellEdit],
    ops: &[WorkbookOp],
) -> Result<usize, ApplyError> {
    let mut book = umya_spreadsheet::reader::xlsx::read(src)
        .map_err(|e| ApplyError::Open(e.to_string()))?;
    let target = sheet_index_by_name(&book, sheet)?;

    let mut applied = 0;
    for edit in cells {
        let (col, row) = parse_a1(&edit.address)?;
        let ws = book
            .sheet_mut(target)
            .map_err(|e| ApplyError::Open(e.to_string()))?;
        let cell = ws.cell_mut((col, row));
        match &edit.value {
            serde_json::Value::Null => {
                cell.set_value("");
            }
            serde_json::Value::Bool(b) => {
                cell.set_value_bool(*b);
            }
            serde_json::Value::Number(n) => {
                let f = n.as_f64().unwrap_or(0.0);
                cell.set_value_number(f);
            }
            serde_json::Value::String(s) => {
                if let Some(formula) = s.strip_prefix('=') {
                    cell.set_formula(formula);
                } else {
                    cell.set_value(s.clone());
                }
            }
            other => {
                cell.set_value(other.to_string());
            }
        }
        applied += 1;
    }

    for op in ops {
        match op {
            WorkbookOp::InsertRows { sheet, row, count } => {
                let idx = sheet_index_by_name(&book, sheet.as_deref())?;
                let ws = book
                    .sheet_mut(idx)
                    .map_err(|e| ApplyError::Open(e.to_string()))?;
                ws.insert_new_row(*row, *count);
            }
            WorkbookOp::InsertCols { sheet, col, count } => {
                let idx = sheet_index_by_name(&book, sheet.as_deref())?;
                let col_idx = parse_col_spec(col)?;
                let ws = book
                    .sheet_mut(idx)
                    .map_err(|e| ApplyError::Open(e.to_string()))?;
                ws.insert_new_column_by_index(col_idx, *count);
            }
            WorkbookOp::AddSheet { name } => {
                book.new_sheet(name.clone())
                    .map_err(|e| ApplyError::BadSheetName(e.to_string()))?;
            }
            WorkbookOp::RenameSheet { from, to } => {
                let idx = sheet_index_by_name(&book, Some(from))?;
                book.set_sheet_name(idx, to.to_string())
                    .map_err(|e| ApplyError::BadSheetName(e.to_string()))?;
            }
        }
    }

    umya_spreadsheet::writer::xlsx::write(&book, dst).map_err(|_| ApplyError::Write)?;
    Ok(applied)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::create::{SheetData, build_xlsx};

    fn write_fixture(rows: Vec<Vec<serde_json::Value>>) -> std::path::PathBuf {
        let dir = std::env::temp_dir();
        let p = dir.join(format!("excel-mcp-apply-{}.xlsx", uuid::Uuid::new_v4()));
        let sheets = vec![SheetData { name: None, rows }];
        std::fs::write(&p, build_xlsx(&sheets).unwrap()).unwrap();
        p
    }

    fn out_path() -> std::path::PathBuf {
        std::env::temp_dir().join(format!("excel-mcp-apply-out-{}.xlsx", uuid::Uuid::new_v4()))
    }

    fn read_back_cell(p: &Path, addr: &str) -> Option<String> {
        let book = umya_spreadsheet::reader::xlsx::read(p).unwrap();
        let ws = book.sheet(0).unwrap();
        // umya returns None for cells that hold no value (never written).
        ws.cell(addr)
            .map(|c| c.value().to_string())
            .filter(|s| !s.is_empty())
    }

    #[test]
    fn parse_a1_addresses() {
        assert_eq!(parse_a1("A1").unwrap(), (1, 1));
        assert_eq!(parse_a1("B3").unwrap(), (2, 3));
        assert_eq!(parse_a1("AA10").unwrap(), (27, 10));
        assert_eq!(parse_a1("a1").unwrap(), (1, 1));
        assert!(parse_a1("1A").is_err());
        assert!(parse_a1("A").is_err());
        assert!(parse_a1("A0").is_err());
        assert!(parse_a1("").is_err());
        assert!(parse_a1("A-1").is_err());
    }

    #[test]
    fn applies_cell_edits() {
        let src = write_fixture(vec![
            vec![serde_json::json!("name"), serde_json::json!("qty")],
            vec![serde_json::json!("apple"), serde_json::json!(3)],
        ]);
        let dst = out_path();
        let applied = apply_edits(
            &src,
            &dst,
            None,
            &[
                CellEdit { address: "B2".into(), value: serde_json::json!(9) },
                CellEdit { address: "A3".into(), value: serde_json::json!("cherry") },
                CellEdit { address: "B3".into(), value: serde_json::json!("=SUM(B2:B2)") },
            ],
            &[],
        )
        .unwrap();
        assert_eq!(applied, 3);
        assert_eq!(read_back_cell(&dst, "B2").as_deref(), Some("9"));
        assert_eq!(read_back_cell(&dst, "A3").as_deref(), Some("cherry"));
        // Formula roundtrip: umya keeps the formula (value may be cached).
        let book = umya_spreadsheet::reader::xlsx::read(&dst).unwrap();
        let ws = book.sheet(0).unwrap();
        let formula = ws.cell("B3").map(|c| c.formula().to_string());
        assert!(formula.unwrap_or_default().contains("SUM(B2:B2)"));
    }

    #[test]
    fn insert_rows_shifts_values() {
        let src = write_fixture(vec![
            vec![serde_json::json!("a")],
            vec![serde_json::json!("b")],
            vec![serde_json::json!("c")],
        ]);
        let dst = out_path();
        apply_edits(
            &src,
            &dst,
            None,
            &[],
            &[WorkbookOp::InsertRows { sheet: None, row: 2, count: 1 }],
        )
        .unwrap();
        // Row 2 was blank; "b" moved to row 3, "c" to row 4.
        assert_eq!(read_back_cell(&dst, "A2"), None);
        assert_eq!(read_back_cell(&dst, "A3").as_deref(), Some("b"));
        assert_eq!(read_back_cell(&dst, "A4").as_deref(), Some("c"));
    }

    #[test]
    fn insert_cols_shifts_values() {
        let src = write_fixture(vec![
            vec![serde_json::json!("a"), serde_json::json!("b")],
        ]);
        let dst = out_path();
        apply_edits(
            &src,
            &dst,
            None,
            &[],
            &[WorkbookOp::InsertCols { sheet: None, col: "A".into(), count: 1 }],
        )
        .unwrap();
        assert_eq!(read_back_cell(&dst, "A1"), None);
        assert_eq!(read_back_cell(&dst, "B1").as_deref(), Some("a"));
        assert_eq!(read_back_cell(&dst, "C1").as_deref(), Some("b"));
    }

    #[test]
    fn add_and_rename_sheet() {
        let src = write_fixture(vec![vec![serde_json::json!(1)]]);
        let dst = out_path();
        apply_edits(
            &src,
            &dst,
            None,
            &[],
            &[
                WorkbookOp::AddSheet { name: "Data".into() },
                WorkbookOp::RenameSheet { from: "Sheet1".into(), to: "Main".into() },
            ],
        )
        .unwrap();
        let book = umya_spreadsheet::reader::xlsx::read(&dst).unwrap();
        let names: Vec<String> = book
            .sheet_collection()
            .iter()
            .map(|ws| ws.name().to_string())
            .collect();
        assert!(names.contains(&"Main".to_string()), "names: {names:?}");
        assert!(names.contains(&"Data".to_string()), "names: {names:?}");
    }

    #[test]
    fn missing_sheet_errors() {
        let src = write_fixture(vec![vec![serde_json::json!(1)]]);
        let dst = out_path();
        let err = apply_edits(
            &src,
            &dst,
            Some("nope"),
            &[CellEdit { address: "A1".into(), value: serde_json::json!(1) }],
            &[],
        );
        assert!(err.is_err());
    }

    #[test]
    fn source_file_is_untouched() {
        let src = write_fixture(vec![vec![serde_json::json!("original")], vec![serde_json::json!("x")]]);
        let before = std::fs::read(&src).unwrap();
        let dst = out_path();
        apply_edits(
            &src,
            &dst,
            None,
            &[CellEdit { address: "A1".into(), value: serde_json::json!("edited") }],
            &[],
        )
        .unwrap();
        let after = std::fs::read(&src).unwrap();
        assert_eq!(before, after);
        assert_ne!(before, std::fs::read(&dst).unwrap());
    }
}
