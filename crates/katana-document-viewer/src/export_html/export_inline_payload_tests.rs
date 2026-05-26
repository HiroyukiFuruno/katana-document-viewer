use super::*;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{
    ByteRange, FootnoteDefinitionNode, FootnoteReferenceNode, InlineSpan, KmmNode, KmmNodeId,
    KmmNodeKind, LineColumn, LineColumnRange, LinkNode, RawSnippet, SourceSpan, TextSpan,
};
fn definition_node() -> KmmNode {
    KmmNode {
        id: KmmNodeId("def".to_string()),
        kind: KmmNodeKind::FootnoteDefinition(FootnoteDefinitionNode {
            label: "1".to_string(),
            text: "body".to_string(),
        }),
        source: source_span("body"),
        children: Vec::new(),
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

#[path = "export_inline_payload_basic_tests.rs"]
mod basic_tests;
#[path = "export_inline_payload_footnote_tests.rs"]
mod footnote_tests;
#[path = "export_inline_payload_node_tests.rs"]
mod node_tests;
