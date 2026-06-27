use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::inline_test_support::*;
use super::test_support::{image, node};
use katana_markdown_model::{CodeBlockRole, KmmNodeKind};

#[test]
fn inline_atom_text_covers_text_styles_code_and_html() {
    assert_inline_text(text_kind("&amp;"), "&");
    assert_inline_text(span_kind("strong", SpanKind::Strong), "strong");
    assert_inline_text(span_kind("em", SpanKind::Emphasis), "em");
    assert_inline_text(span_kind("strike", SpanKind::Strikethrough), "strike");
    assert_inline_text(inline_code_kind("code"), "code");
    assert_inline_text(inline_html_kind("<b>html</b>"), "html");
}

#[test]
fn inline_atom_text_covers_link_image_and_footnotes() {
    assert_inline_text(link_kind("link"), "link");
    assert_inline_text(KmmNodeKind::Image(image("inline image")), "inline image");
    assert_inline_text(footnote_reference_kind("1"), "[1]");
    assert_inline_text(footnote_definition_kind("1", "note"), "note");
}

#[test]
fn inline_atom_image_text_is_used_inside_paragraph_children() {
    let current = node(
        KmmNodeKind::Paragraph,
        "![inline image](a.png)",
        vec![node(
            KmmNodeKind::Image(image("inline image")),
            "![inline image](a.png)",
            Vec::new(),
        )],
    );

    assert_eq!(
        "inline image",
        ViewerNodeClassifier::node_text(&current, &ViewerNodeKind::Paragraph)
    );
}

#[test]
fn inline_atom_text_covers_math_and_emoji() {
    assert_inline_text(inline_math_kind("x+y"), "x+y");
    assert_inline_text(emoji_kind("🙂", "slight_smile"), "🙂");
}

#[test]
fn inline_math_span_uses_math_style_and_surface_fallback_text() {
    let current = node(
        KmmNodeKind::Paragraph,
        "$E = mc^2$",
        vec![node(inline_math_kind("E = mc^2"), "$E = mc^2$", Vec::new())],
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_eq!("E = mc²", span_text(&spans));
    assert!(
        spans
            .iter()
            .filter(|span| !span.text.trim().is_empty())
            .all(|span| span.style.inline_math)
    );
    assert!(spans.iter().all(|span| !span.style.inline_code));
}

#[test]
fn emoji_span_is_marked_for_os_emoji_rendering() {
    let current = node(
        KmmNodeKind::Paragraph,
        "🙂",
        vec![node(emoji_kind("🙂", "slight_smile"), "🙂", Vec::new())],
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_eq!("🙂", spans[0].text);
    assert!(spans[0].style.emoji);
}

#[test]
fn raw_emoji_text_is_split_for_os_emoji_rendering() {
    let current = node(
        KmmNodeKind::Paragraph,
        "Emoji: 🦀 text ⚠️",
        vec![node(
            text_kind("Emoji: 🦀 text ⚠️"),
            "Emoji: 🦀 text ⚠️",
            Vec::new(),
        )],
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_eq!(
        spans
            .iter()
            .map(|span| (span.text.as_str(), span.style.emoji))
            .collect::<Vec<_>>(),
        vec![
            ("Emoji: ", false),
            ("🦀", true),
            (" text ", false),
            ("⚠️", true)
        ]
    );
}

#[test]
fn code_block_spans_reuse_surface_syntax_colors() {
    let current = node(
        KmmNodeKind::CodeBlock(CodeBlockRole::Plain {
            language: Some("rust".to_string()),
        }),
        "```rust\nfn main() {}\n```",
        Vec::new(),
    );

    let spans = ViewerNodeClassifier::node_spans(
        &current,
        &ViewerNodeKind::Code {
            language: Some("rust".to_string()),
        },
    );

    assert!(spans.iter().any(|span| span.style.monospace));
    assert!(spans.iter().any(|span| span.style.color_rgba[3] > 0));
}
