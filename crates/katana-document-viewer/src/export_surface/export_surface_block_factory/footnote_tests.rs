use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{
    ByteRange, FootnoteDefinitionNode, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, RawSnippet, SourceSpan, TextSpan,
};

use super::*;

#[test]
fn append_footnote_definition_adds_reference_link() {
    let mut blocks = Vec::new();
    let node = footnote_node("1", "note text", Vec::new());

    SurfaceBlockFactory::append_footnote_definition(
        &mut blocks,
        &node,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert!(blocks[0].text_for_tests().contains("1."));
    assert!(blocks[0].text_for_tests().contains("↩"));
}

#[test]
fn footnote_definition_uses_children_body_when_present() {
    let mut blocks = Vec::new();
    let node = footnote_node(
        "2",
        "note body",
        vec![KmmNode {
            id: KmmNodeId("child".to_string()),
            kind: KmmNodeKind::Text(TextSpan {
                text: "child body".to_string(),
            }),
            source: source_span("child body"),
            children: Vec::new(),
        }],
    );

    SurfaceBlockFactory::append_footnote_definition(
        &mut blocks,
        &node,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert!(blocks[0].text_for_tests().contains("2."));
    assert!(blocks[0].text_for_tests().contains("child body"));
}

fn footnote_node(label: &str, text: &str, children: Vec<KmmNode>) -> KmmNode {
    KmmNode {
        id: KmmNodeId(format!("footnote-{label}")),
        kind: KmmNodeKind::FootnoteDefinition(FootnoteDefinitionNode {
            label: label.to_string(),
            text: text.to_string(),
        }),
        source: source_span(text),
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
