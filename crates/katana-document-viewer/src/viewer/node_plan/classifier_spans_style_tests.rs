use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::node;
use crate::ViewerTextSpan;
use katana_markdown_model::{InlineSpan, KmmNodeKind};

#[test]
fn childless_style_nodes_use_inline_marker_text() {
    let current = node(
        KmmNodeKind::Paragraph,
        "**bold** *em* ~~gone~~",
        inline_marker_children(),
    );
    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_style_node(&spans[0], "bold", true, false, false);
    assert_style_node(&spans[1], "em", false, true, false);
    assert_style_node(&spans[2], "gone", false, false, true);
}

fn inline_marker_children() -> Vec<katana_markdown_model::KmmNode> {
    vec![
        node(
            KmmNodeKind::Strong(InlineSpan {
                text: "**bold**".to_string(),
            }),
            "**bold**",
            Vec::new(),
        ),
        node(
            KmmNodeKind::Emphasis(InlineSpan {
                text: "*em*".to_string(),
            }),
            "*em*",
            Vec::new(),
        ),
        node(
            KmmNodeKind::Strikethrough(InlineSpan {
                text: "~~gone~~".to_string(),
            }),
            "~~gone~~",
            Vec::new(),
        ),
    ]
}

fn assert_style_node(span: &ViewerTextSpan, text: &str, bold: bool, italic: bool, strike: bool) {
    assert_eq!(text, span.text);
    assert_eq!(bold, span.style.bold);
    assert_eq!(italic, span.style.italic);
    assert_eq!(strike, span.style.strikethrough);
}
