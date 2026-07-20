use crate::export_quality::html_score_svg_media::RenderedSvgHtmlQuality;
use html_score_markdown_math_source::HtmlMarkdownMathSource;

pub(super) struct HtmlMarkdownRuntime;

impl HtmlMarkdownRuntime {
    pub(super) fn embeds(html: &str, source: &str) -> bool {
        if !Self::requires(source) {
            return true;
        }
        Self::renders_required_diagrams(html, source) && Self::renders_required_math(html, source)
    }

    pub(super) fn requires(source: &str) -> bool {
        Self::source_has_fence(source, &["math", "latex", "mermaid", "drawio", "plantuml"])
            || HtmlMarkdownMathSource::contains_math(source)
    }

    pub(super) fn source_has_fence(source: &str, languages: &[&str]) -> bool {
        source.lines().any(|line| {
            let trimmed = line.trim_start();
            let Some(info) = trimmed
                .strip_prefix("```")
                .or_else(|| trimmed.strip_prefix("~~~"))
            else {
                return false;
            };
            let Some(language) = info.split_whitespace().next() else {
                return false;
            };
            languages.contains(&language)
        })
    }

    pub(super) fn raw_fence_leaked(html: &str, languages: &[&str]) -> bool {
        html.lines().any(|line| {
            raw_fence_line_language(line).is_some_and(|language| languages.contains(&language))
        })
    }

    fn renders_required_diagrams(html: &str, source: &str) -> bool {
        ["mermaid", "drawio", "plantuml"].iter().all(|kind| {
            !Self::source_has_fence(source, &[*kind]) || rendered_diagram_figure(html, kind)
        })
    }

    fn renders_required_math(html: &str, source: &str) -> bool {
        if !Self::source_has_fence(source, &["math", "latex"])
            && !HtmlMarkdownMathSource::contains_math(source)
        {
            return true;
        }
        rendered_runtime_svg(html)
    }
}

fn rendered_runtime_svg(html: &str) -> bool {
    if !RenderedSvgHtmlQuality::has_rendered_svg(html) {
        return false;
    }
    html.contains("data-kdv-render-runtime=")
}

fn rendered_diagram_figure(html: &str, kind: &str) -> bool {
    let marker = format!(r#"data-kdv-diagram="{kind}""#);
    let Some(marker_start) = html.find(&marker) else {
        return false;
    };
    let Some(figure_start) = html[..marker_start].rfind("<figure") else {
        return false;
    };
    let after_figure = &html[figure_start..];
    let figure_end = after_figure
        .find("</figure>")
        .map(|offset| offset + "</figure>".len())
        .unwrap_or(after_figure.len());
    RenderedSvgHtmlQuality::has_rendered_svg(&after_figure[..figure_end])
}

fn raw_fence_line_language(line: &str) -> Option<&str> {
    let marker_start = line.find("```").or_else(|| line.find("~~~"))?;
    let info = &line[marker_start + 3..];
    info.split_whitespace().next()
}

#[path = "html_score_markdown_math_source.rs"]
mod html_score_markdown_math_source;

#[cfg(test)]
#[path = "html_score_markdown_runtime_private_tests.rs"]
mod tests;
