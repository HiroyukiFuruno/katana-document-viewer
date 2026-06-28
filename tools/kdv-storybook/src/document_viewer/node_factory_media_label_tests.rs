use super::KucNodeFactory;
use crate::document_viewer::node_labels::KucNodeLabels;
use katana_document_viewer::{
    ViewerDiagramKind, ViewerInteractionConfig, ViewerNode, ViewerNodeKind, ViewerRect,
};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};

const STORYBOOK_MEDIA_MAX_WIDTH: u32 = 120;
const VIEWER_NODE_WIDTH: f32 = 120.0;
const VIEWER_NODE_HEIGHT: f32 = 32.0;

#[test]
fn non_media_node_disables_controls_and_uses_media_label() {
    let factory = KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH);
    let node = viewer_node(ViewerNodeKind::Paragraph, "text", "paragraph");

    assert!(!factory.media_controls_enabled(&node));
    assert_eq!("media", KucNodeLabels::media_label(&node));
}

#[test]
fn math_node_does_not_use_diagram_controls() {
    let factory =
        KucNodeFactory::new(&[], STORYBOOK_MEDIA_MAX_WIDTH).interaction(ViewerInteractionConfig {
            hover_highlight_enabled: true,
            selection_enabled: true,
            image_controls_enabled: true,
            diagram_controls_enabled: true,
            code_controls_enabled: true,
        });
    let node = viewer_node(ViewerNodeKind::Math, "x", "math");

    assert!(!factory.media_controls_enabled(&node));
    assert_eq!("math", KucNodeLabels::media_label(&node));
}

#[test]
fn diagram_node_uses_diagram_media_label() {
    let node = viewer_node(
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::DrawIo,
        },
        "diagram",
        "diagram",
    );

    assert_eq!("diagram:DrawIo", KucNodeLabels::media_label(&node));
}

fn viewer_node(kind: ViewerNodeKind, text: &str, node_id: &str) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId(node_id.to_string()),
        kind,
        source: source(text),
        text: text.to_string(),
        spans: Vec::new(),
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: VIEWER_NODE_WIDTH,
            height: VIEWER_NODE_HEIGHT,
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
