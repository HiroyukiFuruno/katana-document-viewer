use crate::{MarkdownSource, PreviewConfig, PreviewOutputFactory};
use crate::{
    ViewerDiagramKind, ViewerHtmlRole, ViewerNodeKind, ViewerNodePlan, ViewerNodePlanner,
    ViewerTextStyle, ViewerViewport,
};

const CONTENT_HEIGHT: f32 = 20_000.0;
const NO_COLOR: [u8; 4] = [0, 0, 0, 0];

#[test]
fn katana_sample_fixture_reaches_required_viewer_node_features()
-> Result<(), Box<dyn std::error::Error>> {
    let plan = plan_for(
        include_str!("../../../../assets/fixtures/katana/sample.md"),
        "assets/fixtures/katana/sample.md",
    )?;

    assert_sample_markdown_blocks(&plan);
    assert_sample_task_states(&plan);
    assert_sample_runtime_features(&plan);
    Ok(())
}

fn assert_sample_markdown_blocks(plan: &ViewerNodePlan) {
    assert!(has_kind(plan, |kind| matches!(kind, ViewerNodeKind::List)));
    assert!(has_text(plan, "Nested item 2-1"));
    assert!(has_syntax_highlight(plan, "rust"));
    assert!(has_kind(plan, |kind| matches!(kind, ViewerNodeKind::Table)));
    assert!(has_kind(plan, |kind| matches!(kind, ViewerNodeKind::Rule)));
    assert!(has_html_role(plan, ViewerHtmlRole::Accordion));
    assert!(has_kind(plan, |kind| {
        matches!(kind, ViewerNodeKind::FootnoteDefinition { .. })
    }));
    assert!(has_link_span(plan));
}

#[test]
fn katana_basic_fixture_reaches_gfm_alert_node_features() -> Result<(), Box<dyn std::error::Error>>
{
    let plan = plan_for(
        include_str!("../../../../assets/fixtures/katana/sample_basic.md"),
        "assets/fixtures/katana/sample_basic.md",
    )?;

    assert!(has_alert_label(&plan, "NOTE"));
    assert!(has_alert_label(&plan, "TIP"));
    assert!(has_alert_label(&plan, "IMPORTANT"));
    assert!(has_alert_label(&plan, "WARNING"));
    assert!(has_alert_label(&plan, "CAUTION"));
    assert!(has_strikethrough_span(&plan, "Strikethrough"));
    Ok(())
}

fn assert_sample_task_states(plan: &ViewerNodePlan) {
    assert!(has_task_marker(plan, "[ ]"));
    assert!(has_task_marker(plan, "[x]"));
    assert!(has_task_marker(plan, "[/]"));
    assert!(has_task_marker(plan, "[-]"));
}

fn assert_sample_runtime_features(plan: &ViewerNodePlan) {
    assert!(has_kind(plan, |kind| matches!(kind, ViewerNodeKind::Math)));
}

#[test]
fn katana_diagram_fixture_reaches_all_viewer_diagram_kinds()
-> Result<(), Box<dyn std::error::Error>> {
    let plan = plan_for(
        include_str!("../../../../assets/fixtures/katana/sample_diagrams.md"),
        "assets/fixtures/katana/sample_diagrams.md",
    )?;

    assert!(has_diagram(&plan, ViewerDiagramKind::Mermaid));
    assert!(has_diagram(&plan, ViewerDiagramKind::PlantUml));
    assert!(has_diagram(&plan, ViewerDiagramKind::DrawIo));
    Ok(())
}

#[test]
fn direct_html_fixture_reaches_alignment_link_table_and_accordion_nodes()
-> Result<(), Box<dyn std::error::Error>> {
    let plan = plan_for(
        include_str!("../../../../assets/fixtures/direct/html-alignment.html"),
        "assets/fixtures/direct/html-alignment.html",
    )?;

    assert!(has_html_role(&plan, ViewerHtmlRole::Centered));
    assert!(has_html_role(&plan, ViewerHtmlRole::Right));
    assert!(has_html_role(&plan, ViewerHtmlRole::Left));
    assert!(has_link_target(&plan, "https://example.com/docs"));
    assert!(has_kind(&plan, |kind| matches!(
        kind,
        ViewerNodeKind::Table
    )));
    assert!(has_html_role(&plan, ViewerHtmlRole::Accordion));
    Ok(())
}

fn plan_for(
    content: &str,
    document_id: &str,
) -> Result<ViewerNodePlan, Box<dyn std::error::Error>> {
    let output = PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: content.to_string(),
            document_id: Some(document_id.to_string()),
        },
        &config(),
        CONTENT_HEIGHT,
    )?;
    Ok(ViewerNodePlanner::create(&output.input, 0.0))
}

fn config() -> PreviewConfig {
    PreviewConfig {
        viewport: ViewerViewport {
            width: 1024.0,
            height: 20_000.0,
        },
        ..PreviewConfig::default()
    }
}

fn has_kind(plan: &ViewerNodePlan, matcher: impl Fn(&ViewerNodeKind) -> bool) -> bool {
    plan.nodes.iter().any(|node| matcher(&node.kind))
}

fn has_html_role(plan: &ViewerNodePlan, expected: ViewerHtmlRole) -> bool {
    has_kind(
        plan,
        |kind| matches!(kind, ViewerNodeKind::Html { role } if *role == expected),
    )
}

fn has_diagram(plan: &ViewerNodePlan, expected: ViewerDiagramKind) -> bool {
    has_kind(
        plan,
        |kind| matches!(kind, ViewerNodeKind::Diagram { kind } if *kind == expected),
    )
}

fn has_text(plan: &ViewerNodePlan, expected: &str) -> bool {
    plan.nodes.iter().any(|node| node.text.contains(expected))
}

fn has_task_marker(plan: &ViewerNodePlan, expected: &str) -> bool {
    plan.nodes
        .iter()
        .filter(|node| matches!(node.kind, ViewerNodeKind::List))
        .any(|node| node.text.contains(expected))
}

fn has_link_span(plan: &ViewerNodePlan) -> bool {
    plan.nodes
        .iter()
        .flat_map(|node| node.spans.iter())
        .any(|span| !span.link_target.is_empty())
}

fn has_link_target(plan: &ViewerNodePlan, target: &str) -> bool {
    plan.nodes
        .iter()
        .flat_map(|node| node.spans.iter())
        .any(|span| span.link_target == target)
}

fn has_strikethrough_span(plan: &ViewerNodePlan, expected: &str) -> bool {
    plan.nodes
        .iter()
        .flat_map(|node| node.spans.iter())
        .any(|span| span.text == expected && span.style.strikethrough)
}

fn has_alert_label(plan: &ViewerNodePlan, expected: &str) -> bool {
    has_kind(
        plan,
        |kind| matches!(kind, ViewerNodeKind::Alert { label } if label == expected),
    )
}

fn has_syntax_highlight(plan: &ViewerNodePlan, language_name: &str) -> bool {
    plan.nodes.iter().any(|node| {
        matches!(&node.kind, ViewerNodeKind::Code { language: Some(value) } if value == language_name)
            && node.spans.iter().any(has_syntax_span)
    })
}

fn has_syntax_span(span: &crate::ViewerTextSpan) -> bool {
    let style = span.style;
    style.monospace && style.color_rgba != NO_COLOR && style != ViewerTextStyle::default()
}
