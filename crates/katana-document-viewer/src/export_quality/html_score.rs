use crate::export_quality::types::{ExportFormatQualityScore, check};
use crate::forge::ExportFormat;

pub(crate) struct HtmlQualityScore;

impl HtmlQualityScore {
    pub(crate) fn score(bytes: &[u8]) -> ExportFormatQualityScore {
        let html = std::str::from_utf8(bytes).map_or("", |value| value);
        ExportFormatQualityScore::new(
            ExportFormat::Html,
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
                    "html evaluates inline markdown",
                    evaluates_inline_markdown(html),
                    true,
                    15,
                ),
                check("html evaluates gfm alert", evaluates_alert(html), true, 15),
                check("html evaluates task state", evaluates_task(html), true, 10),
                check("html embeds render runtime", embeds_runtime(html), true, 10),
                check("html hides raw markdown", no_raw_markdown(html), true, 10),
            ],
        )
    }
}

fn has_kdv_root(html: &str) -> bool {
    html.contains("data-kdv-export")
}

fn has_export_style(html: &str) -> bool {
    html.contains("data-kdv-export-style")
}

fn evaluates_inline_markdown(html: &str) -> bool {
    html.contains("<strong>") && html.contains("<a href=")
}

fn evaluates_alert(html: &str) -> bool {
    html.contains("data-github-alert=")
}

fn evaluates_task(html: &str) -> bool {
    html.contains("data-kdv-task-state=")
}

fn embeds_runtime(html: &str) -> bool {
    html.contains("data-kdv-render-runtime=")
}

fn no_raw_markdown(html: &str) -> bool {
    [
        "**太字**",
        "[!WARNING]",
        "[/] 進行中",
        "```math",
        "```mermaid",
    ]
    .iter()
    .all(|needle| !html.contains(needle))
}
