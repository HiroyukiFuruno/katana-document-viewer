use crate::preview_runtime::fixture_score_matrix_render_support::RenderedExportBytes;
use crate::preview_runtime::fixture_score_matrix_support::{
    FixtureScoreAssertions, FixtureScoreCase,
};
use crate::{ViewerDiagramKind, ViewerHtmlRole, ViewerNodeKind, ViewerNodePlan, ViewerNodePlanner};

#[test]
fn fixture_score_binds_katana_full_samples_to_export_score()
-> Result<(), Box<dyn std::error::Error>> {
    for case in full_sample_cases() {
        let output = case.fixture.preview_output()?;
        let plan = ViewerNodePlanner::create(&output.input, 0.0);
        for requirement in case.requirements {
            assert!(
                requirement.is_satisfied(&plan),
                "{} missing {}",
                case.fixture.name,
                requirement.name()
            );
        }
        let export_bytes = RenderedExportBytes::from_output(&output)?;
        FixtureScoreAssertions::assert_export_score(
            case.fixture.name,
            &export_bytes.score_report(case.fixture.content),
        );
    }
    Ok(())
}

struct FullSampleCase<'a> {
    fixture: FixtureScoreCase<'a>,
    requirements: Vec<FullSampleRequirement>,
}

fn full_sample_cases() -> Vec<FullSampleCase<'static>> {
    vec![
        katana_full_case(
            "katana/sample.md",
            "assets/fixtures/katana/sample.md",
            include_str!("../../../../assets/fixtures/katana/sample.md"),
        ),
        katana_full_case(
            "katana/sample.ja.md",
            "assets/fixtures/katana/sample.ja.md",
            include_str!("../../../../assets/fixtures/katana/sample.ja.md"),
        ),
        katana_html_case(
            "katana/sample_html.md",
            "assets/fixtures/katana/sample_html.md",
            include_str!("../../../../assets/fixtures/katana/sample_html.md"),
        ),
        katana_html_case(
            "katana/sample_html.ja.md",
            "assets/fixtures/katana/sample_html.ja.md",
            include_str!("../../../../assets/fixtures/katana/sample_html.ja.md"),
        ),
    ]
}

fn katana_full_case(
    name: &'static str,
    document_id: &'static str,
    content: &'static str,
) -> FullSampleCase<'static> {
    FullSampleCase {
        fixture: FixtureScoreCase {
            name,
            document_id,
            content,
        },
        requirements: vec![
            FullSampleRequirement::Kind("list", RequiredKind::List),
            FullSampleRequirement::Kind("table", RequiredKind::Table),
            FullSampleRequirement::Kind("rule", RequiredKind::Rule),
            FullSampleRequirement::Kind("math", RequiredKind::Math),
            FullSampleRequirement::Kind("footnote", RequiredKind::Footnote),
            FullSampleRequirement::HtmlRole("html:center", ViewerHtmlRole::Centered),
            FullSampleRequirement::HtmlRole("accordion", ViewerHtmlRole::Accordion),
            FullSampleRequirement::Diagram("diagram:mermaid", ViewerDiagramKind::Mermaid),
            FullSampleRequirement::Diagram("diagram:plantuml", ViewerDiagramKind::PlantUml),
            FullSampleRequirement::Diagram("diagram:drawio", ViewerDiagramKind::DrawIo),
        ],
    }
}

fn katana_html_case(
    name: &'static str,
    document_id: &'static str,
    content: &'static str,
) -> FullSampleCase<'static> {
    FullSampleCase {
        fixture: FixtureScoreCase {
            name,
            document_id,
            content,
        },
        requirements: vec![
            FullSampleRequirement::HtmlRole("html:center", ViewerHtmlRole::Centered),
            FullSampleRequirement::Link("html:language-link"),
        ],
    }
}

enum FullSampleRequirement {
    Kind(&'static str, RequiredKind),
    HtmlRole(&'static str, ViewerHtmlRole),
    Diagram(&'static str, ViewerDiagramKind),
    Link(&'static str),
}

impl FullSampleRequirement {
    fn name(&self) -> &'static str {
        match self {
            Self::Kind(name, _)
            | Self::HtmlRole(name, _)
            | Self::Diagram(name, _)
            | Self::Link(name) => name,
        }
    }

    fn is_satisfied(&self, plan: &ViewerNodePlan) -> bool {
        match self {
            Self::Kind(_, kind) => has_required_kind(plan, *kind),
            Self::HtmlRole(_, role) => FixtureScoreAssertions::has_html_role(plan, *role),
            Self::Diagram(_, kind) => has_diagram(plan, *kind),
            Self::Link(_) => has_any_link(plan),
        }
    }
}

#[derive(Clone, Copy)]
enum RequiredKind {
    Footnote,
    List,
    Math,
    Rule,
    Table,
}

fn has_required_kind(plan: &ViewerNodePlan, required: RequiredKind) -> bool {
    FixtureScoreAssertions::has_kind(plan, |kind| match required {
        RequiredKind::Footnote => matches!(kind, ViewerNodeKind::FootnoteDefinition { .. }),
        RequiredKind::List => matches!(kind, ViewerNodeKind::List),
        RequiredKind::Math => matches!(kind, ViewerNodeKind::Math),
        RequiredKind::Rule => matches!(kind, ViewerNodeKind::Rule),
        RequiredKind::Table => matches!(kind, ViewerNodeKind::Table),
    })
}

fn has_diagram(plan: &ViewerNodePlan, expected: ViewerDiagramKind) -> bool {
    FixtureScoreAssertions::has_kind(
        plan,
        |kind| matches!(kind, ViewerNodeKind::Diagram { kind } if *kind == expected),
    )
}

fn has_any_link(plan: &ViewerNodePlan) -> bool {
    plan.nodes
        .iter()
        .flat_map(|node| node.spans.iter())
        .any(|span| !span.link_target.is_empty())
}
