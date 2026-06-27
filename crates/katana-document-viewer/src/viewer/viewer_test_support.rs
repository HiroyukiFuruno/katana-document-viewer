use super::{ViewerRectFactory, ViewerTarget};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};

const TARGET_RECT_X: f32 = 10.0;
const TARGET_RECT_Y: f32 = 10.0;
const TARGET_RECT_WIDTH: f32 = 40.0;
const TARGET_RECT_HEIGHT: f32 = 20.0;

pub(super) fn sample_target() -> ViewerTarget {
    target_at("node-diagram-1", TARGET_RECT_X, TARGET_RECT_Y)
}

pub(super) fn target_at(node_id: &str, x: f32, y: f32) -> ViewerTarget {
    ViewerTarget {
        node_id: KmmNodeId(node_id.to_string()),
        source: sample_span("```mermaid"),
        artifact_id: crate::ArtifactId(format!("artifact-{node_id}")),
        rect: ViewerRectFactory::from_origin_size(x, y, TARGET_RECT_WIDTH, TARGET_RECT_HEIGHT),
    }
}

fn sample_span(text: &str) -> SourceSpan {
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
