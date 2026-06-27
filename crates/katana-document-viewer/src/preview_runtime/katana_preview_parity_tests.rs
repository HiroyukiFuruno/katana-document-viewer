use super::{MarkdownSource, PreviewConfig, PreviewError, PreviewOutputFactory};
use crate::{ViewerDiagramKind, ViewerNodeKind, ViewerNodePlan, ViewerNodePlanner};

const CONTENT_HEIGHT: f32 = 320.0;

#[test]
fn nested_mermaid_inside_markdown_fence_stays_code() -> Result<(), PreviewError> {
    let plan = plan_for(concat!(
        "Before\n",
        "```markdown\n",
        "```mermaid\n",
        "graph TD\n",
        "    A --> B\n",
        "```\n",
        "```\n",
        "After",
    ))?;

    assert!(!has_diagram(&plan, ViewerDiagramKind::Mermaid));
    assert!(has_code(&plan));
    Ok(())
}

#[test]
fn empty_mermaid_fences_stay_code_nodes() -> Result<(), PreviewError> {
    let plan = plan_for(concat!(
        "Before\n",
        "```mermaid\n",
        "  \n",
        "```\n",
        "\n",
        "~~~mermaid\n",
        "  \n",
        "~~~\n",
        "After",
    ))?;

    assert!(!has_diagram(&plan, ViewerDiagramKind::Mermaid));
    assert!(has_code(&plan));
    Ok(())
}

#[test]
fn zenuml_mermaid_fence_stays_mermaid_diagram() -> Result<(), PreviewError> {
    let plan = plan_for(concat!(
        "Before\n",
        "```mermaid\n",
        "zenuml\n",
        "    title Order Service\n",
        "```\n",
        "After",
    ))?;

    assert_eq!(1, diagram_count(&plan, ViewerDiagramKind::Mermaid));
    Ok(())
}

#[test]
fn real_mermaid_after_non_diagram_fence_is_diagram() -> Result<(), PreviewError> {
    let plan = plan_for(concat!(
        "````markdown\n",
        "```mermaid\n",
        "graph TD\n",
        "    A --> B\n",
        "```\n",
        "````\n",
        "\n",
        "```mermaid\n",
        "graph TD\n",
        "    C --> D\n",
        "```",
    ))?;

    assert_eq!(1, diagram_count(&plan, ViewerDiagramKind::Mermaid));
    Ok(())
}

#[test]
fn table_followed_by_alert_keeps_both_viewer_nodes() -> Result<(), PreviewError> {
    let plan = plan_for(concat!(
        "| A | B |\n",
        "|---|---|\n",
        "| 1 | 2 |\n",
        "\n",
        "> [!TIP]\n",
        "> This is a tip.",
    ))?;

    assert!(has_kind(&plan, |kind| matches!(
        kind,
        ViewerNodeKind::Table
    )));
    assert!(has_kind(
        &plan,
        |kind| matches!(kind, ViewerNodeKind::Alert { label } if label == "TIP")
    ));
    Ok(())
}

#[test]
fn list_then_indented_mermaid_creates_diagram_node() -> Result<(), PreviewError> {
    let plan = plan_for(concat!(
        "- List item\n",
        "\n",
        "  ```mermaid\n",
        "  graph TD\n",
        "  A-->B\n",
        "  ```",
    ))?;

    assert!(has_kind(&plan, |kind| matches!(kind, ViewerNodeKind::List)));
    assert!(has_diagram(&plan, ViewerDiagramKind::Mermaid));
    Ok(())
}

#[test]
fn commonmark_blocks_keep_viewer_nodes_through_preview_output() -> Result<(), PreviewError> {
    let plan = plan_for(concat!("> quoted\n", "\n", "---"))?;

    assert!(has_kind(&plan, |kind| matches!(
        kind,
        ViewerNodeKind::BlockQuote
    )));
    assert!(has_kind(&plan, |kind| matches!(kind, ViewerNodeKind::Rule)));
    Ok(())
}

#[test]
fn raw_drawio_and_raw_plantuml_create_diagram_nodes() -> Result<(), PreviewError> {
    let plan = plan_for(concat!(
        "# Consecutive\n",
        "```mermaid\n",
        "graph TD\n",
        "```\n",
        "<mxGraphModel><root><mxCell id=\"0\"/></root></mxGraphModel>\n",
        "@startuml\n",
        "A -> B\n",
        "@enduml",
    ))?;

    assert!(has_diagram(&plan, ViewerDiagramKind::Mermaid));
    assert!(has_diagram(&plan, ViewerDiagramKind::DrawIo));
    assert!(has_diagram(&plan, ViewerDiagramKind::PlantUml));
    Ok(())
}

fn plan_for(content: &str) -> Result<ViewerNodePlan, PreviewError> {
    let output = PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: content.to_string(),
            document_id: Some("katana-preview-parity.md".to_string()),
        },
        &PreviewConfig::default(),
        CONTENT_HEIGHT,
    )?;
    Ok(ViewerNodePlanner::create(&output.input, 0.0))
}

fn has_diagram(plan: &ViewerNodePlan, expected: ViewerDiagramKind) -> bool {
    diagram_count(plan, expected) > 0
}

fn diagram_count(plan: &ViewerNodePlan, expected: ViewerDiagramKind) -> usize {
    plan.nodes
        .iter()
        .filter(|node| matches!(&node.kind, ViewerNodeKind::Diagram { kind } if *kind == expected))
        .count()
}

fn has_code(plan: &ViewerNodePlan) -> bool {
    has_kind(plan, |kind| matches!(kind, ViewerNodeKind::Code { .. }))
}

fn has_kind(plan: &ViewerNodePlan, predicate: impl Fn(&ViewerNodeKind) -> bool) -> bool {
    plan.nodes.iter().any(|node| predicate(&node.kind))
}
