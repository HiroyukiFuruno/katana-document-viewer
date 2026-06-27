mod byte_search;
mod content_scorer;
mod decoded_surface;
mod pdf_decoder;
mod pdf_image_streams;
mod pdf_page_gap;
mod pdf_visual_content;
mod raster_decoder;
mod rgb_converter;
mod scorer;

use pdf_decoder::PdfSurfaceDecoder;
pub(crate) use pdf_visual_content::PdfVisualContent;
use raster_decoder::RasterSurfaceDecoder;
use scorer::SurfaceSimilarityScorer;

use byte_search::ByteSearch;
use decoded_surface::DecodedRgbSurface;
use pdf_image_streams::{PdfImagePage, PdfImageStreams};
use rgb_converter::SurfaceRgbConverter;

const SURFACE_EQUIVALENCE_THRESHOLD: u8 = 95;
const PERFECT_SCORE: u8 = 100;
const ZERO_SCORE: u8 = 0;
const PERCENT_SCALE: f64 = 100.0;
const BYTE_MAX: f64 = 255.0;
const RGB_CHANNELS: usize = 3;
const RGBA_CHANNELS: usize = 4;
const PDF_IMAGE_MARKER: &[u8] = b"/Subtype /Image";
const PDF_DICTIONARY_START: &[u8] = b"<<";
const PDF_STREAM_START: &[u8] = b"stream\n";
const PDF_STREAM_END: &[u8] = b"\nendstream";
const PDF_WIDTH_KEY: &[u8] = b"/Width";
const PDF_HEIGHT_KEY: &[u8] = b"/Height";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceEquivalenceImage<'a> {
    pub width: u32,
    pub height: u32,
    pub rgba: &'a [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceEquivalenceArtifacts<'a> {
    pub raster_reference: SurfaceEquivalenceImage<'a>,
    pub pdf_reference: SurfaceEquivalenceImage<'a>,
    pub pdf: &'a [u8],
    pub png: &'a [u8],
    pub jpeg: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceEquivalenceReport {
    pub pdf_score: u8,
    pub png_score: u8,
    pub jpeg_score: u8,
    pub minimum_score: u8,
    pub threshold: u8,
    pub failures: Vec<String>,
}

impl SurfaceEquivalenceReport {
    pub fn is_pass(&self) -> bool {
        self.minimum_score >= self.threshold && self.failures.is_empty()
    }
}

pub struct SurfaceEquivalenceGate;

impl SurfaceEquivalenceGate {
    pub fn evaluate(artifacts: &SurfaceEquivalenceArtifacts<'_>) -> SurfaceEquivalenceReport {
        let mut failures = Vec::new();
        let png_score = Self::raster_score("png", artifacts.png, artifacts, &mut failures);
        let jpeg_score = Self::raster_score("jpeg", artifacts.jpeg, artifacts, &mut failures);
        let pdf_score = Self::pdf_score(artifacts, &mut failures);
        let minimum_score = png_score.min(pdf_score).min(jpeg_score);
        Self::push_threshold_failure("surface equivalence", minimum_score, &mut failures);
        SurfaceEquivalenceReport {
            pdf_score,
            png_score,
            jpeg_score,
            minimum_score,
            threshold: SURFACE_EQUIVALENCE_THRESHOLD,
            failures,
        }
    }

    fn raster_score(
        label: &str,
        bytes: &[u8],
        artifacts: &SurfaceEquivalenceArtifacts<'_>,
        failures: &mut Vec<String>,
    ) -> u8 {
        let decoded = RasterSurfaceDecoder::decode(bytes);
        match decoded {
            Ok(surface) => {
                SurfaceSimilarityScorer::score(&artifacts.raster_reference, &surface, failures)
            }
            Err(error) => {
                failures.push(format!("{label} decode failed: {error}"));
                ZERO_SCORE
            }
        }
    }

    fn pdf_score(artifacts: &SurfaceEquivalenceArtifacts<'_>, failures: &mut Vec<String>) -> u8 {
        let decoded = PdfSurfaceDecoder::decode(artifacts.pdf);
        match decoded {
            Ok(surface) => {
                SurfaceSimilarityScorer::score(&artifacts.pdf_reference, &surface, failures)
            }
            Err(error) => {
                failures.push(format!("pdf decode failed: {error}"));
                ZERO_SCORE
            }
        }
    }

    fn push_threshold_failure(label: &str, score: u8, failures: &mut Vec<String>) {
        if score >= SURFACE_EQUIVALENCE_THRESHOLD {
            return;
        }
        failures.push(format!(
            "{label} score is {score}/{PERFECT_SCORE}, required {SURFACE_EQUIVALENCE_THRESHOLD}"
        ));
    }
}

#[cfg(test)]
#[path = "surface_equivalence_test_support.rs"]
pub(super) mod test_support;

#[cfg(test)]
#[path = "surface_equivalence_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "surface_equivalence_failure_tests.rs"]
mod failure_tests;
