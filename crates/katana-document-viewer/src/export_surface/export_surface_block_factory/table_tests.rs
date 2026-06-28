use super::*;
use katana_markdown_model::{
    ByteRange, LineColumn, LineColumnRange, RawSnippet, SourceSpan, TableAlignment, TableCell,
    TableNode, TableRow,
};

#[test]
fn fallback_without_contract_becomes_wrapped_text() {
    let mut blocks = Vec::new();
    let table = TableNode {
        alignments: vec![TableAlignment::Left],
        rows: vec![row(vec!["header"])],
    };
    let theme = crate::KdvThemeSnapshot::katana_light();

    SurfaceBlockFactory::append_table(&mut blocks, &table, "header", 0, 0, &theme);

    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].text_for_tests(), "header");
}

#[test]
fn quote_depth_forces_contract_table_to_wrapped_text() {
    let mut blocks = Vec::new();
    let table = contract_table();
    let theme = crate::KdvThemeSnapshot::katana_light();

    SurfaceBlockFactory::append_table(&mut blocks, &table, "fallback", 1, 0, &theme);

    assert!(blocks.len() >= 2);
}

#[test]
fn valid_contract_table_becomes_table_block() {
    let mut blocks = Vec::new();
    let table = contract_table();
    let theme = crate::KdvThemeSnapshot::katana_light();

    SurfaceBlockFactory::append_table(&mut blocks, &table, "fallback", 0, 0, &theme);

    assert_eq!(blocks.len(), 1);
    assert!(matches!(blocks[0], super::super::SurfaceBlock::Table(_)));
}

fn contract_table() -> TableNode {
    TableNode {
        alignments: vec![TableAlignment::Left, TableAlignment::Left],
        rows: vec![
            row(vec!["head1", "head2"]),
            row(vec!["---", "----"]),
            row(vec!["body1", "body2"]),
        ],
    }
}

fn row(values: Vec<&str>) -> TableRow {
    TableRow {
        cells: values
            .into_iter()
            .map(|value| TableCell {
                text: value.to_string(),
                source: source_span(value),
            })
            .collect(),
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
