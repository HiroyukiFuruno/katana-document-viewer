use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::{node, text_node};
use crate::ViewerTextSpan;
use katana_markdown_model::{DescriptionItem, HeadingNode, InlineSpan, KmmNodeKind};

#[test]
fn node_spans_falls_back_to_plain_when_inline_spans_are_empty() {
    let current = node(KmmNodeKind::Paragraph, "fallback", Vec::new());

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_eq!(vec![ViewerTextSpan::plain("")], spans);
}

#[test]
fn heading_spans_keeps_text_when_markdown_marker_is_missing() {
    let current = node(
        KmmNodeKind::Heading(katana_markdown_model::HeadingNode {
            level: 2,
            text: "Title".to_string(),
        }),
        "Title",
        Vec::new(),
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Heading { level: 2 });

    assert_eq!(1, spans.len());
    assert_eq!("Title", spans[0].text);
}

#[test]
fn heading_spans_keep_empty_fallback_without_marker_strip() {
    let current = node(
        KmmNodeKind::Heading(HeadingNode {
            level: 2,
            text: String::new(),
        }),
        "",
        Vec::new(),
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Heading { level: 2 });

    assert_eq!(1, spans.len());
    assert_eq!("", spans[0].text);
}

#[test]
fn heading_spans_strip_marker_without_following_space() {
    let current = node(
        KmmNodeKind::Heading(HeadingNode {
            level: 2,
            text: "Title".to_string(),
        }),
        "##Title",
        Vec::new(),
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Heading { level: 2 });

    assert_eq!(1, spans.len());
    assert_eq!("Title", spans[0].text);
}

#[test]
fn node_spans_renders_description_list_as_lines() {
    let current = node(
        KmmNodeKind::DescriptionList {
            items: vec![
                DescriptionItem {
                    term: "Term".to_string(),
                    description: "Definition".to_string(),
                },
                DescriptionItem {
                    term: "Next".to_string(),
                    description: "Item".to_string(),
                },
            ],
        },
        "Term: Definition\nNext: Item",
        Vec::new(),
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_eq!(1, spans.len());
    assert_eq!("Term: Definition\nNext: Item", spans[0].text);
}

#[test]
fn inline_node_spans_applies_children_styles_for_strong_nodes() {
    let current = node(
        KmmNodeKind::Paragraph,
        "line",
        vec![node(
            KmmNodeKind::Strong(InlineSpan {
                text: "ignored".to_string(),
            }),
            "line",
            vec![text_node("bold")],
        )],
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_eq!(1, spans.len());
    assert!(spans[0].style.bold);
    assert_eq!("bold", spans[0].text);
}

#[test]
fn inline_node_spans_supports_strikethrough_text_markers() {
    let current = node(
        KmmNodeKind::Paragraph,
        "~~x~~",
        vec![node(
            KmmNodeKind::Strikethrough(InlineSpan {
                text: "~~x~~".to_string(),
            }),
            "~~x~~",
            Vec::new(),
        )],
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_eq!(1, spans.len());
    assert!(spans[0].style.strikethrough);
    assert_eq!("x", spans[0].text);
}

#[test]
fn inline_node_spans_applies_children_styles_for_emphasis_nodes() {
    let current = node(
        KmmNodeKind::Paragraph,
        "*x*",
        vec![node(
            KmmNodeKind::Emphasis(InlineSpan {
                text: "*x*".to_string(),
            }),
            "*x*",
            vec![text_node("x")],
        )],
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_eq!(1, spans.len());
    assert!(spans[0].style.italic);
    assert_eq!("x", spans[0].text);
}

#[test]
fn heading_spans_preserves_text_when_heading_marker_count_is_invalid() {
    let current = node(
        KmmNodeKind::Heading(HeadingNode {
            level: 2,
            text: "#######title".to_string(),
        }),
        "#######title",
        Vec::new(),
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Heading { level: 2 });

    assert_eq!(1, spans.len());
    assert_eq!("#######title", spans[0].text);
}
