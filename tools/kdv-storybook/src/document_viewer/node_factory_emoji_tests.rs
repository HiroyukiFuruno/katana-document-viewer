use super::KucNodeFactory;
use katana_document_viewer::{
    ViewerNode, ViewerNodeKind, ViewerRect, ViewerTextSpan, ViewerTextStyle,
};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};

const NODE_WIDTH: f32 = 120.0;
const NODE_HEIGHT: f32 = 32.0;

#[test]
fn text_node_preserves_emoji_span_render_contract() {
    let factory = KucNodeFactory::new(&[], 120);
    let mut node = viewer_node("Status 🙂");
    node.spans = vec![ViewerTextSpan::styled(
        "🙂",
        ViewerTextStyle::default().emoji(),
    )];

    let ui_node = factory.viewer_node(&node);

    assert_eq!("🙂", ui_node.props().text.spans[0].text);
    assert!(ui_node.props().text.spans[0].style.emoji);
}

fn viewer_node(text: &str) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId("emoji-node".to_string()),
        kind: ViewerNodeKind::Paragraph,
        source: source_span(text),
        text: text.to_string(),
        spans: Vec::new(),
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: viewer_rect(),
        artifact_id: None,
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

fn viewer_rect() -> ViewerRect {
    ViewerRect {
        x: 0.0,
        y: 0.0,
        width: NODE_WIDTH,
        height: NODE_HEIGHT,
    }
}
