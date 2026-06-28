use crate::export_quality::visual_content_stats::RgbVisualContentStats;

use super::pdf_decoder::PdfSurfaceDecoder;
use super::{RGB_CHANNELS, decoded_surface::DecodedRgbSurface};

pub(crate) struct PdfVisualContent;

impl PdfVisualContent {
    pub(crate) fn has_visible_content(bytes: &[u8]) -> bool {
        PdfSurfaceDecoder::decode(bytes)
            .is_ok_and(|surface| Self::stats_from_surface(&surface).has_visible_content())
    }

    fn stats_from_surface(surface: &DecodedRgbSurface) -> RgbVisualContentStats {
        let mut stats = RgbVisualContentStats::empty();
        for pixel in surface.rgb.chunks_exact(RGB_CHANNELS) {
            stats.observe([pixel[0], pixel[1], pixel[2]]);
        }
        stats
    }
}
