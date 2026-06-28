use crate::preview_runtime::fixture_score_matrix_support::{
    ExportBytes, FixtureScoreAssertions, FixtureScoreCase,
};
use crate::{ViewerHtmlRole, ViewerNodeKind, ViewerNodePlan, ViewerNodePlanner};

#[test]
fn fixture_score_binds_direct_html_requirements_to_export_score()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = FixtureScoreCase {
        name: "direct/html-alignment.html",
        document_id: "assets/fixtures/direct/html-alignment.html",
        content: include_str!("../../../../assets/fixtures/direct/html-alignment.html"),
    };
    let output = fixture.preview_output()?;
    let plan = ViewerNodePlanner::create(&output.input, 0.0);

    assert_direct_html_requirements(&plan);
    let export_bytes = ExportBytes::from_output(&output)?;
    FixtureScoreAssertions::assert_export_score(
        fixture.name,
        &export_bytes.score_report(fixture.content),
    );
    Ok(())
}

fn assert_direct_html_requirements(plan: &ViewerNodePlan) {
    assert!(
        FixtureScoreAssertions::has_html_role(plan, ViewerHtmlRole::Centered),
        "center alignment node is required"
    );
    assert!(
        FixtureScoreAssertions::has_html_role(plan, ViewerHtmlRole::Right),
        "right alignment node is required"
    );
    assert!(
        FixtureScoreAssertions::has_html_role(plan, ViewerHtmlRole::Left),
        "left alignment node is required"
    );
    assert!(
        FixtureScoreAssertions::has_kind(plan, |kind| matches!(kind, ViewerNodeKind::Table)),
        "table node is required"
    );
    assert!(
        FixtureScoreAssertions::has_html_role(plan, ViewerHtmlRole::Accordion),
        "accordion node is required"
    );
    assert!(
        FixtureScoreAssertions::has_link_target(plan, "https://example.com/docs"),
        "link target is required"
    );
}
