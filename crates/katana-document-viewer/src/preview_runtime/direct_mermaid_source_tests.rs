use super::{MarkdownSource, PreviewConfig, PreviewError, PreviewOutputFactory};
use crate::{
    ArtifactFormat, DocumentKind, ViewerAssetLoadPriority, ViewerDiagramKind, ViewerNodeKind,
    ViewerNodePlanner, ViewerViewport,
};
use katana_markdown_model::{CodeBlockRole, DiagramKind, KmmNodeKind};

const CONTENT_HEIGHT: f32 = 120.0;

#[test]
fn direct_mmd_source_becomes_mermaid_diagram_node() -> Result<(), PreviewError> {
    let output = direct_output("graph TD\nA --> B", "fixture.mmd")?;
    let node = output
        .input
        .snapshot
        .document
        .nodes
        .first()
        .ok_or_else(|| PreviewError::Render("document node missing".to_string()))?;

    assert_eq!(DocumentKind::Diagram, output.input.snapshot.kind);
    assert!(matches!(
        &node.kind,
        KmmNodeKind::CodeBlock(CodeBlockRole::Diagram {
            kind: DiagramKind::Mermaid
        })
    ));
    assert!(node.source.raw.text.contains("```mermaid"));
    assert!(node.source.raw.text.contains("A --> B"));
    Ok(())
}

#[test]
fn direct_mermaid_source_requests_visible_svg_asset_for_viewer() -> Result<(), PreviewError> {
    let output = PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: "sequenceDiagram\nA->>B: ok".to_string(),
            document_id: Some("fixture.mermaid".to_string()),
        },
        &PreviewConfig {
            viewport: ViewerViewport {
                width: 640.0,
                height: 480.0,
            },
            ..PreviewConfig::default()
        },
        CONTENT_HEIGHT,
    )?;
    let plan = ViewerNodePlanner::create(&output.input, 0.0);

    assert!(matches!(
        plan.nodes[0].kind,
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::Mermaid
        }
    ));
    assert_eq!(ArtifactFormat::Svg, plan.asset_requests[0].format);
    assert_eq!(
        ViewerAssetLoadPriority::Visible,
        plan.asset_requests[0].priority
    );
    Ok(())
}

fn direct_output(content: &str, document_id: &str) -> Result<super::PreviewOutput, PreviewError> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: content.to_string(),
            document_id: Some(document_id.to_string()),
        },
        &PreviewConfig::default(),
        CONTENT_HEIGHT,
    )
}
