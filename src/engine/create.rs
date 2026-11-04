//! Workbook creation on rust_xlsxwriter: 2-D arrays or {sheets:[{name,rows}]}.

/// One sheet's 2-D data: rows of scalars; `=...` strings are formulas.
pub struct SheetData {
    pub name: Option<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateError {
    #[error("data must be a 2-D array of cell values or {{\"sheets\":[{{\"name\",\"rows\"}}]}}")]
    BadShape,
    #[error("row {0} has a non-array value; data must be an array of arrays")]
    BadRow(usize),
    #[error("workbook could not be written")]
    Write,
}

/// Parse the tool's `data` argument into per-sheet sheet data.
pub fn parse_data(data: &serde_json::Value) -> Result<Vec<SheetData>, CreateError> {
    match data {
        serde_json::Value::Array(rows) => {
            let mut parsed = Vec::new();
            for (i, row) in rows.iter().enumerate() {
                let serde_json::Value::Array(cells) = row else {
                    return Err(CreateError::BadRow(i));
                };
                parsed.push(cells.clone());
            }
            Ok(vec![SheetData {
                name: None,
                rows: parsed,
            }])
        }
        serde_json::Value::Object(map) => {
            let Some(serde_json::Value::Array(sheets)) = map.get("sheets") else {
                return Err(CreateError::BadShape);
            };
            let mut out = Vec::new();
            for sheet in sheets {
                let serde_json::Value::Object(sm) = sheet else {
                    return Err(CreateError::BadShape);
                };
                let name = sm.get("name").and_then(|v| v.as_str()).map(String::from);
                let Some(serde_json::Value::Array(rows)) = sm.get("rows") else {
                    return Err(CreateError::BadShape);
                };
                let mut parsed = Vec::new();
                for row in rows {
                    let serde_json::Value::Array(cells) = row else {
                        return Err(CreateError::BadRow(0));
                    };
                    parsed.push(cells.clone());
                }
                out.push(SheetData {
                    name,
                    rows: parsed,
                });
            }
            Ok(out)
        }
        _ => Err(CreateError::BadShape),
    }
}

/// Excel refuses []:*?/\ in sheet names and caps them at 31 chars.
fn sanitize_sheet_name(name: &str, index: usize) -> String {
    let fallback = if index == 0 {
        "Sheet1".to_string()
    } else {
        format!("Sheet{}", index + 1)
    };
    let base = if name.trim().is_empty() {
        fallback
    } else {
        name.to_string()
    };
    base.chars()
        .map(|c| if "[]:*?/\\".contains(c) { '_' } else { c })
        .take(31)
        .collect()
}

/// Write the sheets to xlsx bytes.
pub fn build_xlsx(sheets: &[SheetData]) -> Result<Vec<u8>, CreateError> {
    let mut workbook = rust_xlsxwriter::Workbook::new();
    for (i, sheet) in sheets.iter().enumerate() {
        let name = sanitize_sheet_name(sheet.name.as_deref().unwrap_or(""), i);
        let worksheet = workbook.add_worksheet();
        worksheet.set_name(&name).map_err(|_| CreateError::Write)?;
        for (r, row) in sheet.rows.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                write_cell(worksheet, r as u32, c as u16, cell)?;
            }
        }
    }
    workbook
        .save_to_buffer()
        .map_err(|_| CreateError::Write)
}

fn write_cell(
    worksheet: &mut rust_xlsxwriter::Worksheet,
    row: u32,
    col: u16,
    value: &serde_json::Value,
) -> Result<(), CreateError> {
    use rust_xlsxwriter::XlsxError;
    let map_err = |_: XlsxError| CreateError::Write;
    match value {
        serde_json::Value::Number(n) => {
            let f = n.as_f64().unwrap_or(0.0);
            worksheet.write_number(row, col, f).map(|_| ()).map_err(map_err)
        }
        serde_json::Value::Bool(b) => worksheet.write_boolean(row, col, *b).map(|_| ()).map_err(map_err),
        serde_json::Value::String(s) => {
            if s.starts_with('=') {
                worksheet.write_formula(row, col, s.as_str()).map(|_| ()).map_err(map_err)
            } else {
                worksheet.write_string(row, col, s.as_str()).map(|_| ()).map_err(map_err)
            }
        }
        serde_json::Value::Null => Ok(()),
        // objects/arrays in cells: stringify, matching lenient tool input
        other => {
            let text = serde_json::to_string(other).unwrap_or_default();
            worksheet.write_string(row, col, text).map(|_| ()).map_err(map_err)
        }
    }
}

/// Total cells written, for the tool's response.
pub fn count_cells(sheets: &[SheetData]) -> usize {
    sheets
        .iter()
        .map(|s| s.rows.iter().map(|r| r.len()).sum::<usize>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_2d_array() {
        let sheets = parse_data(&json!([[1, "a"], [2, "=SUM(A1:A2)"]])).unwrap();
        assert_eq!(sheets.len(), 1);
        assert_eq!(count_cells(&sheets), 4);
    }

    #[test]
    fn parse_named_sheets() {
        let sheets = parse_data(&json!({"sheets": [
            {"name": "Sales", "rows": [[1, 2]]},
            {"name": "Costs", "rows": [[3, 4]]},
        ]}))
        .unwrap();
        assert_eq!(sheets.len(), 2);
        assert_eq!(sheets[1].name.as_deref(), Some("Costs"));
    }

    #[test]
    fn parse_rejects_scalar() {
        assert!(parse_data(&json!("nope")).is_err());
        assert!(parse_data(&json!({"sheets": "nope"})).is_err());
    }

    #[test]
    fn parse_rejects_non_array_row() {
        assert!(parse_data(&json!([[1], "nope"])).is_err());
    }

    #[test]
    fn build_reads_back_with_calamine() {
        let sheets = parse_data(&json!([
            ["name", "qty"],
            ["apple", 3],
        ]))
        .unwrap();
        let bytes = build_xlsx(&sheets).unwrap();
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[0..2], b"PK");

        use calamine::{Data as CellData, Reader, Xlsx};
        let mut reader = Xlsx::new(std::io::Cursor::new(bytes)).unwrap();
        let range = reader.worksheet_range("Sheet1").unwrap();
        assert_eq!(range.get((0, 0)), Some(&CellData::from("name")));
        // Numbers are written as f64, so 3 reads back as Float(3.0).
        assert!(matches!(range.get((1, 1)), Some(CellData::Float(f)) if *f == 3.0));
    }

    #[test]
    fn formula_is_written() {
        let sheets = parse_data(&json!([["=SUM(A1:A2)"]])).unwrap();
        let bytes = build_xlsx(&sheets).unwrap();
        use calamine::{Reader, Xlsx};
        let mut reader = Xlsx::new(std::io::Cursor::new(bytes)).unwrap();
        let formula = reader.worksheet_formula("Sheet1").unwrap();
        assert_eq!(formula.get((0, 0)), Some(&"SUM(A1:A2)".to_string()));
    }

    #[test]
    fn sheet_name_sanitized() {
        let sheets = vec![SheetData {
            name: Some("bad/name[1]".into()),
            rows: vec![vec![json!(1)]],
        }];
        let bytes = build_xlsx(&sheets).unwrap();
        use calamine::{Reader, Xlsx};
        let mut reader = Xlsx::new(std::io::Cursor::new(bytes)).unwrap();
        let names = reader.sheet_names();
        assert!(names.iter().any(|n| !n.contains('/')));
    }

    #[test]
    fn empty_name_gets_default() {
        let sheets = vec![
            SheetData {
                name: None,
                rows: vec![],
            },
            SheetData {
                name: Some("".into()),
                rows: vec![],
            },
        ];
        let bytes = build_xlsx(&sheets).unwrap();
        use calamine::{Reader, Xlsx};
        let mut reader = Xlsx::new(std::io::Cursor::new(bytes)).unwrap();
        let names = reader.sheet_names();
        assert!(names.contains(&"Sheet1".to_string()));
        assert!(names.contains(&"Sheet2".to_string()));
    }
}
