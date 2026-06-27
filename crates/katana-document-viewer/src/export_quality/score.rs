use crate::export_quality::html_score::HtmlQualityScore;
use crate::export_quality::image_score::ImageQualityScore;
use crate::export_quality::markdown_link_source::MarkdownLinkSource;
use crate::export_quality::surface_equivalence::{
    PdfVisualContent, SurfaceEquivalenceGate, SurfaceEquivalenceReport,
};
use crate::export_quality::types::{
    ExportFormatQualityScore, ExportQualityArtifacts, ExportQualityCheck, ExportQualityReport,
    TOTAL_SCORE_MAX, check,
};
use crate::forge::ExportFormat;

pub struct ExportQualityGate;

impl ExportQualityGate {
    pub fn evaluate(artifacts: &ExportQualityArtifacts<'_>) -> ExportQualityReport {
        let format_scores = Self::format_scores(artifacts);
        let surface_report = artifacts
            .surface_equivalence
            .map(|surface| SurfaceEquivalenceGate::evaluate(&surface));
        let fatal_failures = Self::fatal_failures(&format_scores, &surface_report);
        let warnings = Self::warnings(&format_scores, &surface_report);
        let total_score = format_scores.iter().map(|score| score.score).sum();
        ExportQualityReport {
            total_score,
            max_score: TOTAL_SCORE_MAX,
            format_scores,
            fatal_failures,
            warnings,
        }
    }

    fn format_scores(artifacts: &ExportQualityArtifacts<'_>) -> Vec<ExportFormatQualityScore> {
        vec![
            Self::score_html(artifacts.html, artifacts.source_markdown),
            Self::score_pdf(artifacts.pdf, artifacts.source_markdown),
            Self::score_png(artifacts.png),
            Self::score_jpeg(artifacts.jpeg),
        ]
    }

    fn fatal_failures(
        format_scores: &[ExportFormatQualityScore],
        surface_report: &Option<SurfaceEquivalenceReport>,
    ) -> Vec<String> {
        let mut fatal_failures = format_scores
            .iter()
            .flat_map(|score| score.fatal_failures())
            .collect::<Vec<_>>();
        if let Some(surface) = surface_report
            && !surface.is_pass()
        {
            fatal_failures.extend(
                surface
                    .failures
                    .iter()
                    .map(|failure| format!("Surface: {failure}")),
            );
        }
        fatal_failures
    }

    fn warnings(
        format_scores: &[ExportFormatQualityScore],
        surface_report: &Option<SurfaceEquivalenceReport>,
    ) -> Vec<String> {
        let mut warnings = format_scores
            .iter()
            .filter(|score| !score.is_pass())
            .map(|score| format!("{:?} quality score is {}/100", score.format, score.score))
            .collect::<Vec<_>>();
        if let Some(surface) = surface_report
            && !surface.is_pass()
        {
            warnings.push(format!(
                "surface equivalence score is {}/100",
                surface.minimum_score
            ));
        }
        warnings
    }

    fn score_html(bytes: &[u8], source_markdown: &str) -> ExportFormatQualityScore {
        HtmlQualityScore::score(bytes, source_markdown)
    }

    fn score_pdf(bytes: &[u8], source_markdown: &str) -> ExportFormatQualityScore {
        let text = String::from_utf8_lossy(bytes);
        ExportFormatQualityScore::new(
            ExportFormat::Pdf,
            Self::pdf_checks(bytes, source_markdown, &text),
        )
    }

    fn pdf_checks(bytes: &[u8], source_markdown: &str, text: &str) -> Vec<ExportQualityCheck> {
        [
            ("pdf is non-empty", !bytes.is_empty(), 15),
            ("pdf has header", bytes.starts_with(b"%PDF-1.4"), 15),
            (
                "pdf embeds page image",
                text.contains("/Subtype /Image"),
                20,
            ),
            (
                "pdf has page tree",
                text.contains("/Type /Pages") && text.contains("/Type /Page"),
                15,
            ),
            (
                "pdf keeps link annotations",
                !requires_link_annotation(source_markdown) || text.contains("/Subtype /Link"),
                15,
            ),
            (
                "pdf image stream is not visually blank",
                PdfVisualContent::has_visible_content(bytes),
                20,
            ),
        ]
        .into_iter()
        .map(|(name, passed, points)| check(name, passed, true, points))
        .collect()
    }

    fn score_png(bytes: &[u8]) -> ExportFormatQualityScore {
        let decoded = ImageQualityScore::decode_dimensions(bytes);
        Self::score_image(ExportFormat::Png, bytes, decoded, b"\x89PNG\r\n\x1a\n")
    }

    fn score_jpeg(bytes: &[u8]) -> ExportFormatQualityScore {
        let decoded = ImageQualityScore::decode_dimensions(bytes);
        Self::score_image(ExportFormat::Jpeg, bytes, decoded, b"\xff\xd8\xff")
    }

    fn score_image(
        format: ExportFormat,
        bytes: &[u8],
        decoded: Result<(u32, u32), image::ImageError>,
        signature: &[u8],
    ) -> ExportFormatQualityScore {
        ImageQualityScore::score(format, bytes, decoded, signature)
    }
}

fn requires_link_annotation(source_markdown: &str) -> bool {
    MarkdownLinkSource::contains_markdown_link(source_markdown)
        || source_contains_html_anchor(source_markdown)
}

fn source_contains_html_anchor(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    let mut rest = lower.as_str();
    while let Some(anchor_start) = rest.find("<a") {
        let after_anchor = &rest[anchor_start + 2..];
        let Some(tag_end) = after_anchor.find('>') else {
            return false;
        };
        let tag = &after_anchor[..tag_end];
        if tag.starts_with(char::is_whitespace) && tag_has_href_attribute(tag) {
            return true;
        }
        rest = &after_anchor[tag_end..];
    }
    false
}

fn tag_has_href_attribute(tag: &str) -> bool {
    let mut rest = tag;
    while let Some(href_start) = rest.find("href") {
        let before = href_start
            .checked_sub(1)
            .and_then(|index| rest.as_bytes().get(index))
            .copied();
        let after = rest[href_start + "href".len()..].trim_start();
        if before.is_some_and(|value| value.is_ascii_whitespace()) && after.starts_with('=') {
            return true;
        }
        rest = &rest[href_start + "href".len()..];
    }
    false
}

#[cfg(test)]
#[path = "score_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "score_pdf_test_support.rs"]
mod score_pdf_test_support;

#[cfg(test)]
#[path = "score_pdf_tests.rs"]
mod pdf_tests;

#[cfg(test)]
#[path = "score_pdf_markdown_link_tests.rs"]
mod pdf_markdown_link_tests;
