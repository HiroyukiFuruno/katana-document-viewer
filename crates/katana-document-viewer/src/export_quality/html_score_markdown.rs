use crate::export_quality::html_score_svg_media::RenderedSvgHtmlQuality;
use crate::export_quality::markdown_image_source::MarkdownImageSource;
use crate::export_quality::types::{ExportQualityCheck, check};
use footnote::footnote_reference_raw_leaked;
use html_score_markdown_alignment::HtmlMarkdownAlignment;
use html_score_markdown_blocks::HtmlMarkdownBlocks;
use html_score_markdown_inline::evaluates_inline_markdown;
use html_score_markdown_runtime::HtmlMarkdownRuntime;
use html_score_markdown_task::HtmlMarkdownTask;

pub(super) struct HtmlMarkdownQuality;

impl HtmlMarkdownQuality {
    pub(super) fn checks(html: &str, source: &str) -> Vec<ExportQualityCheck> {
        let mut checks = Self::semantic_checks(html, source);
        checks.extend(Self::runtime_checks(html, source));
        checks.extend(HtmlMarkdownBlocks::checks(html, source));
        checks.extend(HtmlMarkdownAlignment::checks(html, source));
        checks
    }

    fn semantic_checks(html: &str, source: &str) -> Vec<ExportQualityCheck> {
        vec![
            check(
                "html evaluates inline markdown",
                evaluates_inline_markdown(html, source),
                true,
                15,
            ),
            check(
                "html evaluates gfm alert",
                evaluates_alert(html, source),
                true,
                15,
            ),
            check(
                "html evaluates task state",
                HtmlMarkdownTask::evaluates(html, source),
                true,
                10,
            ),
            check(
                "html renders markdown image",
                renders_markdown_image(html, source),
                true,
                0,
            ),
        ]
    }

    fn runtime_checks(html: &str, source: &str) -> Vec<ExportQualityCheck> {
        vec![
            check(
                "html embeds render runtime",
                HtmlMarkdownRuntime::embeds(html, source),
                true,
                10,
            ),
            check(
                "html hides raw markdown",
                no_raw_markdown(html, source),
                true,
                10,
            ),
        ]
    }
}

fn evaluates_alert(html: &str, source: &str) -> bool {
    !requires_alert(source) || html.contains("data-github-alert=")
}
fn renders_markdown_image(html: &str, source: &str) -> bool {
    !requires_markdown_image(source)
        || html.contains("<img ")
        || RenderedSvgHtmlQuality::has_rendered_svg(html)
}
fn no_raw_markdown(html: &str, source: &str) -> bool {
    let mut needles = Vec::new();
    let runtime_fence_languages = ["math", "latex", "mermaid", "drawio", "plantuml"];
    if source.contains("**太字**") {
        needles.push("**太字**");
    }
    if requires_alert(source) {
        needles.extend([
            "[!WARNING]",
            "[!NOTE]",
            "[!TIP]",
            "[!IMPORTANT]",
            "[!CAUTION]",
        ]);
    }
    if HtmlMarkdownTask::requires_task(source) {
        needles.push("[/] 進行中");
    }
    needles.iter().all(|needle| !html.contains(needle))
        && (!HtmlMarkdownRuntime::source_has_fence(source, &runtime_fence_languages)
            || !HtmlMarkdownRuntime::raw_fence_leaked(html, &runtime_fence_languages))
        && !footnote_reference_raw_leaked(html, source)
}

#[cfg(test)]
pub(super) fn requires_runtime(source: &str) -> bool {
    HtmlMarkdownRuntime::requires(source)
}

fn requires_alert(source: &str) -> bool {
    let mut inside_fence = false;
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            inside_fence = !inside_fence;
            continue;
        }
        if inside_fence {
            continue;
        }
        if source_line_is_alert(trimmed) {
            return true;
        }
    }
    false
}

fn source_line_is_alert(trimmed: &str) -> bool {
    if let Some(quoted) = trimmed.strip_prefix('>').map(|quoted| quoted.trim_start()) {
        return quoted.starts_with("[!NOTE]")
            || quoted.starts_with("[!WARNING]")
            || quoted.starts_with("[!TIP]")
            || quoted.starts_with("[!IMPORTANT]")
            || quoted.starts_with("[!CAUTION]");
    }

    trimmed.starts_with("[!NOTE]")
        || trimmed.starts_with("[!WARNING]")
        || trimmed.starts_with("[!TIP]")
        || trimmed.starts_with("[!IMPORTANT]")
        || trimmed.starts_with("[!CAUTION]")
}

fn requires_markdown_image(source: &str) -> bool {
    let definitions = MarkdownImageSource::reference_definitions(source);
    let mut inside_fence = false;
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            inside_fence = !inside_fence;
            continue;
        }
        if !inside_fence && MarkdownImageSource::line_contains_markdown_image(trimmed, &definitions)
        {
            return true;
        }
    }
    false
}

#[path = "html_score_markdown_footnote.rs"]
mod footnote;

#[path = "html_score_markdown_alignment.rs"]
mod html_score_markdown_alignment;
#[path = "html_score_markdown_blocks.rs"]
mod html_score_markdown_blocks;
#[path = "html_score_markdown_inline.rs"]
mod html_score_markdown_inline;
#[path = "html_score_markdown_runtime.rs"]
mod html_score_markdown_runtime;
#[path = "html_score_markdown_task.rs"]
mod html_score_markdown_task;
