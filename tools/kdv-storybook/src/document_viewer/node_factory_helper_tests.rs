use crate::document_viewer::node_labels::KucNodeLabels;
use katana_document_viewer::{ViewerNode, ViewerNodeKind, ViewerRect};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};

const NODE_WIDTH: f32 = 120.0;
const NODE_HEIGHT: f32 = 32.0;

#[test]
fn helper_labels_cover_media_defaults() {
    let math = viewer_node(ViewerNodeKind::Math, "x");
    let paragraph = viewer_node(ViewerNodeKind::Paragraph, "text");

    assert_eq!("math", KucNodeLabels::media_label(&math));
    assert_eq!("media", KucNodeLabels::media_label(&paragraph));
}

fn viewer_node(kind: ViewerNodeKind, text: &str) -> ViewerNode {
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
