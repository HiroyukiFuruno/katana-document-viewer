use super::image::SurfaceImageBlock;
use crate::export_surface_helpers::SURFACE_CONTENT_WIDTH;
use crate::export_surface_svg::SurfaceSvgRasterizer;
#[cfg(test)]
use image::RgbaImage;

const SVG_DATA_PREFIXES: [&str; 3] = [
    "data:image/svg+xml,",
    "data:image/svg+xml;charset=utf-8,",
    "data:image/svg+xml;utf8,",
];
const PERCENT_ESCAPE_BYTE_LEN: usize = 3;
const HEX_HIGH_NIBBLE_MULTIPLIER: u8 = 16;
const ASCII_HEX_ALPHA_OFFSET: u8 = 10;

impl SurfaceImageBlock {
    pub(crate) fn from_data_uri(
        src: &str,
        requested_width: Option<u32>,
        alt: String,
    ) -> Option<Self> {
        let svg = svg_payload(src)?;
        let max_width = requested_width.unwrap_or(SURFACE_CONTENT_WIDTH);
        let rendered = SurfaceSvgRasterizer::rasterize_for_export_surface(&svg, max_width)?;
        Some(Self {
            display_width: rendered.display_width_px(),
            display_height: rendered.display_height_px(),
            image: rendered.image,
            _alt: alt,
        })
    }
}

fn svg_payload(src: &str) -> Option<String> {
    for prefix in SVG_DATA_PREFIXES {
        if src.len() >= prefix.len() && src[..prefix.len()].eq_ignore_ascii_case(prefix) {
            return percent_decode(&src[prefix.len()..]);
        }
    }
    None
}

fn percent_decode(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && index + 2 < bytes.len()
            && let Some(decoded) = hex_byte(bytes[index + 1], bytes[index + 2])
        {
            output.push(decoded);
            index += PERCENT_ESCAPE_BYTE_LEN;
            continue;
        }
        output.push(bytes[index]);
        index += 1;
    }
    String::from_utf8(output).ok()
}

fn hex_byte(high: u8, low: u8) -> Option<u8> {
    Some(hex_digit(high)? * HEX_HIGH_NIBBLE_MULTIPLIER + hex_digit(low)?)
}

fn hex_digit(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + ASCII_HEX_ALPHA_OFFSET),
        b'A'..=b'F' => Some(value - b'A' + ASCII_HEX_ALPHA_OFFSET),
        _ => None,
    }
}

#[cfg(test)]
fn scaled_image(image: RgbaImage, requested_width: Option<u32>) -> RgbaImage {
    let max_width = requested_width
        .unwrap_or(image.width())
        .min(SURFACE_CONTENT_WIDTH);
    if image.width() <= max_width {
        return image;
    }
    let height = (image.height() as f32 * max_width as f32 / image.width() as f32)
        .round()
        .max(1.0) as u32;
    image::imageops::resize(
        &image,
        max_width,
        height,
        image::imageops::FilterType::Lanczos3,
    )
}

#[cfg(test)]
#[path = "export_surface_blocks_data_image_tests.rs"]
mod tests;
