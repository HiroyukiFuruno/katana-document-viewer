use super::{MarkdownSource, PreviewConfig, PreviewError, PreviewOutputFactory};
use crate::{
    ArtifactFormat, DocumentKind, ViewerAssetLoadPriority, ViewerDiagramKind, ViewerNodeKind,
    ViewerNodePlanner, ViewerViewport,
};
use katana_markdown_model::{CodeBlockRole, DiagramKind, KmmNode, KmmNodeKind};

const CONTENT_HEIGHT: f32 = 120.0;

#[test]
fn direct_drawio_source_becomes_drawio_diagram_node() -> Result<(), PreviewError> {
    let output = direct_output("<mxfile><diagram /></mxfile>", "fixture.drawio")?;
    let node = first_document_node(&output)?;

    assert_eq!(DocumentKind::Diagram, output.input.snapshot.kind);
    assert!(matches!(
        &node.kind,
        KmmNodeKind::CodeBlock(CodeBlockRole::Diagram {
            kind: DiagramKind::DrawIo
        })
    ));
    assert!(node.source.raw.text.contains("<mxfile>"));
    Ok(())
}

#[test]
fn direct_drowio_source_keeps_katana_drawio_typo_compatibility() -> Result<(), PreviewError> {
    let output = direct_output("<mxfile><diagram /></mxfile>", "fixture.drowio")?;
    let node = first_document_node(&output)?;

    assert_eq!(DocumentKind::Diagram, output.input.snapshot.kind);
    assert!(matches!(
        &node.kind,
        KmmNodeKind::CodeBlock(CodeBlockRole::Diagram {
            kind: DiagramKind::DrawIo
        })
    ));
    Ok(())
}

#[test]
fn direct_drawio_source_requests_visible_svg_asset_for_viewer() -> Result<(), PreviewError> {
    let output = direct_output_with_viewport(
        "<mxfile><diagram /></mxfile>",
        "fixture.drawio",
        ViewerViewport {
            width: 640.0,
            height: 480.0,
        },
    )?;
    let plan = ViewerNodePlanner::create(&output.input, 0.0);

    assert!(matches!(
        plan.nodes[0].kind,
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::DrawIo
        }
    ));
    assert_eq!(
        Some(plan.asset_requests[0].artifact_id.clone()),
        plan.nodes[0].artifact_id
    );
    assert_eq!(ArtifactFormat::Svg, plan.asset_requests[0].format);
    assert_eq!(
        ViewerAssetLoadPriority::Visible,
        plan.asset_requests[0].priority
    );
    Ok(())
}

#[test]
fn direct_plantuml_source_becomes_plantuml_diagram_node() -> Result<(), PreviewError> {
    let output = direct_output("@startuml\nA -> B\n@enduml", "fixture.puml")?;
    let node = first_document_node(&output)?;

    assert_eq!(DocumentKind::Diagram, output.input.snapshot.kind);
    assert!(matches!(
        &node.kind,
        KmmNodeKind::CodeBlock(CodeBlockRole::Diagram {
            kind: DiagramKind::PlantUml
        })
    ));
    assert!(node.source.raw.text.contains("```plantuml"));
    Ok(())
}

#[test]
fn direct_plantuml_source_requests_visible_svg_asset_for_viewer() -> Result<(), PreviewError> {
    let output = direct_output_with_viewport(
        "@startuml\nA -> B\n@enduml",
        "fixture.plantuml",
        ViewerViewport {
            width: 640.0,
            height: 480.0,
        },
    )?;
    let plan = ViewerNodePlanner::create(&output.input, 0.0);

    assert!(matches!(
        plan.nodes[0].kind,
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::PlantUml
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

fn direct_output_with_viewport(
    content: &str,
    document_id: &str,
    viewport: ViewerViewport,
) -> Result<super::PreviewOutput, PreviewError> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: content.to_string(),
            document_id: Some(document_id.to_string()),
        },
        &PreviewConfig {
            viewport,
            ..PreviewConfig::default()
        },
        CONTENT_HEIGHT,
    )
}

fn first_document_node(output: &super::PreviewOutput) -> Result<&KmmNode, PreviewError> {
    output
        .input
        .snapshot
        .document
        .nodes
        .first()
        .ok_or_else(|| PreviewError::Render("document node missing".to_string()))
}
