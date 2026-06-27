use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::inline_test_support::*;
use super::test_support::node;
use crate::ViewerTextSpan;
use katana_markdown_model::{HeadingNode, KmmNode, KmmNodeKind};

#[test]
fn heading_spans_strip_markdown_marker() {
    let current = node(
        KmmNodeKind::Heading(HeadingNode {
            level: 2,
            text: "Title".to_string(),
        }),
        "## Title",
        vec![node(text_kind("## Title"), "## Title", Vec::new())],
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Heading { level: 2 });

    assert_eq!("Title", spans[0].text);
}

#[test]
fn heading_spans_preserve_inline_code_without_raw_backticks() {
    let spans = centered_heading_spans(centered_heading_children_without_marker());

    assert_eq!("1.1 ", spans[0].text);
    assert_eq!(r#"<h1 align="center">"#, spans[1].text);
    assert!(spans[1].style.inline_code);
    assert_eq!(" — Centered Heading", spans[2].text);
    assert!(!spans.iter().any(|span| span.text.contains('`')));
}

#[test]
fn heading_spans_strip_marker_before_inline_code_children() {
    let spans = centered_heading_spans(centered_heading_children_with_marker());

    assert_eq!("1.1 ", spans[0].text);
    assert_eq!(r#"<h1 align="center">"#, spans[1].text);
    assert!(spans[1].style.inline_code);
    assert!(!spans.iter().any(|span| span.text.contains('#')));
    assert!(!spans.iter().any(|span| span.text.contains('`')));
}

#[test]
fn heading_spans_preserve_spaces_from_heading_text() {
    let current = node(
        KmmNodeKind::Heading(HeadingNode {
            level: 3,
            text: "4.1 Basic Code Block".to_string(),
        }),
        "### 4.1 Basic Code Block",
        vec![node(text_kind("###"), "###", Vec::new())],
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Heading { level: 3 });

    assert_eq!("4.1 Basic Code Block", spans[0].text);
}

fn centered_heading_spans(children: Vec<KmmNode>) -> Vec<ViewerTextSpan> {
    let current = node(centered_heading_kind(), centered_heading_raw(), children);
    ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Heading { level: 3 })
}

fn centered_heading_kind() -> KmmNodeKind {
    KmmNodeKind::Heading(HeadingNode {
        level: 3,
        text: r#"1.1 `<h1 align="center">` — Centered Heading"#.to_string(),
    })
}

fn centered_heading_raw() -> &'static str {
    r#"### 1.1 `<h1 align="center">` — Centered Heading"#
}

fn centered_heading_children_without_marker() -> Vec<KmmNode> {
    vec![
        node(text_kind("1.1 "), "1.1 ", Vec::new()),
        centered_heading_inline_code(),
        node(
            text_kind(" — Centered Heading"),
            " — Centered Heading",
            Vec::new(),
        ),
    ]
}

fn centered_heading_children_with_marker() -> Vec<KmmNode> {
    vec![
        node(text_kind("###"), "###", Vec::new()),
        node(text_kind(" 1.1 "), " 1.1 ", Vec::new()),
        centered_heading_inline_code(),
        node(
            text_kind(" — Centered Heading"),
            " — Centered Heading",
            Vec::new(),
        ),
    ]
}

fn centered_heading_inline_code() -> KmmNode {
    node(
        inline_code_kind(r#"<h1 align="center">"#),
        r#"`<h1 align="center">`"#,
        Vec::new(),
    )
}
