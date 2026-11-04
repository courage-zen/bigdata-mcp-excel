//! Reading workbooks: calamine for structure/cached values, IronCalc for
//! formula recompute. The merge rule: a cell's `value` is the recomputed
//! value when IronCalc loaded the workbook, else the cached value; `formula`
//! is present whenever the cell holds one.

use std::collections::BTreeMap;
use std::path::Path;

use calamine::{Data as CellData, Range, Reader, Xlsx};
use serde::Serialize;

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct SheetSummary {
    pub name: String,
    /// Dimensions of the requested sheet only; other sheets report 0.
    pub rows: u32,
    pub cols: u32,
}

#[derive(Debug, Serialize)]
pub struct CellOut {
    pub value: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReadResult {
    pub sheets: Vec<SheetSummary>,
    /// Cells keyed by A1 address for the requested sheet.
    pub cells: BTreeMap<String, CellOut>,
    pub merged: Vec<String>,
    pub truncated: bool,
    /// Set when IronCalc recompute was unavailable (workbook features its
    /// importer refuses); values are calamine's cached ones.
    pub values_are_cached: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("workbook could not be opened: {0}")]
    Open(String),
    #[error("sheet not found: {0}")]
    NoSuchSheet(String),
}

/// A1 column letters for a 0-based index (0 -> A, 26 -> AA).
pub fn a1_col(col: usize) -> String {
    let mut n = col as i64;
    let mut out = Vec::new();
    while n >= 26 {
        out.push(b'A' + (n % 26) as u8);
        n = n / 26 - 1;
    }
    out.push(b'A' + (n % 26) as u8);
    out.reverse();
    String::from_utf8(out).unwrap()
}

pub fn a1(row: usize, col: usize) -> String {
    format!("{}{}", a1_col(col), row + 1)
}

/// Read one sheet. `sheet`: name or 0-based index as string; default: first.
pub fn read_sheet(
    path: &Path,
    sheet: Option<&str>,
    max_rows: usize,
) -> Result<ReadResult, ReadError> {
    let mut workbook: Xlsx<_> = Xlsx::new(
        std::fs::File::open(path).map_err(|e| ReadError::Open(e.to_string()))?,
    )
    .map_err(|e| ReadError::Open(e.to_string()))?;
    let names = workbook.sheet_names().to_vec();
    if names.is_empty() {
        return Err(ReadError::Open("workbook has no sheets".into()));
    }
    let name = match sheet {
        None => names[0].clone(),
        Some(sel) => {
            if let Ok(idx) = sel.parse::<usize>() {
                names
                    .get(idx)
                    .cloned()
                    .ok_or_else(|| ReadError::NoSuchSheet(sel.to_string()))?
            } else {
                if !names.iter().any(|n| n == sel) {
                    return Err(ReadError::NoSuchSheet(sel.to_string()));
                }
                sel.to_string()
            }
        }
    };

    let range: Range<CellData> = workbook
        .worksheet_range(&name)
        .map_err(|e| ReadError::Open(e.to_string()))?;
    let formulas: Range<String> = workbook
        .worksheet_formula(&name)
        .unwrap_or_default();

    // IronCalc recompute (fail-soft, pattern from genoffice xlsx-sidecar).
    let computed = recompute(path);

    // Calamine ranges are bbox-local; get_value() takes absolute (row, col)
    // so both lookups share the loop's absolute coordinates. end() is None
    // on an empty sheet, in which case size is 0.
    let val_end = range.end();
    let mut cells = BTreeMap::new();
    let total_rows = val_end.map(|e| (e.0 + 1) as u32).unwrap_or(0);
    let total_cols = val_end.map(|e| (e.1 + 1) as u32).unwrap_or(0);
    let truncated = (total_rows as usize) > max_rows;
    let rows_to_read = (total_rows as usize).min(max_rows);
    for r in 0..rows_to_read {
        for c in 0..total_cols as usize {
            let cached = range.get_value((r as u32, c as u32));
            let formula = formulas.get_value((r as u32, c as u32)).cloned();
            let has_cached = cached.is_some() && !matches!(cached, Some(CellData::Empty));
            if !has_cached && formula.is_none() {
                continue;
            }
            let addr = a1(r, c);
            let value = match computed
                .as_ref()
                .and_then(|m| m.get(&(name.clone(), (r + 1) as u32, (c + 1) as u16)))
            {
                Some(v) => v.clone(),
                None => cached.map(cell_to_json).unwrap_or(serde_json::Value::Null),
            };
            cells.insert(addr, CellOut { value, formula });
        }
    }

    let merged = workbook
        .merge_cells_by_sheet_name(&name)
        .map_err(|e| ReadError::Open(e.to_string()))?
        .iter()
        // Dimensions has no Display; render "A1:B2"-style from 0-based tuples.
        .map(|rng| {
            format!(
                "{}:{}",
                a1(rng.start.0 as usize, rng.start.1 as usize),
                a1(rng.end.0 as usize, rng.end.1 as usize)
            )
        })
        .collect();

    Ok(ReadResult {
        sheets: names
            .iter()
            .map(|n| SheetSummary {
                name: n.clone(),
                rows: if *n == name { total_rows as u32 } else { 0 },
                cols: if *n == name { total_cols as u32 } else { 0 },
            })
            .collect(),
        cells,
        merged,
        truncated,
        values_are_cached: computed.is_none(),
    })
}

fn cell_to_json(data: &CellData) -> serde_json::Value {
    match data {
        CellData::Empty => serde_json::Value::Null,
        CellData::String(s) => serde_json::Value::String(s.clone()),
        CellData::Float(f) => {
            // Whole floats read as integers for cleaner agent output.
            if f.fract() == 0.0 && f.abs() < 9e15 {
                serde_json::json!(*f as i64)
            } else {
                serde_json::json!(f)
            }
        }
        CellData::Bool(b) => serde_json::json!(b),
        CellData::DateTime(dt) => serde_json::json!(dt.to_string()),
        CellData::Int(i) => serde_json::json!(i),
        CellData::Error(e) => serde_json::json!(e.to_string()),
        CellData::DateTimeIso(dt) => serde_json::json!(dt),
        CellData::DurationIso(d) => serde_json::json!(d),
    }
}

/// IronCalc recompute of the whole workbook:
/// `(sheet_name, row, col)` 1-based -> value. None when the import fails
/// (fail-soft: cached values are then returned instead).
fn recompute(path: &Path) -> Option<BTreeMap<(String, u32, u16), serde_json::Value>> {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use ironcalc::base::cell::CellValue;
    use ironcalc::import::load_from_xlsx;

    let path_text = path.to_str()?;
    let load = catch_unwind(AssertUnwindSafe(|| {
        load_from_xlsx(path_text, "en", "UTC", "en")
    }));
    let mut model = load.ok()?.ok()?;
    model.evaluate();

    let mut out: BTreeMap<(String, u32, u16), serde_json::Value> = BTreeMap::new();
    for (sheet_idx, worksheet) in model.workbook.worksheets.iter().enumerate() {
        let sheet_idx = sheet_idx as u32;
        let name = worksheet.name.clone();
        // Iterate only the cells IronCalc actually holds; row-major order.
        for (row, col, _) in worksheet.sheet_data.cells() {
            if let Ok(value) = model.get_cell_value_by_index(sheet_idx, row, col) {
                let json = match value {
                    CellValue::Number(n) => {
                        if n.fract() == 0.0 && n.abs() < 9e15 {
                            serde_json::json!(n as i64)
                        } else {
                            serde_json::json!(n)
                        }
                    }
                    CellValue::String(t) => serde_json::Value::String(t),
                    CellValue::Boolean(b) => serde_json::json!(b),
                    CellValue::None => continue,
                };
                out.insert((name.clone(), row as u32, col as u16), json);
            }
        }
    }
    Some(out)
}

/// Render the read result as a Markdown table (one per requested sheet).
pub fn to_markdown(result: &ReadResult) -> String {
    if result.cells.is_empty() {
        return "(no cells in range)".to_string();
    }
    // Determine grid bounds from A1 addresses.
    let max_row = result
        .cells
        .keys()
        .filter_map(|k| k[1..].parse::<usize>().ok())
        .max()
        .unwrap_or(1);
    let max_col = result
        .cells
        .keys()
        .map(|k| col_letters(k))
        .map(|letters| letters_to_index(&letters))
        .max()
        .unwrap_or(0);

    let cell_value = |r: usize, c: usize| -> String {
        let addr = a1(r, c);
        match result.cells.get(&addr) {
            Some(cell) => match &cell.value {
                serde_json::Value::Null => String::new(),
                // Plain scalars, not JSON: no quotes around text.
                serde_json::Value::String(s) => s.clone(),
                v => v.to_string(),
            },
            None => String::new(),
        }
    };

    let mut lines = Vec::new();
    let header: Vec<String> = (0..=max_col).map(|c| cell_value(0, c)).collect();
    lines.push(format!("| {} |", header.join(" | ")));
    lines.push(format!("|{}|", (0..=max_col).map(|_| "---".to_string()).collect::<Vec<_>>().join("|")));
    for r in 1..max_row {
        let row: Vec<String> = (0..=max_col).map(|c| cell_value(r, c)).collect();
        lines.push(format!("| {} |", row.join(" | ")));
    }
    if result.truncated {
        lines.push("*(truncated)*".to_string());
    }
    if result.values_are_cached {
        lines.push("*(values are cached; formulas not recomputed)*".to_string());
    }
    lines.join("\n")
}

fn col_letters(addr: &str) -> String {
    addr.chars().take_while(|c| c.is_ascii_alphabetic()).collect()
}

fn letters_to_index(letters: &str) -> usize {
    letters
        .bytes()
        .map(|b| (b - b'A' + 1) as usize)
        .fold(0usize, |acc, v| acc * 26 + v)
        .saturating_sub(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::create::{SheetData, build_xlsx};

    fn write_fixture(rows: Vec<Vec<serde_json::Value>>) -> std::path::PathBuf {
        let dir = std::env::temp_dir();
        let p = dir.join(format!("excel-mcp-test-{}.xlsx", uuid::Uuid::new_v4()));
        let sheets = vec![SheetData {
            name: None,
            rows,
        }];
        std::fs::write(&p, build_xlsx(&sheets).unwrap()).unwrap();
        p
    }

    #[test]
    fn a1_addressing() {
        assert_eq!(a1(0, 0), "A1");
        assert_eq!(a1(0, 25), "Z1");
        assert_eq!(a1(0, 26), "AA1");
        assert_eq!(a1(1, 26), "AA2");
        assert_eq!(a1(0, 51), "AZ1");
        assert_eq!(a1(0, 52), "BA1");
    }

    #[test]
    fn reads_values_and_formulas() {
        let p = write_fixture(vec![
            vec![serde_json::json!("h"), serde_json::json!("v")],
            vec![serde_json::json!(1), serde_json::json!(2)],
            vec![serde_json::json!("=SUM(A2:B2)"), serde_json::json!(9)],
        ]);
        let result = read_sheet(&p, None, 500).unwrap();
        assert_eq!(
            result.cells.get("A2").map(|c| &c.value),
            Some(&serde_json::json!(1))
        );
        assert_eq!(
            result.cells.get("A3").map(|c| c.formula.clone()),
            Some(Some("SUM(A2:B2)".to_string()))
        );
        // Recomputed by ironcalc (or cached): A3 must read 3, not empty.
        let a3 = &result.cells["A3"].value;
        assert!(
            *a3 == serde_json::json!(3) || *a3 == serde_json::Value::String("3".into()),
            "A3 was {a3}"
        );
    }

    #[test]
    fn max_rows_truncates_with_flag() {
        let rows: Vec<Vec<serde_json::Value>> =
            (0..10).map(|i| vec![serde_json::json!(i)]).collect();
        let p = write_fixture(rows);
        let result = read_sheet(&p, None, 5).unwrap();
        assert!(result.truncated);
        assert!(result.cells.len() <= 5);
    }

    #[test]
    fn missing_sheet_errors() {
        let p = write_fixture(vec![vec![serde_json::json!(1)]]);
        assert!(read_sheet(&p, Some("nope"), 500).is_err());
    }

    #[test]
    fn sheet_by_index() {
        let p = write_fixture(vec![vec![serde_json::json!(1)]]);
        let result = read_sheet(&p, Some("0"), 500).unwrap();
        assert!(!result.cells.is_empty());
        let bad = read_sheet(&p, Some("99"), 500);
        assert!(bad.is_err());
    }

    #[test]
    fn markdown_table_renders() {
        let p = write_fixture(vec![
            vec![serde_json::json!("h1"), serde_json::json!("h2")],
            vec![serde_json::json!("a"), serde_json::json!(1)],
        ]);
        let result = read_sheet(&p, None, 500).unwrap();
        let md = to_markdown(&result);
        assert!(md.starts_with("| h1 | h2 |"), "got: {md}");
        assert!(md.contains("| a | 1 |"), "got: {md}");
    }
}
