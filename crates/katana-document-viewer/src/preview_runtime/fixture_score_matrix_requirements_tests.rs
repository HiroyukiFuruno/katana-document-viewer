use crate::preview_runtime::fixture_score_matrix_support::{
    ExportBytes, FixtureScoreAssertions, FixtureScoreCase,
};
use crate::{
    ViewerHtmlRole, ViewerNodeKind, ViewerNodePlan, ViewerNodePlanner, ViewerTextSpan,
    ViewerTextStyle,
};

const NO_COLOR: [u8; 4] = [0, 0, 0, 0];

#[test]
fn fixture_requirements_and_export_scores_cover_all_formats()
-> Result<(), Box<dyn std::error::Error>> {
    for case in fixture_cases() {
        let output = case.fixture.preview_output()?;
        let plan = ViewerNodePlanner::create(&output.input, 0.0);
        for requirement in case.requirements {
            assert!(
                requirement.is_satisfied(&plan),
                "{} missing requirement {}",
                case.fixture.name,
                requirement.name()
            );
        }
        let export_bytes = ExportBytes::from_output(&output)?;
        FixtureScoreAssertions::assert_export_score(
            case.fixture.name,
            &export_bytes.score_report(case.fixture.content),
        );
    }
    Ok(())
}

fn fixture_cases() -> Vec<RequirementScoreCase<'static>> {
    vec![direct_html_case(), katana_basic_markdown_case()]
}

fn direct_html_case() -> RequirementScoreCase<'static> {
    RequirementScoreCase {
        fixture: FixtureScoreCase {
            name: "direct/html-alignment.html",
            document_id: "assets/fixtures/direct/html-alignment.html",
            content: include_str!("../../../../assets/fixtures/direct/html-alignment.html"),
        },
        requirements: vec![
            ScoreRequirement::HtmlRole("html:center", ViewerHtmlRole::Centered),
            ScoreRequirement::HtmlRole("html:right", ViewerHtmlRole::Right),
            ScoreRequirement::HtmlRole("html:left", ViewerHtmlRole::Left),
            ScoreRequirement::Kind("html:table", RequiredKind::Table),
            ScoreRequirement::HtmlRole("html:accordion", ViewerHtmlRole::Accordion),
            ScoreRequirement::Link("html:link", "https://example.com/docs"),
        ],
    }
}

fn katana_basic_markdown_case() -> RequirementScoreCase<'static> {
    RequirementScoreCase {
        fixture: FixtureScoreCase {
            name: "katana/sample_basic.md",
            document_id: "assets/fixtures/katana/sample_basic.md",
            content: include_str!("../../../../assets/fixtures/katana/sample_basic.md"),
        },
        requirements: vec![
            ScoreRequirement::Kind("markdown:list", RequiredKind::List),
            ScoreRequirement::Text("markdown:nested-list", "Nested item 2-1"),
            ScoreRequirement::Syntax("markdown:syntax-highlight", "rust"),
            ScoreRequirement::Strike("markdown:strikethrough", "Strikethrough"),
            ScoreRequirement::Kind("markdown:table", RequiredKind::Table),
            ScoreRequirement::Kind("markdown:rule", RequiredKind::Rule),
            ScoreRequirement::Link("markdown:link", "https://github.com"),
            ScoreRequirement::Text("task:todo", "[ ]"),
            ScoreRequirement::Text("task:done", "[x]"),
            ScoreRequirement::Text("task:progress", "[/]"),
            ScoreRequirement::Text("task:blocked", "[-]"),
            ScoreRequirement::Kind("markdown:alert", RequiredKind::Alert),
            ScoreRequirement::HtmlRole("markdown:accordion", ViewerHtmlRole::Accordion),
            ScoreRequirement::Kind("markdown:math", RequiredKind::Math),
            ScoreRequirement::Kind("markdown:footnote", RequiredKind::Footnote),
        ],
    }
}

struct RequirementScoreCase<'a> {
    fixture: FixtureScoreCase<'a>,
    requirements: Vec<ScoreRequirement>,
}

enum ScoreRequirement {
    Kind(&'static str, RequiredKind),
    HtmlRole(&'static str, ViewerHtmlRole),
    Link(&'static str, &'static str),
    Strike(&'static str, &'static str),
    Syntax(&'static str, &'static str),
    Text(&'static str, &'static str),
}

impl ScoreRequirement {
    fn name(&self) -> &'static str {
        match self {
            Self::Kind(name, _)
            | Self::HtmlRole(name, _)
            | Self::Link(name, _)
            | Self::Strike(name, _)
            | Self::Syntax(name, _)
            | Self::Text(name, _) => name,
        }
    }

    fn is_satisfied(&self, plan: &ViewerNodePlan) -> bool {
        match self {
            Self::Kind(_, kind) => has_required_kind(plan, *kind),
            Self::HtmlRole(_, role) => FixtureScoreAssertions::has_html_role(plan, *role),
            Self::Link(_, target) => FixtureScoreAssertions::has_link_target(plan, target),
            Self::Strike(_, text) => has_strikethrough_span(plan, text),
            Self::Syntax(_, language) => has_syntax_highlight(plan, language),
            Self::Text(_, text) => FixtureScoreAssertions::has_text(plan, text),
        }
    }
}

#[derive(Clone, Copy)]
enum RequiredKind {
    Alert,
    Footnote,
    List,
    Math,
    Rule,
    Table,
}

fn has_required_kind(plan: &ViewerNodePlan, required: RequiredKind) -> bool {
    FixtureScoreAssertions::has_kind(plan, |kind| match required {
        RequiredKind::Alert => matches!(kind, ViewerNodeKind::Alert { .. }),
        RequiredKind::Footnote => matches!(kind, ViewerNodeKind::FootnoteDefinition { .. }),
        RequiredKind::List => matches!(kind, ViewerNodeKind::List),
        RequiredKind::Math => matches!(kind, ViewerNodeKind::Math),
        RequiredKind::Rule => matches!(kind, ViewerNodeKind::Rule),
        RequiredKind::Table => matches!(kind, ViewerNodeKind::Table),
    })
}

fn has_syntax_highlight(plan: &ViewerNodePlan, language_name: &str) -> bool {
    plan.nodes.iter().any(|node| {
        matches!(&node.kind, ViewerNodeKind::Code { language: Some(value) } if value == language_name)
            && node.spans.iter().any(has_syntax_span)
    })
}

fn has_syntax_span(span: &ViewerTextSpan) -> bool {
    let style = span.style;
    style.monospace && style.color_rgba != NO_COLOR && style != ViewerTextStyle::default()
}

fn has_strikethrough_span(plan: &ViewerNodePlan, expected: &str) -> bool {
    plan.nodes
        .iter()
        .flat_map(|node| node.spans.iter())
        .any(|span| span.text == expected && span.style.strikethrough)
}
