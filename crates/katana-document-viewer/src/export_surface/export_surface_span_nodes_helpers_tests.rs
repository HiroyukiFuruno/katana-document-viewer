use super::SurfaceTextStyle;
use super::{
    append_inline_html, append_inline_math, append_link, append_style_node, append_unknown_node,
    html_style, push, push_plain,
};
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{
    ByteRange, HeadingNode, InlineSpan, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, RawSnippet, SourceSpan, TextSpan,
};

const EMPTY_ID: &str = "inline-span-helper";
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
fn text_node(text: &str, source_text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind: KmmNodeKind::Text(TextSpan {
            text: text.to_string(),
        }),
        source: source_span(source_text),
        children: Vec::new(),
    }
}

fn node(kind: KmmNodeKind, children: Vec<KmmNode>, source_text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind,
        source: source_span(source_text),
        children,
    }
}

#[test]
fn append_unknown_node_reuses_children_before_raw_fallback() {
    let mut spans = Vec::new();
    append_unknown_node(
        &mut spans,
        &node(
            KmmNodeKind::Heading(HeadingNode {
                level: 1,
                text: String::new(),
            }),
            vec![text_node("inner", "")],
            "",
        ),
        SurfaceTextStyle::default(),
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].text, "inner");
    assert!(spans[0].is_plain());
}

#[test]
fn append_unknown_node_uses_raw_source_when_no_childs() {
    let mut spans = Vec::new();
    append_unknown_node(
        &mut spans,
        &node(
            KmmNodeKind::Heading(HeadingNode {
                level: 1,
                text: String::new(),
            }),
            Vec::new(),
            "A &lt; B",
        ),
        SurfaceTextStyle::default(),
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].text, "A < B");
    assert!(spans[0].is_plain());
}

#[test]
fn append_style_node_pushes_text_when_no_children() {
    let mut spans = Vec::new();
    append_style_node(
        &mut spans,
        &node(
            KmmNodeKind::Emphasis(InlineSpan {
                text: "styled".to_string(),
            }),
            Vec::new(),
            "",
        ),
        "styled",
        SurfaceTextStyle::default().bold(),
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].text, "styled");
    assert!(spans[0].style.bold);
}

#[test]
fn append_style_node_walks_children_when_present() {
    let mut spans = Vec::new();
    append_style_node(
        &mut spans,
        &node(
            KmmNodeKind::Emphasis(InlineSpan {
                text: "ignored".to_string(),
            }),
            vec![text_node("from child", "")],
            "",
        ),
        "ignored",
        SurfaceTextStyle::default().italic(),
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].text, "from child");
    assert!(spans[0].style.italic);
}

#[test]
fn append_link_skips_empty_text() {
    let mut spans = Vec::new();
    append_link(&mut spans, "", "target", SurfaceTextStyle::default());

    assert!(spans.is_empty());
}

#[test]
fn append_inline_html_applies_inline_code_style() {
    let mut spans = Vec::new();
    append_inline_html(&mut spans, "<code>c</code>", SurfaceTextStyle::default());

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].text, "c");
    assert!(spans[0].style.monospace);
}

#[test]
fn inline_text_style_switches_by_html_tag() {
    assert!(html_style("<code>x</code>", SurfaceTextStyle::default()).inline_code);
    assert!(html_style("<strong>x</strong>", SurfaceTextStyle::default()).bold);
    assert!(html_style("<b>x</b>", SurfaceTextStyle::default()).bold);
    assert!(html_style("<em>x</em>", SurfaceTextStyle::default()).italic);
    assert!(html_style("<i>x</i>", SurfaceTextStyle::default()).italic);
    assert!(html_style("<u>x</u>", SurfaceTextStyle::default()).underline);
    assert!(html_style("<mark>x</mark>", SurfaceTextStyle::default()).highlight);
    assert!(html_style("<s>x</s>", SurfaceTextStyle::default()).strikethrough);
    assert!(html_style("<del>x</del>", SurfaceTextStyle::default()).strikethrough);
    assert!(html_style("plain", SurfaceTextStyle::default()) == SurfaceTextStyle::default());
}

#[test]
fn push_plain_decodes_entities() {
    let mut spans = Vec::new();
    push_plain(&mut spans, "A &lt; B&amp;C", SurfaceTextStyle::default());

    assert_eq!(spans[0].text, "A < B&C");
}

#[test]
fn push_ignores_empty_text() {
    let mut spans = Vec::new();
    push(&mut spans, String::new(), SurfaceTextStyle::default());

    assert!(spans.is_empty());
}

#[test]
fn append_inline_math_falls_back_to_text_when_math_svg_is_unavailable() {
    let mut spans = Vec::new();
    append_inline_math(
        &mut spans,
        r"\frac{1",
        SurfaceTextStyle::default(),
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(spans.len(), 1);
    assert!(spans[0].inline_image.is_none() || spans[0].text.contains("frac"));
}
