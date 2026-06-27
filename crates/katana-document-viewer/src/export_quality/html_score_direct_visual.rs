use crate::export_quality::html_score_svg_media::RenderedSvgHtmlQuality;
use crate::export_quality::types::{ExportQualityCheck, check};
use html_score_direct_visual_img::HtmlImgSourceQuality;
use html_score_direct_visual_source::DirectVisualSource;
use html_score_direct_visual_svg::RawSvgHtmlQuality;

pub(super) struct HtmlDirectVisualQuality;

impl HtmlDirectVisualQuality {
    pub(super) fn checks(html: &str, source: &str) -> Vec<ExportQualityCheck> {
        let mut checks = Vec::new();
        if DirectVisualSource::is_direct_image(source) {
            Self::push_check(
                &mut checks,
                "html renders direct image",
                Self::html_has_image_media(html, source),
            );
        }
        for diagram in Self::diagram_checks(source) {
            Self::push_diagram_check(&mut checks, html, diagram);
        }
        checks
    }

    fn push_diagram_check(
        checks: &mut Vec<ExportQualityCheck>,
        html: &str,
        diagram: DirectDiagramCheck,
    ) {
        if diagram.required {
            let name = format!("html renders direct {}", diagram.label);
            Self::push_check(
                checks,
                &name,
                Self::html_has_diagram_media(html, diagram.kind),
            );
        }
    }

    fn diagram_checks(source: &str) -> [DirectDiagramCheck; 3] {
        [
            DirectDiagramCheck::new("Draw.io", "drawio", DirectVisualSource::is_drawio(source)),
            DirectDiagramCheck::new("Mermaid", "mermaid", DirectVisualSource::is_mermaid(source)),
            DirectDiagramCheck::new(
                "PlantUML",
                "plantuml",
                DirectVisualSource::is_plantuml(source),
            ),
        ]
    }

    fn push_check(checks: &mut Vec<ExportQualityCheck>, name: &str, passed: bool) {
        checks.push(check(name, passed, true, 0));
    }

    fn html_has_image_media(html: &str, source: &str) -> bool {
        if DirectVisualSource::is_raw_svg(source) {
            return RawSvgHtmlQuality::has_media(html, source);
        }
        let Some(uri) = DirectVisualSource::source_uri(source) else {
            return false;
        };
        if HtmlImgSourceQuality::has_uri(html, &uri) {
            return true;
        }
        DirectVisualSource::is_svg_file(source)
            && html.contains(&uri)
            && RenderedSvgHtmlQuality::has_rendered_svg(html)
    }

    fn html_has_diagram_media(html: &str, kind: &str) -> bool {
        let Some(figure) = Self::diagram_figure(html, kind) else {
            return false;
        };
        figure.contains("data-kdv-diagram-theme=")
            && RenderedSvgHtmlQuality::has_rendered_svg(figure)
    }

    fn diagram_figure<'a>(html: &'a str, kind: &str) -> Option<&'a str> {
        let marker = format!(r#"data-kdv-diagram="{kind}""#);
        let marker_start = html.find(&marker)?;
        let figure_start = html[..marker_start].rfind("<figure")?;
        let after_figure = &html[figure_start..];
        let figure_end = after_figure
            .find("</figure>")
            .map(|offset| offset + "</figure>".len())
            .unwrap_or(after_figure.len());
        Some(&after_figure[..figure_end])
    }
}

#[path = "html_score_direct_visual_img.rs"]
mod html_score_direct_visual_img;
#[path = "html_score_direct_visual_source.rs"]
mod html_score_direct_visual_source;
#[path = "html_score_direct_visual_svg.rs"]
mod html_score_direct_visual_svg;

#[derive(Clone, Copy)]
struct DirectDiagramCheck {
    label: &'static str,
    kind: &'static str,
    required: bool,
}

impl DirectDiagramCheck {
    fn new(label: &'static str, kind: &'static str, required: bool) -> Self {
        Self {
            label,
            kind,
            required,
        }
    }
}

#[cfg(test)]
#[path = "html_score_direct_visual_helper_tests.rs"]
mod helper_tests;
