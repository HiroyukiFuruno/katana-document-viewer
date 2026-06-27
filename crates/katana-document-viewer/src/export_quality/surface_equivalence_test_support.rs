use super::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use image::ImageEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use std::io::Write;

const FIRST_PDF_IMAGE_OBJECT_NUMBER: usize = 5;
const JPEG_TEST_QUALITY: u8 = 95;

pub(super) fn combined_rgba(first: &[u8], second: &[u8]) -> Vec<u8> {
    let mut rgba = first.to_vec();
    rgba.extend_from_slice(second);
    rgba
}

pub(super) fn surface_image(rgba: &[u8], width: u32, height: u32) -> SurfaceEquivalenceImage<'_> {
    SurfaceEquivalenceImage {
        width,
        height,
        rgba,
    }
}

pub(in crate::export_quality) fn repeated_rgba(
    pixel: [u8; RGBA_CHANNELS],
    width: u32,
    height: u32,
) -> Vec<u8> {
    let pixels = width as usize * height as usize;
    let mut rgba = Vec::with_capacity(pixels * RGBA_CHANNELS);
    for _ in 0..pixels {
        rgba.extend_from_slice(&pixel);
    }
    rgba
}

pub(in crate::export_quality) fn encode_png(
    rgba: &[u8],
    width: u32,
    height: u32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    PngEncoder::new(&mut bytes).write_image(rgba, width, height, image::ColorType::Rgba8.into())?;
    Ok(bytes)
}

pub(in crate::export_quality) fn encode_jpeg(
    rgba: &[u8],
    width: u32,
    height: u32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let rgb = SurfaceRgbConverter::rgba_to_rgb(rgba)?;
    let mut bytes = Vec::new();
    JpegEncoder::new_with_quality(&mut bytes, JPEG_TEST_QUALITY).write_image(
        &rgb,
        width,
        height,
        image::ColorType::Rgb8.into(),
    )?;
    Ok(bytes)
}

pub(in crate::export_quality) fn fake_pdf(
    pages: &[(&[u8], u32, u32)],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut bytes = b"%PDF-1.4\n".to_vec();
    for (index, (rgba, width, height)) in pages.iter().enumerate() {
        let object_number = index + FIRST_PDF_IMAGE_OBJECT_NUMBER;
        let stream = compressed_rgb(rgba)?;
        bytes.extend_from_slice(
            format!(
                "{object_number} 0 obj\n<< /Type /XObject /Subtype /Image /Width {width} /Height {height} /Filter /FlateDecode >>\nstream\n"
            )
            .as_bytes(),
        );
        bytes.extend_from_slice(&stream);
        bytes.extend_from_slice(b"\nendstream\nendobj\n");
    }
    Ok(bytes)
}

fn compressed_rgb(rgba: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let rgb = SurfaceRgbConverter::rgba_to_rgb(rgba)?;
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&rgb)?;
    Ok(encoder.finish()?)
}
