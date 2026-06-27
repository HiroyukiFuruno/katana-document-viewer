use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::{cell, list_item, node, text_node};
use katana_markdown_model::{
    DollarMathBlockNode, KmmNode, KmmNodeKind, ListNode, TableNode, TableRow,
};

#[test]
fn node_text_covers_math_table_and_list_text() {
    assert_text_matches("E=mc^2", math_node(), ViewerNodeKind::Math);
    assert_text_matches("A | B\n1 | 2", table_node_for_text(), ViewerNodeKind::Table);
    assert_text_matches(
        "`Broken | Done",
        unmatched_backtick_node(),
        ViewerNodeKind::Table,
    );
    assert_text_matches(
        "3. child\n[/]\n1. default",
        ordered_list_kmm_node(),
        ViewerNodeKind::List,
    );
}

fn assert_text_matches(expected: &str, node: KmmNode, kind: ViewerNodeKind) {
    assert_eq!(expected, text(&node, kind));
}

fn math_node() -> KmmNode {
    node(
        KmmNodeKind::DollarMathBlock(DollarMathBlockNode {
            expression: "E=mc^2".to_string(),
        }),
        "$$E=mc^2$$",
        Vec::new(),
    )
}

fn table_node_for_text() -> KmmNode {
    node(table_kind(), "| A | B |", Vec::new())
}

fn unmatched_backtick_node() -> KmmNode {
    node(
        KmmNodeKind::Table(TableNode {
            alignments: Vec::new(),
            rows: vec![TableRow {
                cells: vec![cell("`Broken"), cell("Done")],
            }],
        }),
        "| `Broken | Done |",
        Vec::new(),
    )
}

fn ordered_list_kmm_node() -> KmmNode {
    node(ordered_list_kind(), "1. child", Vec::new())
}

#[test]
fn node_text_covers_quote_and_alert_text() {
    let quote = node(KmmNodeKind::BlockQuote, "> quoted\n> body", Vec::new());
    assert_eq!("quoted\nbody", text(&quote, ViewerNodeKind::BlockQuote));

    let quote_child = node(KmmNodeKind::BlockQuote, "> child", vec![text_node("child")]);
    assert_eq!("child", text(&quote_child, ViewerNodeKind::BlockQuote));

    let alert = node(
        KmmNodeKind::Alert {
            label: "WARNING".to_string(),
        },
        "> [!WARNING]\n> body",
        Vec::new(),
    );
    assert_eq!(
        "WARNING: body",
        text(
            &alert,
            ViewerNodeKind::Alert {
                label: "WARNING".to_string(),
            },
        )
    );
}

#[test]
fn paragraph_soft_line_breaks_match_export_surface_lines() {
    let paragraph = node(
        KmmNodeKind::Paragraph,
        "first source line\nsecond source line\nthird source line",
        vec![
            text_node("first source line"),
            text_node("\n"),
            text_node("second source line"),
            text_node("\n"),
            text_node("third source line"),
        ],
    );

    assert_eq!(
        "first source line\nsecond source line\nthird source line",
        text(&paragraph, ViewerNodeKind::Paragraph)
    );
}

#[test]
fn block_quote_text_keeps_nested_and_decorated_quote_lines() {
    let nested = node(
        KmmNodeKind::BlockQuote,
        "> Outer quote\n> > Inner quote\n> > > Even deeper",
        Vec::new(),
    );
    assert_eq!(
        "Outer quote\n  Inner quote\n    Even deeper",
        text(&nested, ViewerNodeKind::BlockQuote)
    );

    let decorated = node(
        KmmNodeKind::BlockQuote,
        "> **Bold quote**\n>\n> - List item 1\n> - List item 2\n>\n> ```rust\n> let quoted_code = true;\n> ```",
        Vec::new(),
    );
    assert_eq!(
        "Bold quote\n\n- List item 1\n- List item 2\n\nlet quoted_code = true;",
        text(&decorated, ViewerNodeKind::BlockQuote)
    );
}

#[test]
fn legacy_note_blockquote_text_stays_on_single_quote_line() {
    let quote = node(
        KmmNodeKind::Alert {
            label: "NOTE".to_string(),
        },
        "> **Note**\n> GitHub では note 系ブロックを blockquote として表現する。",
        Vec::new(),
    );

    assert_eq!(
        "Note GitHub では note 系ブロックを blockquote として表現する。",
        text(&quote, ViewerNodeKind::BlockQuote)
    );
}

fn text(node: &KmmNode, kind: ViewerNodeKind) -> String {
    ViewerNodeClassifier::node_text(node, &kind)
}

fn ordered_list_kind() -> KmmNodeKind {
    KmmNodeKind::List(ordered_list_data())
}

fn ordered_list_data() -> ListNode {
    ListNode {
        ordered: true,
        task_markers: vec!["[/]".to_string()],
        items: vec![
            list_item(Some(3), None, vec![text_node("child")]),
            list_item(None, Some("[/]"), Vec::new()),
            list_item(None, None, vec![text_node("default")]),
        ],
    }
}

fn table_kind() -> KmmNodeKind {
    KmmNodeKind::Table(table_node())
}

fn table_node() -> TableNode {
    TableNode {
        alignments: Vec::new(),
        rows: vec![
            TableRow {
                cells: vec![cell("A"), cell("B")],
            },
            TableRow {
                cells: vec![cell("1"), cell("2")],
            },
        ],
    }
}
