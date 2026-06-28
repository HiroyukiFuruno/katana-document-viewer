use crate::preview_runtime::fixture_score_matrix_support::{
    ExportBytes, FixtureScoreAssertions, FixtureScoreCase,
};
use crate::{ViewerHtmlRole, ViewerNodeKind, ViewerNodePlan, ViewerNodePlanner, ViewerTextStyle};

const NO_COLOR: [u8; 4] = [0, 0, 0, 0];

#[test]
fn fixture_score_binds_katana_basic_markdown_requirements_to_export_score()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = FixtureScoreCase {
        name: "katana/sample_basic.md",
        document_id: "assets/fixtures/katana/sample_basic.md",
        content: include_str!("../../../../assets/fixtures/katana/sample_basic.md"),
    };
    let output = fixture.preview_output()?;
    let plan = ViewerNodePlanner::create(&output.input, 0.0);

    assert_katana_basic_requirements(&plan);
    let export_bytes = ExportBytes::from_output(&output)?;
    FixtureScoreAssertions::assert_export_score(
        fixture.name,
        &export_bytes.score_report(fixture.content),
    );
    Ok(())
}

fn assert_katana_basic_requirements(plan: &ViewerNodePlan) {
    assert_markdown_blocks(plan);
    assert_task_states(plan);
    assert_runtime_blocks(plan);
}

fn assert_markdown_blocks(plan: &ViewerNodePlan) {
    assert!(FixtureScoreAssertions::has_kind(plan, |kind| {
        matches!(kind, ViewerNodeKind::List)
    }));
    assert!(FixtureScoreAssertions::has_text(plan, "Nested item 2-1"));
    assert!(has_syntax_highlight(plan, "rust"));
    assert!(FixtureScoreAssertions::has_kind(plan, |kind| {
        matches!(kind, ViewerNodeKind::Table)
    }));
    assert!(FixtureScoreAssertions::has_kind(plan, |kind| {
        matches!(kind, ViewerNodeKind::Rule)
    }));
    assert!(FixtureScoreAssertions::has_link_target(
        plan,
        "https://github.com"
    ));
}

fn assert_task_states(plan: &ViewerNodePlan) {
    assert!(has_task_marker(plan, "[ ]"));
    assert!(has_task_marker(plan, "[x]"));
    assert!(has_task_marker(plan, "[/]"));
    assert!(has_task_marker(plan, "[-]"));
}

fn assert_runtime_blocks(plan: &ViewerNodePlan) {
    assert!(FixtureScoreAssertions::has_kind(plan, |kind| {
        matches!(kind, ViewerNodeKind::Alert { .. })
    }));
    assert!(FixtureScoreAssertions::has_html_role(
        plan,
        ViewerHtmlRole::Accordion
    ));
    assert!(FixtureScoreAssertions::has_kind(plan, |kind| {
        matches!(kind, ViewerNodeKind::Math)
    }));
    assert!(FixtureScoreAssertions::has_kind(plan, |kind| {
        matches!(kind, ViewerNodeKind::FootnoteDefinition { .. })
    }));
}

fn has_task_marker(plan: &ViewerNodePlan, expected: &str) -> bool {
    plan.nodes
        .iter()
        .filter(|node| matches!(node.kind, ViewerNodeKind::List))
        .any(|node| node.text.contains(expected))
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
