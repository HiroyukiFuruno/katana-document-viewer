use super::*;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::{ImageEncoder, Rgb, RgbImage, Rgba, RgbaImage};

const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
const JPEG_SIGNATURE: &[u8] = b"\xff\xd8\xff";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[test]
fn blank_document_scale_png_fails_visual_content_check() -> Result<(), image::ImageError> {
    let image = RgbaImage::from_pixel(WIDTH, HEIGHT, Rgba([255, 255, 255, 255]));
    let bytes = encode_png(&image)?;
    let decoded = ImageQualityScore::decode_dimensions(&bytes);

    let score = ImageQualityScore::score(ExportFormat::Png, &bytes, decoded, PNG_SIGNATURE);

    assert_failed_check(&score, "png is not visually blank");
    assert!(!score.is_pass());
    Ok(())
}

#[test]
fn text_like_png_passes_visual_content_check() -> Result<(), image::ImageError> {
    let mut image = RgbaImage::from_pixel(WIDTH, HEIGHT, Rgba([255, 255, 255, 255]));
    draw_text_like_marks(&mut image);
    let bytes = encode_png(&image)?;
    let decoded = ImageQualityScore::decode_dimensions(&bytes);

    let score = ImageQualityScore::score(ExportFormat::Png, &bytes, decoded, PNG_SIGNATURE);

    assert_passed_check(&score, "png is not visually blank");
    assert!(score.is_pass());
    Ok(())
}

#[test]
fn single_dot_document_scale_png_fails_visual_content_check() -> Result<(), image::ImageError> {
    let mut image = RgbaImage::from_pixel(WIDTH, HEIGHT, Rgba([255, 255, 255, 255]));
    image.put_pixel(72, 80, Rgba([24, 24, 24, 255]));
    let bytes = encode_png(&image)?;
    let decoded = ImageQualityScore::decode_dimensions(&bytes);

    let score = ImageQualityScore::score(ExportFormat::Png, &bytes, decoded, PNG_SIGNATURE);

    assert_failed_check(&score, "png is not visually blank");
    assert!(!score.is_pass());
    Ok(())
}

#[test]
fn single_dot_document_scale_jpeg_fails_visual_content_check() -> Result<(), image::ImageError> {
    let mut image = RgbImage::from_pixel(WIDTH, HEIGHT, Rgb([255, 255, 255]));
    image.put_pixel(72, 80, Rgb([24, 24, 24]));
    let bytes = encode_jpeg(&image)?;
    let decoded = ImageQualityScore::decode_dimensions(&bytes);

    let score = ImageQualityScore::score(ExportFormat::Jpeg, &bytes, decoded, JPEG_SIGNATURE);

    assert_failed_check(&score, "jpeg is not visually blank");
    assert!(!score.is_pass());
    Ok(())
}

#[test]
fn low_info_png_fails_visual_content_check() -> Result<(), image::ImageError> {
    let mut image = RgbaImage::from_pixel(WIDTH, HEIGHT, Rgba([255, 255, 255, 255]));
    draw_low_info_marks(&mut image);
    let bytes = encode_png(&image)?;
    let decoded = ImageQualityScore::decode_dimensions(&bytes);

    let score = ImageQualityScore::score(ExportFormat::Png, &bytes, decoded, PNG_SIGNATURE);

    assert_failed_check(&score, "png is not visually blank");
    assert!(!score.is_pass());
    Ok(())
}

#[test]
fn low_info_jpeg_fails_visual_content_check() -> Result<(), image::ImageError> {
    let mut image = RgbImage::from_pixel(WIDTH, HEIGHT, Rgb([255, 255, 255]));
    draw_low_info_rgb_marks(&mut image);
    let bytes = encode_jpeg(&image)?;
    let decoded = ImageQualityScore::decode_dimensions(&bytes);

    let score = ImageQualityScore::score(ExportFormat::Jpeg, &bytes, decoded, JPEG_SIGNATURE);

    assert_failed_check(&score, "jpeg is not visually blank");
    assert!(!score.is_pass());
    Ok(())
}

fn draw_text_like_marks(image: &mut RgbaImage) {
    for y in 80..96 {
        for x in 72..260 {
            image.put_pixel(x, y, Rgba([24, 24, 24, 255]));
        }
    }
}

fn draw_low_info_marks(image: &mut RgbaImage) {
    for y in 80..100 {
        for x in 72..97 {
            image.put_pixel(x, y, Rgba([24, 24, 24, 255]));
        }
    }
}

fn draw_low_info_rgb_marks(image: &mut RgbImage) {
    for y in 80..100 {
        for x in 72..97 {
            image.put_pixel(x, y, Rgb([24, 24, 24]));
        }
    }
}

fn encode_png(image: &RgbaImage) -> Result<Vec<u8>, image::ImageError> {
    let mut bytes = Vec::new();
    PngEncoder::new(&mut bytes).write_image(
        image.as_raw(),
        image.width(),
        image.height(),
        image::ColorType::Rgba8.into(),
    )?;
    Ok(bytes)
}

fn encode_jpeg(image: &RgbImage) -> Result<Vec<u8>, image::ImageError> {
    let mut bytes = Vec::new();
    JpegEncoder::new_with_quality(&mut bytes, 100).write_image(
        image.as_raw(),
        image.width(),
        image.height(),
        image::ColorType::Rgb8.into(),
    )?;
    Ok(bytes)
}

fn assert_failed_check(score: &ExportFormatQualityScore, name: &str) {
    assert!(
        score
            .checks
            .iter()
            .any(|check| check.name == name && !check.passed)
    );
}

fn assert_passed_check(score: &ExportFormatQualityScore, name: &str) {
    assert!(
        score
            .checks
            .iter()
            .any(|check| check.name == name && check.passed)
    );
}
