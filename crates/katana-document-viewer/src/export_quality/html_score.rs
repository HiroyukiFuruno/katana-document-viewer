use crate::export_quality::types::{ExportFormatQualityScore, ExportQualityCheck, check};
use crate::forge::ExportFormat;
use html_score_direct_visual::HtmlDirectVisualQuality as DirectVisualQuality;
use html_score_markdown::HtmlMarkdownQuality;
use html_score_source_text::HtmlSourceTextQuality;

pub(crate) struct HtmlQualityScore;

impl HtmlQualityScore {
    pub(crate) fn score(bytes: &[u8], source_markdown: &str) -> ExportFormatQualityScore {
        let html = std::str::from_utf8(bytes).map_or("", |value| value);
        ExportFormatQualityScore::new(
            ExportFormat::Html,
            html_checks(bytes, html, source_markdown),
        )
    }
}

fn html_checks(bytes: &[u8], html: &str, source_markdown: &str) -> Vec<ExportQualityCheck> {
    let mut checks = baseline_html_checks(bytes, html);
    checks.extend(markdown_html_checks(html, source_markdown));
    checks.extend(DirectVisualQuality::checks(html, source_markdown));
    checks.extend(HtmlSourceTextQuality::checks(html, source_markdown));
    checks
}

fn baseline_html_checks(bytes: &[u8], html: &str) -> Vec<ExportQualityCheck> {
    vec![
        check("html is non-empty", !bytes.is_empty(), true, 10),
        check(
            "html is utf-8",
            std::str::from_utf8(bytes).is_ok(),
            true,
            10,
        ),
        check("html has kdv root", has_kdv_root(html), true, 10),
        check("html has export style", has_export_style(html), true, 10),
        check(
            "html has no render errors",
            !has_render_error(html),
            true,
            0,
        ),
    ]
}

fn markdown_html_checks(html: &str, source_markdown: &str) -> Vec<ExportQualityCheck> {
    HtmlMarkdownQuality::checks(html, source_markdown)
}

fn has_kdv_root(html: &str) -> bool {
    html.contains("data-kdv-export")
}

fn has_export_style(html: &str) -> bool {
    html.contains("data-kdv-export-style")
}

fn has_render_error(html: &str) -> bool {
    html.to_ascii_lowercase().contains("data-kdv-render-error=")
}

#[cfg(test)]
fn requires_runtime(source_markdown: &str) -> bool {
    html_score_markdown::requires_runtime(source_markdown)
}

#[cfg(test)]
#[path = "html_score_test_modules.rs"]
mod test_modules;

#[path = "html_score_direct_visual.rs"]
mod html_score_direct_visual;

#[path = "html_score_markdown.rs"]
mod html_score_markdown;

#[path = "html_score_source_text.rs"]
mod html_score_source_text;
