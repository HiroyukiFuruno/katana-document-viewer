use crate::{
    Artifact, ArtifactFormat, ArtifactId, SourceSpan, ViewerNode, ViewerNodeKind, ViewerRect,
    ViewerTextSpan,
};
use katana_markdown_model::{ByteRange, LineColumn, LineColumnRange, RawSnippet};

pub(super) fn node_with_artifact(label: &str, artifact_id: ArtifactId, y: f32) -> ViewerNode {
    ViewerNode {
        node_id: crate::KmmNodeId(format!("node-{label}")),
        kind: ViewerNodeKind::Diagram {
            kind: crate::ViewerDiagramKind::Mermaid,
        },
        source: source_span(label),
        text: label.to_string(),
        spans: vec![ViewerTextSpan::plain(label)],
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: ViewerRect {
            x: 24.0,
            y,
            width: 320.0,
            height: 180.0,
        },
        artifact_id: Some(artifact_id),
    }
}

pub(super) fn node_without_artifact(label: &str, y: f32) -> ViewerNode {
    ViewerNode {
        node_id: crate::KmmNodeId(format!("node-no-artifact-{label}")),
        kind: ViewerNodeKind::Diagram {
            kind: crate::ViewerDiagramKind::Mermaid,
        },
        source: source_span(label),
        text: label.to_string(),
        spans: vec![ViewerTextSpan::plain(label)],
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: ViewerRect {
            x: 24.0,
            y,
            width: 320.0,
            height: 180.0,
        },
        artifact_id: None,
    }
}

pub(super) fn text_artifact(id: ArtifactId, format: ArtifactFormat, bytes: &[u8]) -> Artifact {
    crate::ArtifactFactory::image_asset_with_id(
        id,
        format,
        crate::DocumentId("document".to_string()),
        crate::SourceRevision("rev".to_string()),
        crate::ArtifactBytes {
            bytes: bytes.to_vec(),
        },
        "test",
        crate::ArtifactDiagnostics {
            entries: Vec::new(),
        },
    )
}

pub(super) fn source_span(text: &str) -> SourceSpan {
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
