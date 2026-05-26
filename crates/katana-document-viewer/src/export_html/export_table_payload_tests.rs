use super::*;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{
    ByteRange, LineColumn, LineColumnRange, RawSnippet, SourceSpan, TableAlignment, TableCell,
    TableNode, TableRow,
};

#[test]
fn append_falls_back_without_contract() {
    let mut html = String::new();
    let table = table(&[]);
    TableHtmlWriter::append(
        &mut html,
        &table,
        "fallback",
        &KdvThemeSnapshot::katana_light(),
    );
    assert_eq!(html, "<p>fallback</p>\n");
}

#[test]
fn append_short_and_wide_columns() {
    let mut html = String::new();
    let table = table(&[
        row(&["short", "narrow"]),
        row(&["---"]),
        row(&["very wide text", "text"]),
    ]);
    TableHtmlWriter::append(&mut html, &table, "", &KdvThemeSnapshot::katana_light());
    assert!(html.contains("<col data-kdv-column-size=\"short\">"));
    assert!(html.contains("<col data-kdv-column-size=\"wide\">"));
}

#[test]
fn append_uses_missing_alignments_as_unspecified() {
    let mut html = String::new();
    let table = TableNode {
        alignments: vec![TableAlignment::Left],
        rows: vec![
            row(&["left", "wide"]),
            row(&["---", "---"]),
            row(&["left", "narrow"]),
        ],
    };
    TableHtmlWriter::append(&mut html, &table, "", &KdvThemeSnapshot::katana_light());
    assert!(html.contains("data-align=\"left\""));
    assert!(html.contains("data-align=\"unspecified\""));
}

#[test]
fn append_header_does_nothing_without_rows() {
    let mut html = String::new();
    let table = table(&[]);
    TableHtmlWriter::append_header(
        &mut html,
        &table,
        &Vec::new(),
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(html, "");
}

fn row(values: &[&str]) -> TableRow {
    TableRow {
        cells: values
            .iter()
            .map(|value| TableCell {
                text: value.to_string(),
                source: source_span(value),
            })
            .collect(),
    }
}

fn table(rows: &[TableRow]) -> TableNode {
    TableNode {
        alignments: Vec::new(),
        rows: rows.to_vec(),
    }
}

fn source_span(text: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: text.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: text.len() + 1,
            },
        },
        raw: RawSnippet {
            text: text.to_string(),
        },
    }
}
