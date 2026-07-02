use super::{SurfaceImageBlock, hex_byte, hex_digit, percent_decode, scaled_image, svg_payload};
use image::Rgba;

const ALT: &str = "fixture";
const ORIGINAL_WIDTH: u32 = 16;
const ORIGINAL_HEIGHT: u32 = 10;
const TINY_HEIGHT: u32 = 1;
const SCALED_WIDTH: u32 = 8;
const SCALED_HEIGHT: u32 = 5;
const SVG_RASTER_SCALE: u32 = 2;

#[test]
fn from_data_uri_renders_svg_data_uri() -> Result<(), Box<dyn std::error::Error>> {
    let image = image_from_data_uri("data:image/svg+xml,", ORIGINAL_WIDTH, ORIGINAL_HEIGHT, None)?;

    assert_eq!(image.display_width, ORIGINAL_WIDTH);
    assert_eq!(image.display_height, ORIGINAL_HEIGHT);
    assert_eq!(image.image.width(), ORIGINAL_WIDTH * SVG_RASTER_SCALE);
    assert_eq!(image.image.height(), ORIGINAL_HEIGHT * SVG_RASTER_SCALE);
    assert_eq!(image.alt_for_tests(), ALT);
    Ok(())
}

#[test]
fn from_data_uri_accepts_supported_svg_prefixes() -> Result<(), Box<dyn std::error::Error>> {
    let prefixes = [
        "data:image/svg+xml;charset=utf-8,",
        "data:image/svg+xml;utf8,",
        "DATA:IMAGE/SVG+XML,",
    ];

    for prefix in prefixes {
        let image = image_from_data_uri(prefix, ORIGINAL_WIDTH, ORIGINAL_HEIGHT, None)?;
        assert_eq!(image.display_width, ORIGINAL_WIDTH);
        assert_eq!(image.display_height, ORIGINAL_HEIGHT);
        assert_eq!(image.image.width(), ORIGINAL_WIDTH * SVG_RASTER_SCALE);
        assert_eq!(image.image.height(), ORIGINAL_HEIGHT * SVG_RASTER_SCALE);
    }
    Ok(())
}

#[test]
fn from_data_uri_scales_requested_width() -> Result<(), Box<dyn std::error::Error>> {
    let image = image_from_data_uri(
        "data:image/svg+xml,",
        ORIGINAL_WIDTH,
        ORIGINAL_HEIGHT,
        Some(SCALED_WIDTH),
    )?;

    assert_eq!(image.display_width, SCALED_WIDTH);
    assert_eq!(image.display_height, SCALED_HEIGHT);
    assert_eq!(image.image.width(), SCALED_WIDTH * SVG_RASTER_SCALE);
    assert_eq!(image.image.height(), SCALED_HEIGHT * SVG_RASTER_SCALE);
    Ok(())
}

#[test]
fn from_data_uri_keeps_minimum_height_when_scaled() -> Result<(), Box<dyn std::error::Error>> {
    let image = image_from_data_uri("data:image/svg+xml,", ORIGINAL_WIDTH, TINY_HEIGHT, Some(1))?;

    assert_eq!(image.display_width, 1);
    assert_eq!(image.display_height, 1);
    assert!(image.image.width() >= image.display_width);
    assert!(image.image.height() >= image.display_height);
    Ok(())
}

#[test]
fn scaled_image_resizes_requested_width_and_keeps_minimum_height() {
    let source =
        image::RgbaImage::from_pixel(ORIGINAL_WIDTH, ORIGINAL_HEIGHT, Rgba([0, 0, 0, 255]));
    let scaled = scaled_image(source, Some(SCALED_WIDTH));

    assert_eq!(scaled.width(), SCALED_WIDTH);
    assert_eq!(scaled.height(), SCALED_HEIGHT);

    let tiny = image::RgbaImage::from_pixel(ORIGINAL_WIDTH, TINY_HEIGHT, Rgba([0, 0, 0, 255]));
    let tiny_scaled = scaled_image(tiny, Some(1));

    assert_eq!(tiny_scaled.width(), 1);
    assert_eq!(tiny_scaled.height(), 1);
}

#[test]
fn from_data_uri_rejects_non_svg_or_invalid_payloads() {
    assert!(
        SurfaceImageBlock::from_data_uri("data:image/png,abc", None, ALT.to_string()).is_none()
    );
    assert!(
        SurfaceImageBlock::from_data_uri("data:image/svg+xml,%FF", None, ALT.to_string()).is_none()
    );
    assert!(
        SurfaceImageBlock::from_data_uri("data:image/svg+xml,%3Csvg%3E", None, ALT.to_string())
            .is_none()
    );
}

#[test]
fn svg_payload_rejects_short_or_unsupported_source() {
    assert_eq!(svg_payload("data"), None);
    assert_eq!(svg_payload("data:text/plain,hello"), None);
}

#[test]
fn percent_decode_decodes_hex_and_preserves_invalid_escape_text() {
    assert_eq!(percent_decode("%3csvg%3E"), Some("<svg>".to_string()));
    assert_eq!(percent_decode("%zz%4G%"), Some("%zz%4G%".to_string()));
}

#[test]
fn percent_decode_rejects_non_utf8_output() {
    assert_eq!(percent_decode("%FF"), None);
}

#[test]
fn hex_byte_decodes_numeric_lower_and_upper_digits() {
    assert_eq!(hex_byte(b'3', b'c'), Some(60));
    assert_eq!(hex_byte(b'3', b'C'), Some(60));
    assert_eq!(hex_byte(b'F', b'f'), Some(255));
}

#[test]
fn hex_digit_rejects_non_hex_digit() {
    assert_eq!(hex_digit(b'g'), None);
}

fn image_from_data_uri(
    prefix: &str,
    width: u32,
    height: u32,
    requested_width: Option<u32>,
) -> Result<SurfaceImageBlock, Box<dyn std::error::Error>> {
    let source = format!("{prefix}{}", encoded_svg(width, height));
    SurfaceImageBlock::from_data_uri(&source, requested_width, ALT.to_string())
        .ok_or_else(|| std::io::Error::other("data SVG image was not rendered").into())
}

fn encoded_svg(width: u32, height: u32) -> String {
    format!(
        "%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22{width}%22%20height%3D%22{height}%22%3E%3Crect%20width%3D%22{width}%22%20height%3D%22{height}%22%20fill%3D%22%23000%22%2F%3E%3C%2Fsvg%3E"
    )
}
