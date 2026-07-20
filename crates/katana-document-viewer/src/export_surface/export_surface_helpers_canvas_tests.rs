use super::SurfaceHelpers;
use crate::export_surface_helpers::PAGE_PADDING;
use image::{Rgba, RgbaImage};

#[test]
fn parse_color_invalid_value_returns_white() {
    let color = SurfaceHelpers::parse_color("#ff");
    assert_eq!(color.0, [255, 255, 255, 255]);
}

#[test]
fn paste_rgba_resized_with_zero_dimensions_is_noop() {
    let source = RgbaImage::from_pixel(1, 1, Rgba([255, 0, 0, 255]));
    let mut target = RgbaImage::from_pixel(4, 4, Rgba([0, 0, 0, 255]));
    let before = target.clone();

    SurfaceHelpers::paste_rgba_resized(&mut target, &source, 1, 1, 0, 2);

    assert_eq!(target, before);
}

#[test]
fn stroke_rect_with_zero_size_is_noop() {
    let mut image = RgbaImage::from_pixel(4, 4, Rgba([255, 255, 255, 255]));
    let before = image.clone();

    SurfaceHelpers::stroke_rect(&mut image, 0, 0, 0, 6, Rgba([0, 0, 0, 255]));

    assert_eq!(image, before);
}

#[test]
fn paste_rgba_skips_fully_transparent_pixels() {
    let source = RgbaImage::from_pixel(2, 2, Rgba([255, 0, 0, 0]));
    let mut target = RgbaImage::from_pixel(6, 6, Rgba([255, 255, 255, 255]));
    let before = target.clone();

    SurfaceHelpers::paste_rgba(&mut target, &source, 2, 2);

    assert_eq!(target, before);
}

#[test]
fn paste_rgba_skips_pixels_outside_target_bounds() {
    let source = RgbaImage::from_pixel(3, 3, Rgba([255, 0, 0, 255]));
    let mut target = RgbaImage::from_pixel(2, 2, Rgba([255, 255, 255, 255]));

    SurfaceHelpers::paste_rgba(&mut target, &source, 1, 1);

    assert_eq!(target.get_pixel(1, 1).0, [255, 0, 0, 255]);
    assert_eq!(target.get_pixel(0, 0).0, [255, 255, 255, 255]);
}

#[test]
fn block_stack_height_sums_explicit_blocks_without_implicit_gap() {
    let height = SurfaceHelpers::block_stack_height([300, 300, 300].into_iter());

    assert_eq!(300 * 3, height);
}

#[test]
fn surface_block_height_wraps_stack_with_page_padding() {
    let height = SurfaceHelpers::surface_block_height([300, 300, 300].into_iter());

    assert_eq!(300 * 3 + PAGE_PADDING * 2, height);
}
