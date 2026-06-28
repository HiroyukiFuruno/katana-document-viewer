use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::node;
use katana_markdown_model::{KmmNodeKind, TableNode, TableRow};

#[test]
fn table_spans_preserve_inline_code_cells_without_raw_backticks() {
    let current = node(
        KmmNodeKind::Table(TableNode {
            alignments: Vec::new(),
            rows: vec![TableRow {
                cells: vec![
                    super::test_support::cell("`PreviewPane`"),
                    super::test_support::cell("Role"),
                ],
            }],
        }),
        "| `PreviewPane` | Role |",
        Vec::new(),
    );

    let text = ViewerNodeClassifier::node_text(&current, &ViewerNodeKind::Table);
    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Table);

    assert_eq!("PreviewPane | Role", text);
    assert!(
        spans
            .iter()
            .any(|span| span.text == "PreviewPane" && span.style.inline_code)
    );
    assert!(spans.iter().all(|span| span.text != "`PreviewPane`"));
}

#[test]
fn table_spans_keeps_unclosed_inline_code_as_plain_text() {
    let current = node(
        KmmNodeKind::Table(TableNode {
            alignments: Vec::new(),
            rows: vec![TableRow {
                cells: vec![
                    super::test_support::cell("`Broken"),
                    super::test_support::cell("Done"),
                ],
            }],
        }),
        "| `Broken | Done |",
        Vec::new(),
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Table);

    assert!(spans.iter().any(|span| span.text == "`Broken"));
    assert!(spans.iter().any(|span| span.text == "Done"));
}

#[test]
fn table_spans_omit_gfm_separator_row() {
    let current = table_with_separator_row();
    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Table);
    let text = spans
        .iter()
        .map(|span| span.text.as_str())
        .collect::<String>();

    assert_eq!("Feature | Status\nHTML alignment | covered", text);
}

#[test]
fn table_fallback_paragraph_spans_keep_export_surface_wrapped_text() {
    let current = node(
        KmmNodeKind::Table(TableNode {
            alignments: Vec::new(),
            rows: vec![TableRow {
                cells: vec![
                    super::test_support::cell("↑ \"English"),
                    super::test_support::cell("日本語\" should appear on the same line, centered."),
                ],
            }],
        }),
        "↑ \"English | 日本語\" should appear on the same line, centered.",
        Vec::new(),
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);
    let text = spans
        .iter()
        .map(|span| span.text.as_str())
        .collect::<String>();

    assert_eq!(
        "↑ \"English | 日本語\" should appear on the same line, centered\n.",
        text
    );
}

fn table_with_separator_row() -> katana_markdown_model::KmmNode {
    node(
        KmmNodeKind::Table(TableNode {
            alignments: Vec::new(),
            rows: vec![
                row("Feature", "Status"),
                row("---", "---"),
                row("HTML alignment", "covered"),
            ],
        }),
        "| Feature | Status |\n| --- | --- |\n| HTML alignment | covered |",
        Vec::new(),
    )
}

fn row(left: &str, right: &str) -> TableRow {
    TableRow {
        cells: vec![
            super::test_support::cell(left),
            super::test_support::cell(right),
        ],
    }
}

#[test]
fn table_spans_ignore_empty_inline_code_cells() {
    let current = node(
        KmmNodeKind::Table(TableNode {
            alignments: Vec::new(),
            rows: vec![TableRow {
                cells: vec![super::test_support::cell("``")],
            }],
        }),
        "| `` |",
        Vec::new(),
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Table);

    assert!(spans.is_empty());
}
