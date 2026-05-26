use super::*;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{
    ByteRange, HeadingNode, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange,
    RawSnippet, SourceSpan, TextSpan,
};

#[test]
fn append_clamps_level_below_range() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let node = node(
        KmmNodeKind::Heading(HeadingNode {
            level: 0,
            text: "# heading".to_string(),
        }),
        "heading",
        Vec::new(),
    );
    HeadingHtmlWriter::append(&mut html, &node, 0, "fallback", &theme);
    assert!(html.starts_with("<h1>"));
}

#[test]
fn append_clamps_level_above_range() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let node = node(
        KmmNodeKind::Heading(HeadingNode {
            level: 9,
            text: "## heading".to_string(),
        }),
        "heading",
        Vec::new(),
    );
    HeadingHtmlWriter::append(&mut html, &node, 9, "fallback", &theme);
    assert!(html.starts_with("<h6>"));
}

#[test]
fn append_strips_first_child_marker() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let heading_node = node(
        KmmNodeKind::Heading(HeadingNode {
            level: 2,
            text: "# fallback".to_string(),
        }),
        "## heading",
        vec![node(
            KmmNodeKind::Text(TextSpan {
                text: "## heading".to_string(),
            }),
            "## heading",
            Vec::new(),
        )],
    );
    HeadingHtmlWriter::append(&mut html, &heading_node, 2, "fallback", &theme);
    assert_eq!(html, "<h2>heading</h2>\n");
}

#[test]
fn append_uses_raw_text_when_no_children() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let heading_node = node(
        KmmNodeKind::Heading(HeadingNode {
            level: 2,
            text: "raw".to_string(),
        }),
        "Raw",
        Vec::new(),
    );
    HeadingHtmlWriter::append(&mut html, &heading_node, 2, "Raw fallback", &theme);
    assert!(html.contains("Raw fallback"));
}

#[test]
fn append_children_without_marker_uses_child_when_not_marker() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let heading_node = node(
        KmmNodeKind::Heading(katana_markdown_model::HeadingNode {
            level: 2,
            text: "x".to_string(),
        }),
        "## heading",
        vec![node(KmmNodeKind::BlockQuote, "not marker", Vec::new())],
    );

    let wrote = HeadingHtmlWriter::append_children_without_marker(&mut html, &heading_node, &theme);

    assert!(wrote);
    assert!(html.contains("not marker"));
}

#[test]
fn strip_heading_marker_ignores_non_text_node() {
    let heading = KmmNode {
        id: KmmNodeId("heading".to_string()),
        kind: KmmNodeKind::Heading(katana_markdown_model::HeadingNode {
            level: 2,
            text: "heading".to_string(),
        }),
        source: source_span("heading"),
        children: Vec::new(),
    };
    assert!(HeadingHtmlWriter::try_strip_heading_marker(&heading).is_none());
}

#[test]
fn strip_heading_marker_returns_none_when_hash_count_is_invalid() {
    assert!(HeadingHtmlWriter::strip_heading_marker("######## title").is_none());
    assert!(HeadingHtmlWriter::strip_heading_marker("no marker").is_none());
}

fn node(kind: KmmNodeKind, raw: &str, children: Vec<KmmNode>) -> KmmNode {
    KmmNode {
        id: KmmNodeId("heading".to_string()),
        kind,
        source: source_span(raw),
        children,
    }
}

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
