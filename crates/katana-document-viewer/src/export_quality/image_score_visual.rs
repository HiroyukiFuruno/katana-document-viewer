use crate::export_quality::visual_content_stats::RgbVisualContentStats;

const VISIBLE_ALPHA_MIN: u8 = 8;

pub(super) struct ImageVisualContent;

impl ImageVisualContent {
    pub(super) fn has_visible_content(bytes: &[u8]) -> bool {
        image::load_from_memory(bytes).is_ok_and(|image| {
            let rgba = image.to_rgba8();
            Self::stats_from_image(&rgba).has_visible_content()
        })
    }

    fn stats_from_image(image: &image::RgbaImage) -> RgbVisualContentStats {
        let mut stats = RgbVisualContentStats::empty();
        for pixel in image.pixels() {
            Self::observe_visible_pixel(&mut stats, pixel.0);
        }
        stats
    }

    fn observe_visible_pixel(stats: &mut RgbVisualContentStats, pixel: [u8; 4]) {
        if pixel[3] <= VISIBLE_ALPHA_MIN {
            return;
        }
        stats.observe([pixel[0], pixel[1], pixel[2]]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_image_bytes_has_no_visible_content() {
        assert!(!ImageVisualContent::has_visible_content(b"not-a-png"));
    }

    #[test]
    fn transparent_pixels_are_ignored_for_visible_content() {
        let mut stats = RgbVisualContentStats::empty();
        ImageVisualContent::observe_visible_pixel(&mut stats, [0, 0, 0, 0]);

        assert!(!stats.has_visible_content());
    }
}
