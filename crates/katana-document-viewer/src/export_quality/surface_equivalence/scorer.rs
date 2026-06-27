use super::content_scorer::SurfaceContentScorer;
use super::pdf_page_gap::PdfPageGapNormalizer;
use super::{
    BYTE_MAX, DecodedRgbSurface, PERCENT_SCALE, SurfaceEquivalenceImage, SurfaceRgbConverter,
    ZERO_SCORE,
};

pub(super) struct SurfaceSimilarityScorer;

impl SurfaceSimilarityScorer {
    pub(super) fn score(
        viewer: &SurfaceEquivalenceImage<'_>,
        candidate: &DecodedRgbSurface,
        failures: &mut Vec<String>,
    ) -> u8 {
        let viewer_rgb = SurfaceRgbConverter::rgba_to_rgb(viewer.rgba);
        match viewer_rgb {
            Ok(rgb) => Self::score_candidate(viewer, &rgb, candidate, failures),
            Err(error) => {
                failures.push(error);
                ZERO_SCORE
            }
        }
    }

    fn score_candidate(
        viewer: &SurfaceEquivalenceImage<'_>,
        viewer_rgb: &[u8],
        candidate: &DecodedRgbSurface,
        failures: &mut Vec<String>,
    ) -> u8 {
        if viewer.width != candidate.width {
            failures.push("surface dimensions differ".to_string());
            return ZERO_SCORE;
        }
        match Self::candidate_rgb_for_height(viewer, candidate, failures) {
            Some(rgb) => Self::score_rgb(viewer_rgb, &rgb),
            None => ZERO_SCORE,
        }
    }

    fn candidate_rgb_for_height(
        viewer: &SurfaceEquivalenceImage<'_>,
        candidate: &DecodedRgbSurface,
        failures: &mut Vec<String>,
    ) -> Option<Vec<u8>> {
        if viewer.height == candidate.height {
            return Some(candidate.rgb.clone());
        }
        if viewer.height > candidate.height {
            failures.push("surface dimensions differ".to_string());
            return None;
        }
        PdfPageGapNormalizer::normalize(candidate, viewer.height).or_else(|| {
            failures.push("surface dimensions differ".to_string());
            None
        })
    }

    fn score_rgb(reference: &[u8], candidate: &[u8]) -> u8 {
        if reference.is_empty() || reference.len() != candidate.len() {
            return ZERO_SCORE;
        }
        let average_score = Self::average_score(reference, candidate);
        let content_score = SurfaceContentScorer::score(reference, candidate);
        average_score.min(content_score)
    }

    fn average_score(reference: &[u8], candidate: &[u8]) -> u8 {
        let total_diff = reference
            .iter()
            .zip(candidate.iter())
            .map(|(left, right)| left.abs_diff(*right) as u64)
            .sum::<u64>();
        let max_diff = reference.len() as f64 * BYTE_MAX;
        let distance = total_diff as f64 / max_diff;
        (PERCENT_SCALE - distance * PERCENT_SCALE).round() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidate_shorter_than_viewer_reports_dimension_failure() {
        let viewer = SurfaceEquivalenceImage {
            width: 1,
            height: 2,
            rgba: &[255, 255, 255, 255, 255, 255, 255, 255],
        };
        let candidate = DecodedRgbSurface {
            width: 1,
            height: 1,
            rgb: vec![255, 255, 255],
        };
        let mut failures = Vec::new();

        let score = SurfaceSimilarityScorer::score(&viewer, &candidate, &mut failures);

        assert_eq!(ZERO_SCORE, score);
        assert_eq!(vec!["surface dimensions differ".to_string()], failures);
    }

    #[test]
    fn score_rgb_rejects_mismatched_lengths() {
        assert_eq!(
            ZERO_SCORE,
            SurfaceSimilarityScorer::score_rgb(&[1, 2], &[1])
        );
    }

    #[test]
    fn score_rgb_rejects_small_reference_content_loss() {
        let reference = surface_with_black_square();
        let candidate = white_surface(TEST_SURFACE_PIXELS);

        let score = SurfaceSimilarityScorer::score_rgb(&reference, &candidate);

        assert_eq!(ZERO_SCORE, score);
    }

    #[test]
    fn score_rgb_rejects_small_extra_candidate_content() {
        let reference = white_surface(TEST_SURFACE_PIXELS);
        let candidate = surface_with_black_square();

        let score = SurfaceSimilarityScorer::score_rgb(&reference, &candidate);

        assert_eq!(ZERO_SCORE, score);
    }

    const TEST_SURFACE_PIXELS: usize = 100;

    fn white_surface(pixel_count: usize) -> Vec<u8> {
        let mut rgb = Vec::with_capacity(pixel_count * 3);
        for _ in 0..pixel_count {
            rgb.extend_from_slice(&[255, 255, 255]);
        }
        rgb
    }

    fn surface_with_black_square() -> Vec<u8> {
        let mut rgb = white_surface(TEST_SURFACE_PIXELS);
        for pixel_index in 0..4 {
            let offset = pixel_index * 3;
            rgb[offset..offset + 3].copy_from_slice(&[0, 0, 0]);
        }
        rgb
    }
}
