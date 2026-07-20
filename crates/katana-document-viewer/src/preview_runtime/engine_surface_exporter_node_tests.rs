use super::*;
use crate::{MarkdownSource, PreviewOutputFactory};

fn preview_outputs() -> Vec<PreviewOutput> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: "```mermaid\ngraph TD\nA --> B\n```".to_string(),
            document_id: Some("engine-surface-exporter-node.md".to_string()),
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

#[test]
fn push_rendered_diagram_skips_nodes_without_artifact_id() {
    let outputs = preview_outputs();
    assert_eq!(1, outputs.len());
    let mut nodes = diagram_nodes(&outputs[0]);
    assert_eq!(1, nodes.len());
    nodes[0].artifact_id = None;
    let mut rendered = Vec::new();

    PreviewSurfaceExporter::push_rendered_diagram(&outputs[0], &nodes[0], &mut rendered);

    assert!(rendered.is_empty());
}

#[test]
fn push_rendered_diagram_skips_missing_artifact_reference() {
    let outputs = preview_outputs();
    assert_eq!(1, outputs.len());
    let mut nodes = diagram_nodes(&outputs[0]);
    assert_eq!(1, nodes.len());
    nodes[0].artifact_id = Some(crate::ArtifactId("missing-artifact".to_string()));
    let mut rendered = Vec::new();

    PreviewSurfaceExporter::push_rendered_diagram(&outputs[0], &nodes[0], &mut rendered);

    assert!(rendered.is_empty());
}
