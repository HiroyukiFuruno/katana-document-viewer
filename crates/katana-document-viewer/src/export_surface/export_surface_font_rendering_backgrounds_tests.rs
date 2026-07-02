use super::*;
use crate::export_surface_font::SurfaceTextBackgroundPalette;
use crate::export_surface_span::{SurfaceTextSpan, SurfaceTextStyle};
use image::{Rgba, RgbaImage};

#[test]
fn does_not_paint_when_range_is_missing() {
    let mut image = RgbaImage::from_pixel(20, 20, Rgba([255, 255, 255, 255]));
    let spans = vec![SurfaceTextSpan::styled(
        "a",
        SurfaceTextStyle::default().highlight(),
    )];
    let ranges = vec![None];

    let before = image.clone();
    draw_span_backgrounds(
        &mut image,
        &spans,
        &ranges,
        2,
        4,
        20.0,
        SurfaceTextBackgroundPalette::default(),
    );

    assert_eq!(image, before);
}

#[test]
fn paints_highlight_background_for_marked_span() {
    let mut image = RgbaImage::from_pixel(20, 20, Rgba([255, 255, 255, 255]));
    let spans = vec![SurfaceTextSpan::styled(
        "a",
        SurfaceTextStyle::default().highlight(),
    )];
    let ranges = vec![Some(SpanVisualRange::new(2.0, 6.0))];

    let palette = SurfaceTextBackgroundPalette::default();
    draw_span_backgrounds(&mut image, &spans, &ranges, 4, 4, 10.0, palette);

    assert_eq!(image.get_pixel(6, 5).0, palette.highlight.0);
    assert_eq!(image.get_pixel(8, 10).0, palette.highlight.0);
    assert_eq!(image.get_pixel(1, 1).0, [255, 255, 255, 255]);
}

#[test]
fn paints_inline_code_background_for_inline_code_span() {
    let mut image = RgbaImage::from_pixel(30, 30, Rgba([255, 255, 255, 255]));
    let spans = vec![SurfaceTextSpan::styled(
        "a",
        SurfaceTextStyle::default().inline_code(),
    )];
    let ranges = vec![Some(SpanVisualRange::new(4.0, 7.0))];

    let palette = SurfaceTextBackgroundPalette {
        inline_code: Rgba([37, 37, 38, 255]),
        ..SurfaceTextBackgroundPalette::default()
    };
    draw_span_backgrounds(&mut image, &spans, &ranges, 8, 12, 20.0, palette);

    assert_eq!(image.get_pixel(10, 13).0, palette.inline_code.0);
    assert_eq!(image.get_pixel(18, 13).0, palette.inline_code.0);
    assert_eq!(image.get_pixel(28, 13).0, [255, 255, 255, 255]);
}
