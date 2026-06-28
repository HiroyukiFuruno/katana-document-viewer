use super::{MarkdownSource, PreviewConfig, PreviewError, PreviewOutputFactory};
use crate::{ArtifactFormat, ViewerNodeKind, ViewerNodePlan, ViewerNodePlanner};

const CONTENT_HEIGHT: f32 = 320.0;

#[test]
fn standalone_local_image_paragraph_becomes_image_node() -> Result<(), PreviewError> {
    let plan = plan_for(concat!(
        "Before\n",
        "\n",
        "![diagram](assets/diagram.png)\n",
        "\n",
        "After",
    ))?;

    assert!(has_kind(&plan, |kind| matches!(
        kind,
        ViewerNodeKind::Paragraph
    )));
    assert_eq!(1, image_count(&plan));
    assert!(plan.asset_requests.iter().any(|request| {
        matches!(request.format, ArtifactFormat::Png)
            && plan.nodes.iter().any(|node| {
                matches!(node.kind, ViewerNodeKind::Image)
                    && node.artifact_id == Some(request.artifact_id.clone())
            })
    }));
    Ok(())
}

#[test]
fn inline_image_without_paragraph_boundary_stays_markdown_paragraph() -> Result<(), PreviewError> {
    let plan = plan_for("![diagram](assets/diagram.png)\nText")?;

    assert_eq!(0, image_count(&plan));
    assert!(has_kind(&plan, |kind| matches!(
        kind,
        ViewerNodeKind::Paragraph
    )));
    Ok(())
}

fn plan_for(content: &str) -> Result<ViewerNodePlan, PreviewError> {
    let output = PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: content.to_string(),
            document_id: Some("katana-preview-image-parity.md".to_string()),
        },
        &PreviewConfig::default(),
        CONTENT_HEIGHT,
    )?;
    Ok(ViewerNodePlanner::create(&output.input, 0.0))
}

fn image_count(plan: &ViewerNodePlan) -> usize {
    plan.nodes
        .iter()
        .filter(|node| matches!(node.kind, ViewerNodeKind::Image))
        .count()
}

fn has_kind(plan: &ViewerNodePlan, predicate: impl Fn(&ViewerNodeKind) -> bool) -> bool {
    plan.nodes.iter().any(|node| predicate(&node.kind))
}
