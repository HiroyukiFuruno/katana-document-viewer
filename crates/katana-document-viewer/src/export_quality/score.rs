use crate::export_quality::html_score::HtmlQualityScore;
use crate::export_quality::image_score::ImageQualityScore;
use crate::export_quality::types::{
    ExportFormatQualityScore, ExportQualityArtifacts, ExportQualityReport, TOTAL_SCORE_MAX, check,
};
use crate::forge::ExportFormat;

pub struct ExportQualityGate;

impl ExportQualityGate {
    pub fn evaluate(artifacts: &ExportQualityArtifacts<'_>) -> ExportQualityReport {
        let format_scores = vec![
            Self::score_html(artifacts.html),
            Self::score_pdf(artifacts.pdf),
            Self::score_png(artifacts.png),
            Self::score_jpeg(artifacts.jpeg),
        ];
        let fatal_failures = format_scores
            .iter()
            .flat_map(|score| score.fatal_failures())
            .collect::<Vec<_>>();
        let warnings = format_scores
            .iter()
            .filter(|score| !score.is_pass())
            .map(|score| format!("{:?} quality score is {}/100", score.format, score.score))
            .collect::<Vec<_>>();
        let total_score = format_scores.iter().map(|score| score.score).sum();
        ExportQualityReport {
            total_score,
            max_score: TOTAL_SCORE_MAX,
            format_scores,
            fatal_failures,
            warnings,
        }
    }

    fn score_html(bytes: &[u8]) -> ExportFormatQualityScore {
        HtmlQualityScore::score(bytes)
    }

    fn score_pdf(bytes: &[u8]) -> ExportFormatQualityScore {
        let text = String::from_utf8_lossy(bytes);
        ExportFormatQualityScore::new(
            ExportFormat::Pdf,
            vec![
                check("pdf is non-empty", !bytes.is_empty(), true, 20),
                check("pdf has header", bytes.starts_with(b"%PDF-1.4"), true, 20),
                check(
                    "pdf embeds page image",
                    text.contains("/Subtype /Image"),
                    true,
                    20,
                ),
                check(
                    "pdf has page tree",
                    text.contains("/Type /Pages") && text.contains("/Type /Page"),
                    true,
                    20,
                ),
                check(
                    "pdf keeps link annotations",
                    text.contains("/Subtype /Link"),
                    true,
                    20,
                ),
            ],
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_reports_warnings_for_failed_formats() {
        let artifacts = ExportQualityArtifacts {
            html: b"",
            pdf: b"",
            png: b"",
            jpeg: b"",
        };

        let report = ExportQualityGate::evaluate(&artifacts);

        assert!(!report.is_pass());
        assert!(
            report
                .warnings
                .iter()
                .any(|warning| warning.contains("Html"))
        );
        assert!(!report.fatal_failures.is_empty());
    }
}
