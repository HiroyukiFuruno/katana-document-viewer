use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::Write;

const PDF_IMAGE_WIDTH: u32 = 640;
const PDF_IMAGE_HEIGHT: u32 = 480;
const RGB_CHANNELS: usize = 3;
const BACKGROUND_CHANNEL: u8 = 255;
const TEXT_CHANNEL: u8 = 24;

pub(crate) struct PostprocessPdfFixture;

impl PostprocessPdfFixture {
    pub(crate) fn base_pdf() -> Vec<u8> {
        let mut bytes = format!(
            concat!(
                "%PDF-1.4\n",
                "1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n",
                "2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n",
                "3 0 obj << /Type /Page /Parent 2 0 R /Resources << ",
                "/XObject << /Im5 5 0 R >> >> /Annots [6 0 R 7 0 R] >> endobj\n",
                "5 0 obj << /Type /XObject /Subtype /Image /Width {} ",
                "/Height {} /Filter /FlateDecode >> stream\n",
            ),
            PDF_IMAGE_WIDTH, PDF_IMAGE_HEIGHT
        )
        .into_bytes();
        bytes.extend_from_slice(&Self::compressed_image_stream());
        bytes.extend_from_slice(
            concat!(
                "\nendstream endobj\n",
                "6 0 obj << /Type /Annot /Subtype /Link /Dest [3 0 R /XYZ 0 0 null] >> endobj\n",
                "7 0 obj << /Type /Annot /Subtype /Link /A << /S /URI ",
                "/URI (https://example.com) >> >> endobj\n",
                "%%EOF\n"
            )
            .as_bytes(),
        );
        bytes
    }

    fn compressed_image_stream() -> Vec<u8> {
        let rgb = Self::text_like_rgb();
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        if let Err(error) = encoder.write_all(&rgb) {
            return format!("KDV_PDF_FIXTURE_COMPRESSION_WRITE_ERROR:{error}").into_bytes();
        }
        match encoder.finish() {
            Ok(bytes) => bytes,
            Err(error) => format!("KDV_PDF_FIXTURE_COMPRESSION_FINISH_ERROR:{error}").into_bytes(),
        }
    }

    fn text_like_rgb() -> Vec<u8> {
        let mut rgb = vec![BACKGROUND_CHANNEL; Self::rgb_len()];
        for y in 80..96 {
            for x in 72..260 {
                let offset = Self::rgb_offset(x, y);
                rgb[offset..offset + RGB_CHANNELS].fill(TEXT_CHANNEL);
            }
        }
        rgb
    }

    fn rgb_len() -> usize {
        (PDF_IMAGE_WIDTH * PDF_IMAGE_HEIGHT) as usize * RGB_CHANNELS
    }

    fn rgb_offset(x: u32, y: u32) -> usize {
        ((y * PDF_IMAGE_WIDTH + x) as usize) * RGB_CHANNELS
    }
}
