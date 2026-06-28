use super::KucNodeFactory;
use katana_document_viewer::{ViewerInteractionConfig, ViewerNode, ViewerNodeKind, ViewerRect};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};

const NODE_WIDTH: f32 = 120.0;
const NODE_HEIGHT: f32 = 32.0;
const MAX_MEDIA_WIDTH: u32 = 120;

#[test]
fn text_node_selection_follows_interaction_config() {
    let selectable = KucNodeFactory::new(&[], MAX_MEDIA_WIDTH).interaction(interaction(true));
    let disabled = KucNodeFactory::new(&[], MAX_MEDIA_WIDTH).interaction(interaction(false));

    assert!(
        selectable
            .viewer_node(&viewer_node("select me"))
            .props()
            .common
            .selectable
    );
    assert!(
        !disabled
            .viewer_node(&viewer_node("select me"))
            .props()
            .common
            .selectable
    );
}

fn interaction(selection_enabled: bool) -> ViewerInteractionConfig {
    ViewerInteractionConfig {
        hover_highlight_enabled: false,
        selection_enabled,
        image_controls_enabled: false,
        diagram_controls_enabled: false,
        code_controls_enabled: false,
    }
}

fn viewer_node(text: &str) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId("node".to_string()),
        kind: ViewerNodeKind::Paragraph,
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
