use crate::export_quality::types::{ExportFormatQualityScore, ExportQualityCheck, check};
use crate::forge::ExportFormat;
use image::GenericImageView;
use image_score_visual::ImageVisualContent;

const DOCUMENT_MIN_WIDTH: u32 = 640;
const DOCUMENT_MIN_HEIGHT: u32 = 480;
const NON_EMPTY_SCORE_WEIGHT: u16 = 20;
const SIGNATURE_SCORE_WEIGHT: u16 = 20;
const DECODE_SCORE_WEIGHT: u16 = 25;
const SCALE_SCORE_WEIGHT: u16 = 25;
const VISUAL_CONTENT_SCORE_WEIGHT: u16 = 10;

pub(crate) struct ImageQualityScore;

impl ImageQualityScore {
    pub(crate) fn score(
        format: ExportFormat,
        bytes: &[u8],
        decoded: Result<(u32, u32), image::ImageError>,
        signature: &[u8],
    ) -> ExportFormatQualityScore {
        let label = format!("{format:?}").to_lowercase();
        ExportFormatQualityScore::new(format, Self::checks(&label, bytes, &decoded, signature))
    }

    pub(crate) fn decode_dimensions(bytes: &[u8]) -> Result<(u32, u32), image::ImageError> {
        image::load_from_memory(bytes).map(|image| image.dimensions())
    }

    fn checks(
        label: &str,
        bytes: &[u8],
        decoded: &Result<(u32, u32), image::ImageError>,
        signature: &[u8],
    ) -> Vec<ExportQualityCheck> {
        vec![
            Self::non_empty_check(label, bytes),
            Self::signature_check(label, bytes, signature),
            Self::decode_check(label, decoded),
            Self::scale_check(label, decoded),
            Self::visual_content_check(label, bytes),
        ]
    }

    fn non_empty_check(label: &str, bytes: &[u8]) -> ExportQualityCheck {
        check(
            &format!("{label} is non-empty"),
            !bytes.is_empty(),
            true,
            NON_EMPTY_SCORE_WEIGHT,
        )
    }

    fn signature_check(label: &str, bytes: &[u8], signature: &[u8]) -> ExportQualityCheck {
        check(
            &format!("{label} has signature"),
            bytes.starts_with(signature),
            true,
            SIGNATURE_SCORE_WEIGHT,
        )
    }

    fn decode_check(
        label: &str,
        decoded: &Result<(u32, u32), image::ImageError>,
    ) -> ExportQualityCheck {
        check(
            &format!("{label} decodes"),
            decoded.is_ok(),
            true,
            DECODE_SCORE_WEIGHT,
        )
    }

    fn scale_check(
        label: &str,
        decoded: &Result<(u32, u32), image::ImageError>,
    ) -> ExportQualityCheck {
        check(
            &format!("{label} dimensions are document scale"),
            document_scale(decoded),
            true,
            SCALE_SCORE_WEIGHT,
        )
    }

    fn visual_content_check(label: &str, bytes: &[u8]) -> ExportQualityCheck {
        check(
            &format!("{label} is not visually blank"),
            ImageVisualContent::has_visible_content(bytes),
            true,
            VISUAL_CONTENT_SCORE_WEIGHT,
        )
    }
}

fn document_scale(decoded: &Result<(u32, u32), image::ImageError>) -> bool {
    decoded
        .as_ref()
        .is_ok_and(|(width, height)| *width >= DOCUMENT_MIN_WIDTH && *height >= DOCUMENT_MIN_HEIGHT)
}

#[cfg(test)]
#[path = "image_score_tests.rs"]
mod tests;

#[path = "image_score_visual.rs"]
mod image_score_visual;
