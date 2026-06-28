use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::{node, text_node};
use crate::ViewerTextSpan;
use katana_markdown_model::{InlineCodeNode, InlineMathNode, InlineSpan, KmmNode, KmmNodeKind};

#[test]
fn node_spans_cover_commonmark_block_and_child_style_branches() {
    let spans = spans_for_commonmark_cases();

    let rule_spans = ViewerNodeClassifier::node_spans(&spans.rule_node, &ViewerNodeKind::Rule);

    assert_eq!(
        "quoted\n- child\nlet quoted_code = true;",
        spans.quote[0].text
    );
    assert!(rule_spans.is_empty());
    assert!(spans.strike[0].style.strikethrough);
    assert_eq!("gone", spans.strike[0].text);
    assert!(spans.code[0].style.inline_code);
    assert_eq!("code", spans.code[0].text);
    assert!(!spans.math[0].style.inline_code);
    assert!(spans.math[0].style.inline_math);
    assert_eq!("x", spans.math[0].text);
}

struct CommonmarkSpanCases {
    quote: Vec<ViewerTextSpan>,
    rule_node: KmmNode,
    strike: Vec<ViewerTextSpan>,
    code: Vec<ViewerTextSpan>,
    math: Vec<ViewerTextSpan>,
}

fn spans_for_commonmark_cases() -> CommonmarkSpanCases {
    CommonmarkSpanCases {
        quote: node_spans(&block_quote_node(), &ViewerNodeKind::BlockQuote),
        rule_node: thematic_break_node(),
        strike: node_spans(&strikethrough_node(), &ViewerNodeKind::Paragraph),
        code: node_spans(&inline_code_node(), &ViewerNodeKind::Paragraph),
        math: node_spans(&inline_math_node(), &ViewerNodeKind::Paragraph),
    }
}

fn node_spans(node: &KmmNode, kind: &ViewerNodeKind) -> Vec<ViewerTextSpan> {
    ViewerNodeClassifier::node_spans(node, kind)
}

fn block_quote_node() -> KmmNode {
    node(
        KmmNodeKind::BlockQuote,
        "> quoted\n> - child\n> ```rust\n> let quoted_code = true;\n> ```",
        vec![paragraph_node("quoted", vec![text_node("quoted")])],
    )
}

fn thematic_break_node() -> KmmNode {
    node(KmmNodeKind::ThematicBreak, "---", Vec::new())
}

fn strikethrough_node() -> KmmNode {
    node(
        KmmNodeKind::Paragraph,
        "~~gone~~",
        vec![node(
            KmmNodeKind::Strikethrough(InlineSpan {
                text: "~~gone~~".to_string(),
            }),
            "~~gone~~",
            vec![text_node("gone")],
        )],
    )
}

fn inline_code_node() -> KmmNode {
    node(
        KmmNodeKind::Paragraph,
        "`code`",
        vec![node(
            KmmNodeKind::InlineCode(InlineCodeNode {
                code: "code".to_string(),
            }),
            "`code`",
            Vec::new(),
        )],
    )
}

fn inline_math_node() -> KmmNode {
    node(
        KmmNodeKind::Paragraph,
        "$x$",
        vec![node(
            KmmNodeKind::InlineMath(InlineMathNode {
                expression: "x".to_string(),
            }),
            "$x$",
            Vec::new(),
        )],
    )
}

fn paragraph_node(raw: &str, children: Vec<KmmNode>) -> KmmNode {
    node(KmmNodeKind::Paragraph, raw, children)
}
