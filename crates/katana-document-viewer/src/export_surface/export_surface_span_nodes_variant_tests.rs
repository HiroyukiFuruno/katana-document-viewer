use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{
    ByteRange, EmojiNode, FootnoteReferenceNode, ImageNode, InlineHtmlNode, InlineMathNode,
    InlineSpan, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange, LinkNode, RawSnippet,
    SourceSpan,
};

use super::{SurfaceInlineSpans, SurfaceTextSpan, SurfaceTextStyle};

const EMPTY_ID: &str = "inline-span-node";

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

fn append_kind(spans: &mut Vec<SurfaceTextSpan>, kind: KmmNodeKind, theme: &KdvThemeSnapshot) {
    let inline_node = KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind,
        source: source_span("x"),
        children: Vec::new(),
    };
    SurfaceInlineSpans::append_node_without_fallback(
        spans,
        &inline_node,
        SurfaceTextStyle::default(),
        theme,
    );
}

#[test]
fn append_node_without_fallback_handles_styled_variants() {
    let mut spans = Vec::new();
    let theme = KdvThemeSnapshot::katana_light();
    for kind in [
        KmmNodeKind::Strong(InlineSpan {
            text: "bold".to_string(),
        }),
        KmmNodeKind::Emphasis(InlineSpan {
            text: "italic".to_string(),
        }),
        KmmNodeKind::Strikethrough(InlineSpan {
            text: "strike".to_string(),
        }),
    ] {
        append_kind(&mut spans, kind, &theme);
    }

    assert_eq!(spans[0].text, "bold");
    assert!(spans[0].style.bold);
    assert_eq!(spans[1].text, "italic");
    assert!(spans[1].style.italic);
    assert_eq!(spans[2].text, "strike");
    assert!(spans[2].style.strikethrough);
}

#[test]
fn append_node_without_fallback_handles_link_image_and_footnote_variants() {
    let mut spans = Vec::new();
    let theme = KdvThemeSnapshot::katana_light();
    for kind in [
        KmmNodeKind::Link(LinkNode {
            label: "label".to_string(),
            destination: "/path".to_string(),
            title: None,
            autolink: false,
        }),
        KmmNodeKind::Image(ImageNode {
            alt: "図".to_string(),
            src: "x.png".to_string(),
            title: None,
        }),
        KmmNodeKind::FootnoteReference(FootnoteReferenceNode {
            label: "1".to_string(),
        }),
    ] {
        append_kind(&mut spans, kind, &theme);
    }

    assert_eq!(spans[0].text, "label");
    assert!(spans[0].link_target.as_deref().is_some());
    assert_eq!(spans[1].text, "図");
    assert_eq!(spans[2].text, "[1]");
}

#[test]
fn append_node_without_fallback_handles_html_math_and_emoji_variants() {
    let mut spans = Vec::new();
    let theme = KdvThemeSnapshot::katana_light();
    for kind in [
        KmmNodeKind::InlineHtml(InlineHtmlNode {
            html: "<em>h</em>".to_string(),
        }),
        KmmNodeKind::InlineMath(InlineMathNode {
            expression: r"\frac{1".to_string(),
        }),
        KmmNodeKind::Emoji(EmojiNode {
            value: "✨".to_string(),
            shortcode: None,
        }),
    ] {
        append_kind(&mut spans, kind, &theme);
    }

    assert_eq!(spans[0].text, "h");
    assert!(spans[0].style.italic);
    assert!(!spans[1].text.contains(r"\frac"));
    assert!(!spans[1].text.is_empty());
    assert_eq!(spans[2].text, "✨");
}
