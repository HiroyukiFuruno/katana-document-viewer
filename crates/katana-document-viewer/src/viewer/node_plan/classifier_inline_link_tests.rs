use super::super::{ViewerHtmlRole, ViewerNodeClassifier, ViewerNodeKind};
use super::inline_test_support::*;
use super::test_support::node;
use crate::ViewerTextSpan;
use katana_markdown_model::{HtmlBlockRole, KmmNodeKind};

#[test]
fn link_label_emoji_keeps_link_action_and_emoji_rendering() {
    let current = node(
        KmmNodeKind::Paragraph,
        "[🧪 test](https://example.com)",
        vec![node(
            link_kind("🧪 test"),
            "[🧪 test](https://example.com)",
            Vec::new(),
        )],
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_eq!("🧪", spans[0].text);
    assert!(spans[0].style.emoji);
    assert_eq!("https://example.com", spans[0].link_target);
    assert_eq!("https://example.com", spans[1].link_target);
}

#[test]
fn node_spans_preserve_inline_styles_links_and_html_marks() {
    let current = styled_span_paragraph();

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_styled_spans(&spans);
}

#[test]
fn footnote_reference_spans_use_markdown_label_and_internal_target() {
    let current = node(
        KmmNodeKind::Paragraph,
        "本文[^1]",
        vec![node(footnote_reference_kind("1"), "[^1]", Vec::new())],
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_eq!("[1]", spans[0].text);
    assert_eq!("#fn-1", spans[0].link_target);
}

#[test]
fn footnote_definition_spans_include_bidirectional_backlink() {
    let current = node(
        footnote_definition_kind("1", "note"),
        "[^1]: note",
        Vec::new(),
    );

    let spans = ViewerNodeClassifier::node_spans(
        &current,
        &ViewerNodeKind::FootnoteDefinition {
            label: "1".to_string(),
        },
    );

    assert_eq!("1. ", spans[0].text);
    assert!(spans[0].link_target.is_empty());
    let backlink = spans.iter().find(|span| span.text == "↩");
    assert!(backlink.is_some_and(|span| span.link_target == "#fnref-1"));
}

#[test]
fn html_block_spans_preserve_anchor_targets() {
    let raw = r#"<p align="center"><a href="sample.md">English</a> | 日本語</p>"#;
    let current = node(
        KmmNodeKind::HtmlBlock(HtmlBlockRole::Centered),
        raw,
        Vec::new(),
    );

    let spans = ViewerNodeClassifier::node_spans(
        &current,
        &ViewerNodeKind::Html {
            role: ViewerHtmlRole::Centered,
        },
    );

    assert_eq!("English", spans[0].text);
    assert_eq!("sample.md", spans[0].link_target);
    assert!(spans[1].text.contains("日本語"));
}

fn assert_styled_spans(spans: &[ViewerTextSpan]) {
    assert!(spans[0].style.bold);
    assert!(spans[1].style.italic);
    assert_eq!("https://example.com", spans[2].link_target);
    assert!(spans[2].style.underline);
    assert!(spans[3].style.highlight);
    assert_eq!("https://html.example", spans[4].link_target);
    assert!(spans[4].style.underline);
}
