use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{
    ByteRange, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange, RawSnippet,
    SourceSpan, TextSpan,
};

use super::{SurfaceInlineSpans, SurfaceTextStyle, append_node_source_text};

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

fn node(kind: KmmNodeKind, children: Vec<KmmNode>, source_text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind,
        source: source_span(source_text),
        children,
    }
}

#[test]
fn from_node_uses_render_path_for_text_kind() {
    let spans = SurfaceInlineSpans::from_node(
        &node(
            KmmNodeKind::Text(TextSpan {
                text: "Hello &amp; world".to_string(),
            }),
            Vec::new(),
            "Hello &amp; world",
        ),
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].text, "Hello & world");
    assert!(spans[0].is_plain());
}

#[test]
fn from_nodes_with_plain_markdown_source_reprocesses_as_markdown_when_needed() {
    let spans = SurfaceInlineSpans::from_nodes(
        &[node(
            KmmNodeKind::Text(TextSpan {
                text: "**bolded**".to_string(),
            }),
            Vec::new(),
            "**bolded**",
        )],
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].text, "bolded");
    assert!(spans[0].style.bold);
}

#[test]
fn from_nodes_source_text_walks_child_nodes_for_empty_raw_node() {
    let mut output = String::new();
    let parent = node(
        KmmNodeKind::BlockQuote,
        vec![node(
            KmmNodeKind::Text(TextSpan {
                text: "inner".to_string(),
            }),
            Vec::new(),
            "leaf",
        )],
        "",
    );

    append_node_source_text(&mut output, &parent);

    assert_eq!(output, "leaf");
}

#[test]
fn from_nodes_without_fallback_recurses_unknown_children() {
    let mut spans = Vec::new();
    let node = node(
        KmmNodeKind::BlockQuote,
        vec![node(
            KmmNodeKind::Text(TextSpan {
                text: "child".to_string(),
            }),
            Vec::new(),
            "",
        )],
        "",
    );

    SurfaceInlineSpans::append_node_without_fallback(
        &mut spans,
        &node,
        SurfaceTextStyle::default(),
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].text, "child");
    assert!(spans[0].is_plain());
}

#[test]
fn from_markdown_without_nodes_returns_plain_entity_decoded_text() {
    let spans = SurfaceInlineSpans::from_markdown("A &amp; B", &KdvThemeSnapshot::katana_light());

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].text, "A & B");
    assert!(spans[0].is_plain());
}

#[test]
fn from_markdown_splits_raw_emoji_for_os_emoji_rendering() {
    let spans =
        SurfaceInlineSpans::from_markdown("Emoji: 🦀 text ⚠️", &KdvThemeSnapshot::katana_light());

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
fn from_markdown_empty_fragment_returns_empty_plain_span() {
    let spans = SurfaceInlineSpans::from_markdown("", &KdvThemeSnapshot::katana_light());

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].text, "");
    assert!(spans[0].is_plain());
}
