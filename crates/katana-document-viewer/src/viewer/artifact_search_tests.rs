use super::ViewerArtifactSearchResolver;
use crate::{
    Artifact, ArtifactBytes, ArtifactDiagnostics, ArtifactFactory, ArtifactFormat, ArtifactId,
    ByteRange, DocumentId, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceRevision,
    SourceSpan, ViewerNode, ViewerNodeKind, ViewerRect, ViewerTextSpan,
};

#[test]
fn svg_artifact_text_resolves_to_matching_viewer_node_rect() {
    let artifact_id = ArtifactId("diagram-svg".to_string());
    let node = node_with_artifact("diagram", artifact_id.clone(), 128.0);
    let artifact = text_artifact(
        artifact_id.clone(),
        ArtifactFormat::Svg,
        br#"<svg><text>Diagram Needle</text></svg>"#,
    );

    let targets = ViewerArtifactSearchResolver::resolve_targets("needle", &[node], &[artifact]);

    assert_eq!(1, targets.len());
    assert_eq!(128.0, targets[0].rect.y);
    assert_eq!("Needle", targets[0].matched.text);
    assert_eq!(Some(artifact_id), targets[0].matched.artifact_id);
}

#[test]
fn artifact_text_ignores_artifacts_without_matching_node() {
    let node = node_with_artifact("diagram", ArtifactId("node-artifact".to_string()), 128.0);
    let artifact = text_artifact(
        ArtifactId("other-artifact".to_string()),
        ArtifactFormat::Svg,
        br#"<svg><text>Diagram Needle</text></svg>"#,
    );

    let targets = ViewerArtifactSearchResolver::resolve_targets("needle", &[node], &[artifact]);

    assert!(targets.is_empty());
}

#[test]
fn non_text_artifact_formats_do_not_create_search_targets() {
    let artifact_id = ArtifactId("diagram-png".to_string());
    let node = node_with_artifact("diagram", artifact_id.clone(), 128.0);
    let artifact = text_artifact(artifact_id, ArtifactFormat::Png, b"needle");

    let targets = ViewerArtifactSearchResolver::resolve_targets("needle", &[node], &[artifact]);

    assert!(targets.is_empty());
}

#[test]
fn explicit_artifact_text_extraction_searches_non_text_formats() {
    let artifact_id = ArtifactId("diagram-png".to_string());
    let node = node_with_artifact("diagram", artifact_id.clone(), 128.0);
    let artifact = text_artifact(artifact_id.clone(), ArtifactFormat::Png, b"raster bytes")
        .with_text_extraction("Raster Needle");

    let targets = ViewerArtifactSearchResolver::resolve_targets("needle", &[node], &[artifact]);

    assert_eq!(1, targets.len());
    assert_eq!(128.0, targets[0].rect.y);
    assert_eq!("Needle", targets[0].matched.text);
    assert_eq!(Some(artifact_id), targets[0].matched.artifact_id);
}

fn node_with_artifact(label: &str, artifact_id: ArtifactId, y: f32) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId(format!("node-{label}")),
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

fn text_artifact(id: ArtifactId, format: ArtifactFormat, bytes: &[u8]) -> Artifact {
    ArtifactFactory::image_asset_with_id(
        id,
        format,
        DocumentId("document".to_string()),
        SourceRevision("rev".to_string()),
        ArtifactBytes {
            bytes: bytes.to_vec(),
        },
        "test",
        ArtifactDiagnostics {
            entries: Vec::new(),
        },
    )
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
