use super::*;
use crate::export_quality::ExportQualityArtifacts;
use image::ImageEncoder;

const FIXTURE_IMAGE_WIDTH: u32 = 700;
const FIXTURE_IMAGE_HEIGHT: u32 = 500;
const RGB_CHANNEL_VALUE: u8 = 240;
const DEFAULT_POSTPROCESS_MILLIS: u128 = 20;
const BASELINE_PDF_GENERATION_MILLIS: u128 = 100;
const TEST_METADATA_PADDING_BYTES: usize = 2048;

#[derive(Clone)]
pub(crate) struct FixtureArtifacts {
    pub(crate) pdf: Vec<u8>,
    html: Vec<u8>,
    png: Vec<u8>,
    jpeg: Vec<u8>,
}

impl FixtureArtifacts {
    pub(crate) fn new() -> Result<Self, image::ImageError> {
        Ok(Self {
            html: Self::html(),
            pdf: Self::base_pdf(),
            png: png_bytes(FIXTURE_IMAGE_WIDTH, FIXTURE_IMAGE_HEIGHT)?,
            jpeg: jpeg_bytes(FIXTURE_IMAGE_WIDTH, FIXTURE_IMAGE_HEIGHT)?,
        })
    }

    pub(crate) fn with_test_metadata() -> Result<Self, image::ImageError> {
        let mut fixture = Self::new()?;
        fixture
            .pdf
            .extend_from_slice(&vec![b' '; TEST_METADATA_PADDING_BYTES]);
        Ok(fixture)
    }

    pub(crate) fn request(&self) -> ExportPostprocessEvaluationRequest<'_> {
        ExportPostprocessEvaluationRequest {
            artifacts: ExportQualityArtifacts {
                html: &self.html,
                pdf: &self.pdf,
                png: &self.png,
                jpeg: &self.jpeg,
            },
            baseline_pdf_generation_millis: BASELINE_PDF_GENERATION_MILLIS,
        }
    }

    pub(crate) fn optimized_pdf(&self) -> Vec<u8> {
        Self::base_pdf()
    }

    pub(crate) fn pdf_without_link_annotation(&self) -> Vec<u8> {
        String::from_utf8_lossy(&self.pdf)
            .replace("/Subtype /Link", "/Subtype /Text")
            .into_bytes()
    }

    pub(crate) fn pdf_with_one_byte_removed(&self) -> Vec<u8> {
        self.pdf[..self.pdf.len() - 1].to_vec()
    }

    fn html() -> Vec<u8> {
        concat!(
            "<main data-kdv-export data-kdv-export-style>",
            "<strong>太字</strong><a href=\"https://example.com\">link</a>",
            "<aside data-github-alert=\"warning\"></aside>",
            "<span data-kdv-task-state=\"done\"></span>",
            "<svg data-kdv-render-runtime=\"katana-render-runtime\"></svg>",
            "</main>"
        )
        .as_bytes()
        .to_vec()
    }

    pub(crate) fn base_pdf() -> Vec<u8> {
        concat!(
            "%PDF-1.4\n",
            "1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n",
            "2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n",
            "3 0 obj << /Type /Page /Parent 2 0 R /Resources << ",
            "/XObject << /Im5 5 0 R >> >> /Annots [6 0 R 7 0 R] >> endobj\n",
            "5 0 obj << /Type /XObject /Subtype /Image >> stream\nabc\nendstream endobj\n",
            "6 0 obj << /Type /Annot /Subtype /Link /Dest [3 0 R /XYZ 0 0 null] >> endobj\n",
            "7 0 obj << /Type /Annot /Subtype /Link /A << /S /URI ",
            "/URI (https://example.com) >> >> endobj\n",
            "%%EOF\n"
        )
        .as_bytes()
        .to_vec()
    }
}

pub(crate) struct StaticPostprocessAdapter {
    adapter_name: String,
    result: Result<Vec<u8>, PdfPostprocessError>,
    elapsed_millis: u128,
}

impl StaticPostprocessAdapter {
    pub(crate) fn success(adapter_name: &str, pdf: Vec<u8>) -> Self {
        Self {
            adapter_name: adapter_name.to_string(),
            result: Ok(pdf),
            elapsed_millis: DEFAULT_POSTPROCESS_MILLIS,
        }
    }

    pub(crate) fn failure(adapter_name: &str, message: &str) -> Self {
        Self {
            adapter_name: adapter_name.to_string(),
            result: Err(PdfPostprocessError::new(message)),
            elapsed_millis: DEFAULT_POSTPROCESS_MILLIS,
        }
    }

    pub(crate) fn with_elapsed_millis(mut self, elapsed_millis: u128) -> Self {
        self.elapsed_millis = elapsed_millis;
        self
    }
}

impl PdfPostprocessAdapter for StaticPostprocessAdapter {
    fn name(&self) -> &str {
        &self.adapter_name
    }

    fn postprocess_pdf(
        &self,
        _input: &PdfPostprocessInput<'_>,
    ) -> Result<PdfPostprocessOutput, PdfPostprocessError> {
        self.result.clone().map(|pdf| PdfPostprocessOutput {
            pdf,
            elapsed_millis: self.elapsed_millis,
        })
    }
}

fn png_bytes(width: u32, height: u32) -> Result<Vec<u8>, image::ImageError> {
    let image = image::RgbImage::from_pixel(
        width,
        height,
        image::Rgb([RGB_CHANNEL_VALUE, RGB_CHANNEL_VALUE, RGB_CHANNEL_VALUE]),
    );
    let mut bytes = Vec::new();
    let encode_result = image::codecs::png::PngEncoder::new(&mut bytes).write_image(
        image.as_raw(),
        width,
        height,
        image::ColorType::Rgb8.into(),
    );
    assert!(encode_result.is_ok(), "png fixture encode failed");
    Ok(bytes)
}

fn jpeg_bytes(width: u32, height: u32) -> Result<Vec<u8>, image::ImageError> {
    let image = image::RgbImage::from_pixel(
        width,
        height,
        image::Rgb([RGB_CHANNEL_VALUE, RGB_CHANNEL_VALUE, RGB_CHANNEL_VALUE]),
    );
    let mut bytes = Vec::new();
    let encode_result = image::codecs::jpeg::JpegEncoder::new(&mut bytes).write_image(
        image.as_raw(),
        width,
        height,
        image::ColorType::Rgb8.into(),
    );
    assert!(encode_result.is_ok(), "jpeg fixture encode failed");
    Ok(bytes)
}
