use super::*;
use crate::{
    ArtifactDiagnostic, ArtifactDiagnostics, ArtifactFactory, ArtifactId, DiagnosticSeverity,
    DocumentId, MarkdownSource, PreviewOutputFactory, SourceRevision,
};

fn preview_outputs() -> Vec<PreviewOutput> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: "```mermaid\ngraph TD\nA --> B\n```".to_string(),
            document_id: Some("engine-surface-exporter-diagrams.md".to_string()),
        },
        &PreviewConfig {
            viewport: crate::ViewerViewport {
                width: 640.0,
                height: 480.0,
            },
            ..PreviewConfig::default()
        },
        120.0,
    )
    .into_iter()
    .collect()
}

fn diagram_nodes(output: &PreviewOutput) -> Vec<ViewerNode> {
    ViewerNodePlanner::create(&output.input, output.scroll_offset)
        .nodes
        .into_iter()
        .filter(|node| matches!(node.kind, ViewerNodeKind::Diagram { .. }))
        .collect()
}

fn artifact(
    artifact_id: &ArtifactId,
    format: ArtifactFormat,
    bytes: &[u8],
    diagnostics: ArtifactDiagnostics,
) -> Artifact {
    ArtifactFactory::image_asset_with_id(
        artifact_id.clone(),
        format,
        DocumentId("doc".to_string()),
        SourceRevision("revision".to_string()),
        crate::ArtifactBytes {
            bytes: bytes.to_vec(),
        },
        "test",
        diagnostics,
    )
}

fn no_diagnostics() -> ArtifactDiagnostics {
    ArtifactDiagnostics {
        entries: Vec::new(),
    }
}

fn fixture() -> (Vec<PreviewOutput>, Vec<ViewerNode>, ArtifactId) {
    let outputs = preview_outputs();
    assert_eq!(1, outputs.len());
    let nodes = diagram_nodes(&outputs[0]);
    assert_eq!(1, nodes.len());
    let artifact_ids = nodes[0].artifact_id.iter().cloned().collect::<Vec<_>>();
    assert_eq!(1, artifact_ids.len());
    (outputs, nodes, artifact_ids[0].clone())
}

#[test]
fn rendered_diagrams_returns_empty_for_no_artifacts() {
    let outputs = PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: "plain text".to_string(),
            document_id: Some("engine-surface-exporter-no-artifacts.md".to_string()),
        },
        &PreviewConfig::default(),
        120.0,
    )
    .into_iter()
    .collect::<Vec<_>>();
    assert_eq!(1, outputs.len());
    assert!(PreviewSurfaceExporter::rendered_diagrams(&outputs[0]).is_empty());
}

#[test]
fn rendered_diagram_rejects_non_svg_format() {
    let (_outputs, nodes, artifact_id) = fixture();
    let value = artifact(
        &artifact_id,
        ArtifactFormat::Png,
        b"not-svg-data",
        no_diagnostics(),
    );
    assert!(PreviewSurfaceExporter::rendered_diagram(&nodes[0], &value).is_none());
}

#[test]
fn rendered_diagram_rejects_diagnostics() {
    let (_outputs, nodes, artifact_id) = fixture();
    let value = artifact(
        &artifact_id,
        ArtifactFormat::Svg,
        b"<svg/>",
        ArtifactDiagnostics {
            entries: vec![ArtifactDiagnostic {
                severity: DiagnosticSeverity::Error,
                code: "test".to_string(),
                message: "rendered artifact has diagnostics".to_string(),
            }],
        },
    );
    assert!(PreviewSurfaceExporter::rendered_diagram(&nodes[0], &value).is_none());
}

#[test]
fn rendered_diagram_rejects_non_svg_payload() {
    let (_outputs, nodes, artifact_id) = fixture();
    let value = artifact(
        &artifact_id,
        ArtifactFormat::Svg,
        b"not starting with svg",
        no_diagnostics(),
    );
    assert!(PreviewSurfaceExporter::rendered_diagram(&nodes[0], &value).is_none());
}

#[test]
fn rendered_diagram_includes_valid_svg_payload() {
    let (_outputs, nodes, artifact_id) = fixture();
    let value = artifact(
        &artifact_id,
        ArtifactFormat::Svg,
        b"<svg><g/></svg>",
        no_diagnostics(),
    );
    let rendered = PreviewSurfaceExporter::rendered_diagram(&nodes[0], &value);
    assert!(matches!(
        rendered,
        Some(diagram) if diagram.kind == "mermaid" && diagram.svg.starts_with("<svg")
    ));
}

#[test]
fn diagram_kind_covers_all_kind_variants() {
    for (kind, expected) in [
        (ViewerDiagramKind::Mermaid, "mermaid"),
        (ViewerDiagramKind::DrawIo, "drawio"),
        (ViewerDiagramKind::PlantUml, "plantuml"),
    ] {
        assert_eq!(
            Some(expected),
            PreviewSurfaceExporter::diagram_kind(&ViewerNodeKind::Diagram { kind })
        );
    }
}

#[test]
fn diagram_kind_rejects_non_diagram_nodes() {
    assert_eq!(
        None,
        PreviewSurfaceExporter::diagram_kind(&ViewerNodeKind::Paragraph)
    );
}
