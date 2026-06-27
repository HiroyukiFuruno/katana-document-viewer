use crate::{ExportFormat, ExportQualityReport, ViewerHtmlRole, ViewerNodeKind, ViewerNodePlan};

const EXPORT_SCORE_THRESHOLD: u16 = 95;

pub(crate) struct FixtureScoreAssertions;

impl FixtureScoreAssertions {
    pub(crate) fn assert_export_score(fixture_name: &str, report: &ExportQualityReport) {
        assert!(
            report.fatal_failures.is_empty(),
            "{fixture_name} export has fatal failures: {report:#?}"
        );
        Self::assert_all_formats_present(fixture_name, report);
        Self::assert_all_scores_pass(fixture_name, report);
    }

    pub(crate) fn has_kind(
        plan: &ViewerNodePlan,
        matcher: impl Fn(&ViewerNodeKind) -> bool,
    ) -> bool {
        plan.nodes.iter().any(|node| matcher(&node.kind))
    }

    pub(crate) fn has_html_role(plan: &ViewerNodePlan, expected: ViewerHtmlRole) -> bool {
        Self::has_kind(
            plan,
            |kind| matches!(kind, ViewerNodeKind::Html { role } if *role == expected),
        )
    }

    pub(crate) fn has_link_target(plan: &ViewerNodePlan, target: &str) -> bool {
        plan.nodes
            .iter()
            .flat_map(|node| node.spans.iter())
            .any(|span| span.link_target == target)
    }

    pub(crate) fn has_text(plan: &ViewerNodePlan, expected: &str) -> bool {
        plan.nodes.iter().any(|node| node.text.contains(expected))
    }

    fn assert_all_formats_present(fixture_name: &str, report: &ExportQualityReport) {
        for expected_format in [
            ExportFormat::Html,
            ExportFormat::Pdf,
            ExportFormat::Png,
            ExportFormat::Jpeg,
        ] {
            assert!(
                report
                    .format_scores
                    .iter()
                    .any(|format_score| format_score.format == expected_format),
                "{fixture_name} export score is missing {expected_format:?}: {report:#?}"
            );
        }
    }

    fn assert_all_scores_pass(fixture_name: &str, report: &ExportQualityReport) {
        for format_score in &report.format_scores {
            assert!(
                format_score.score >= EXPORT_SCORE_THRESHOLD,
                "{fixture_name} {:?} score is {}/{}; report={report:#?}",
                format_score.format,
                format_score.score,
                format_score.max_score
            );
        }
    }
}
