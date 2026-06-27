use crate::export_quality::types::{ExportQualityCheck, check};
use html_score_markdown_block_check::HtmlBlockCheckSpec;
use html_score_markdown_blocks_helpers::{
    html_code_block_count, html_has_code, html_has_heading, html_has_list, html_has_nested_list,
    html_heading_count, html_list_item_count, html_table_count,
};
use html_score_markdown_visible_source::VisibleMarkdownSource;

pub(super) struct HtmlMarkdownBlocks;

impl HtmlMarkdownBlocks {
    pub(super) fn checks(html: &str, source: &str) -> Vec<ExportQualityCheck> {
        let source = VisibleMarkdownSource::from(source);
        let mut checks = Vec::new();
        Self::push_primary_checks(&mut checks, html, &source);
        Self::push_secondary_checks(&mut checks, html, &source);
        checks
    }

    fn push_primary_checks(
        checks: &mut Vec<ExportQualityCheck>,
        html: &str,
        source: &VisibleMarkdownSource,
    ) {
        Self::push_heading_checks(checks, html, source);
        Self::push_list_checks(checks, html, source);
        checks.push(Self::check(
            "html renders blockquote block",
            source.has_blockquote(),
            html.contains("<blockquote"),
        ));
    }

    fn push_heading_checks(
        checks: &mut Vec<ExportQualityCheck>,
        html: &str,
        source: &VisibleMarkdownSource,
    ) {
        checks.push(Self::check(
            "html renders heading block",
            source.has_heading(),
            html_has_heading(html),
        ));
        checks.push(Self::heading_count_check(html, source));
    }

    fn push_list_checks(
        checks: &mut Vec<ExportQualityCheck>,
        html: &str,
        source: &VisibleMarkdownSource,
    ) {
        checks.push(Self::check(
            "html renders list block",
            source.has_list(),
            html_has_list(html),
        ));
        checks.push(Self::count_check(
            "html preserves list item count",
            source.list_item_count(),
            html_list_item_count(html),
        ));
        checks.push(Self::check(
            "html preserves nested list depth",
            source.has_nested_list(),
            html_has_nested_list(html),
        ));
    }

    fn push_secondary_checks(
        checks: &mut Vec<ExportQualityCheck>,
        html: &str,
        source: &VisibleMarkdownSource,
    ) {
        let candidates = [
            Self::table_check(html, source),
            Self::code_check(html, source),
            Self::syntax_check(html, source),
            Self::rule_check(html, source),
            Self::footnote_check(html, source),
            Self::details_check(html, source),
        ];
        checks.push(Self::table_count_check(html, source));
        checks.push(Self::code_count_check(html, source));
        for candidate in candidates {
            checks.push(Self::check(
                candidate.name,
                candidate.required,
                candidate.present,
            ));
        }
    }

    fn check(name: &str, required: bool, present: bool) -> ExportQualityCheck {
        check(name, !required || present, true, 0)
    }

    fn heading_count_check(html: &str, source: &VisibleMarkdownSource) -> ExportQualityCheck {
        let expected = source.heading_count();
        Self::count_check(
            "html preserves heading count",
            expected,
            html_heading_count(html),
        )
    }

    fn count_check(name: &'static str, expected: usize, actual: usize) -> ExportQualityCheck {
        check(name, expected < 2 || actual >= expected, true, 0)
    }

    fn table_check(html: &str, source: &VisibleMarkdownSource) -> HtmlBlockCheckSpec {
        HtmlBlockCheckSpec::new(
            "html renders table block",
            source.has_table(),
            html.contains("<table"),
        )
    }

    fn table_count_check(html: &str, source: &VisibleMarkdownSource) -> ExportQualityCheck {
        Self::count_check(
            "html preserves table count",
            source.table_count(),
            html_table_count(html),
        )
    }

    fn code_check(html: &str, source: &VisibleMarkdownSource) -> HtmlBlockCheckSpec {
        HtmlBlockCheckSpec::new(
            "html renders code block",
            source.has_code_block(),
            html_has_code(html),
        )
    }

    fn code_count_check(html: &str, source: &VisibleMarkdownSource) -> ExportQualityCheck {
        Self::count_check(
            "html preserves code block count",
            source.code_block_count(),
            html_code_block_count(html),
        )
    }

    fn syntax_check(html: &str, source: &VisibleMarkdownSource) -> HtmlBlockCheckSpec {
        HtmlBlockCheckSpec::new(
            "html renders syntax highlighted code",
            source.has_syntax_code_block(),
            html_has_syntax_highlighted_code(html),
        )
    }

    fn rule_check(html: &str, source: &VisibleMarkdownSource) -> HtmlBlockCheckSpec {
        HtmlBlockCheckSpec::new(
            "html renders thematic break",
            source.has_rule(),
            html.contains("<hr"),
        )
    }

    fn footnote_check(html: &str, source: &VisibleMarkdownSource) -> HtmlBlockCheckSpec {
        HtmlBlockCheckSpec::new(
            "html renders footnote definition",
            source.has_footnote(),
            html.contains("data-kdv-footnote-definition"),
        )
    }

    fn details_check(html: &str, source: &VisibleMarkdownSource) -> HtmlBlockCheckSpec {
        HtmlBlockCheckSpec::new(
            "html renders details accordion",
            source.has_details_html(),
            html_has_details_accordion(html),
        )
    }
}

fn html_has_syntax_highlighted_code(html: &str) -> bool {
    html.contains("data-kdv-code-highlighter=") && html.contains("<span style=")
}

fn html_has_details_accordion(html: &str) -> bool {
    let lower = html.to_ascii_lowercase();
    lower.contains("data-kdv-accordion=")
        || (lower.contains("<details") && lower.contains("<summary"))
}

#[cfg(test)]
#[path = "html_score_markdown_blocks_unit_tests.rs"]
mod unit_tests;

#[path = "html_score_markdown_block_check.rs"]
mod html_score_markdown_block_check;
#[path = "html_score_markdown_blocks_helpers.rs"]
mod html_score_markdown_blocks_helpers;

#[path = "html_score_markdown_heading.rs"]
mod html_score_markdown_heading;
#[path = "html_score_markdown_visible_source.rs"]
mod html_score_markdown_visible_source;
