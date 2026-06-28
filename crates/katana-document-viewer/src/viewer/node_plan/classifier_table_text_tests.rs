use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::{cell, node};
use katana_markdown_model::{KmmNodeKind, TableNode, TableRow};

#[test]
fn table_text_omits_gfm_separator_row() {
    let current = node(
        KmmNodeKind::Table(TableNode {
            alignments: Vec::new(),
            rows: vec![
                TableRow {
                    cells: vec![cell("Feature"), cell("Status")],
                },
                TableRow {
                    cells: vec![cell("---"), cell("---")],
                },
                TableRow {
                    cells: vec![cell("HTML alignment"), cell("covered")],
                },
            ],
        }),
        "| Feature | Status |\n| --- | --- |\n| HTML alignment | covered |",
        Vec::new(),
    );

    assert_eq!(
        "Feature | Status\nHTML alignment | covered",
        ViewerNodeClassifier::node_text(&current, &ViewerNodeKind::Table)
    );
}

#[test]
fn pipe_text_without_separator_row_stays_paragraph() {
    let current = node(
        KmmNodeKind::Table(TableNode {
            alignments: Vec::new(),
            rows: vec![TableRow {
                cells: vec![
                    cell("↑ \"English"),
                    cell("日本語\" should appear on the same line, centered."),
                ],
            }],
        }),
        "↑ \"English | 日本語\" should appear on the same line, centered.",
        Vec::new(),
    );

    assert_eq!(
        Some(ViewerNodeKind::Paragraph),
        ViewerNodeClassifier::node_kind(&current.kind)
    );
}
