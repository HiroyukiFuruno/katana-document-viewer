use super::*;
use crate::export_surface_span::{SurfaceTextSpan, SurfaceTextStyle};
use crate::export_surface_svg::SurfaceSvgImage;
use image::{Rgba, RgbaImage};

#[test]
fn does_not_paint_when_span_is_not_inline_image() {
    let mut image = RgbaImage::from_pixel(20, 20, Rgba([255, 255, 255, 255]));
    let spans = vec![SurfaceTextSpan::plain("text")];
    let ranges = vec![Some(SpanVisualRange::new(2.0, 6.0))];
    let before = image.clone();

    draw_inline_images(&mut image, &spans, &ranges, 2, 6, 16.0);

    assert_eq!(image, before);
}

#[test]
fn does_not_paint_when_range_is_missing() {
    let source = RgbaImage::from_pixel(2, 2, Rgba([128, 64, 0, 255]));
    let mut image = RgbaImage::from_pixel(20, 20, Rgba([255, 255, 255, 255]));
    let spans = vec![SurfaceTextSpan::inline_image(
        "image",
        SurfaceSvgImage::from_image(source),
        SurfaceTextStyle::default(),
    )];
    let ranges = vec![None];

    draw_inline_images(&mut image, &spans, &ranges, 5, 4, 12.0);

    assert!(image.pixels().all(|pixel| pixel.0 == [255, 255, 255, 255]));
}

#[test]
fn paints_inline_image_at_calculated_position() {
    let source = RgbaImage::from_pixel(2, 2, Rgba([12, 34, 56, 255]));
    let mut image = RgbaImage::from_pixel(20, 20, Rgba([255, 255, 255, 255]));
    let spans = vec![SurfaceTextSpan::inline_image(
        "image",
        SurfaceSvgImage::from_image(source),
        SurfaceTextStyle::default(),
    )];
    let ranges = vec![Some(SpanVisualRange::new(4.0, 8.0))];

    draw_inline_images(&mut image, &spans, &ranges, 2, 10, 16.0);

    let image_x = 2 + 4;
    let image_y = InlineImagePainter::image_y(2, 10, 16.0);
    assert_eq!(image.get_pixel(image_x, image_y).0, [12, 34, 56, 255]);
}
