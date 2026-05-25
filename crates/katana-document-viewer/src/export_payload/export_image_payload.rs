use crate::export_surface::DocumentSurface;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};

pub(crate) struct ImageExportPayloadFactory;

impl ImageExportPayloadFactory {
    pub(crate) fn create_png(surface: &DocumentSurface) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();
        let encoder = PngEncoder::new(&mut bytes);
        encoder
            .write_image(
                surface.image.as_raw(),
                surface.image.width(),
                surface.image.height(),
                ColorType::Rgba8.into(),
            )
            .map_err(|error| format!("PNG encoding failed: {error}"))?;
        Ok(bytes)
    }

    pub(crate) fn create_jpeg(surface: &DocumentSurface) -> Result<Vec<u8>, String> {
        let mut rgb = Vec::with_capacity(
            (surface.image.width() * surface.image.height() * RGB_CHANNELS_PER_PIXEL) as usize,
        );
        for pixel in surface.image.pixels() {
            rgb.extend_from_slice(&[pixel[0], pixel[1], pixel[2]]);
        }
        let mut bytes = Vec::new();
        let mut encoder = JpegEncoder::new_with_quality(&mut bytes, JPEG_QUALITY);
        encoder
            .encode(
                &rgb,
                surface.image.width(),
                surface.image.height(),
                ColorType::Rgb8.into(),
            )
            .map_err(|error| format!("JPEG encoding failed: {error}"))?;
        Ok(bytes)
    }
}

const RGB_CHANNELS_PER_PIXEL: u32 = 3;
const JPEG_QUALITY: u8 = 90;
