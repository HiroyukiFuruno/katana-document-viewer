use katana_document_viewer::{ViewerNode, ViewerNodeKind, ViewerRect};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};
use katana_ui_core::render_model::UiNode;

const NODE_WIDTH: f32 = 120.0;
const NODE_HEIGHT: f32 = 32.0;

pub(super) fn has_style_class(node: &UiNode, expected: &str) -> bool {
    node.props()
        .style_classes
        .iter()
        .any(|class| class == expected)
}

pub(super) fn viewer_node(kind: ViewerNodeKind, text: &str) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId("node".to_string()),
        kind,
        source: source(text),
        text: text.to_string(),
        spans: Vec::new(),
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: NODE_WIDTH,
            height: NODE_HEIGHT,
        },
        artifact_id: None,
    }
}

fn source(raw: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: raw.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: raw.len() + 1,
            },
        },
        raw: RawSnippet {
            text: raw.to_string(),
        },
    }
}
