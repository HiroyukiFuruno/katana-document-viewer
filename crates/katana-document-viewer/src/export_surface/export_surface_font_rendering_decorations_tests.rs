use super::*;
use crate::export_surface_span::{SurfaceTextSpan, SurfaceTextStyle};
use image::{Rgba, RgbaImage};

#[test]
fn does_not_paint_when_range_is_missing() {
    let mut image = RgbaImage::from_pixel(20, 20, Rgba([255, 255, 255, 255]));
    let spans = vec![SurfaceTextSpan::styled(
        "a",
        SurfaceTextStyle::default().underline(),
    )];
    let ranges = vec![None];

    let before = image.clone();
    draw_span_decorations(&mut image, &spans, &ranges, 2, 4, 14.0);

    assert_eq!(image, before);
}

#[test]
fn does_not_paint_when_style_is_missing() {
    let mut image = RgbaImage::from_pixel(20, 20, Rgba([255, 255, 255, 255]));
    let spans = vec![SurfaceTextSpan::styled("a", SurfaceTextStyle::default())];
    let ranges = vec![Some(SpanVisualRange::new(2.0, 6.0))];
    let before = image.clone();

    draw_span_decorations(&mut image, &spans, &ranges, 2, 4, 14.0);

    assert_eq!(image, before);
}

#[test]
fn paints_underline_and_strike_through() {
    let mut image = RgbaImage::from_pixel(30, 30, Rgba([255, 255, 255, 255]));
    let color = Rgba([10, 20, 30, 255]);
    let spans = vec![SurfaceTextSpan::styled(
        "a",
        SurfaceTextStyle::default()
            .underline()
            .strikethrough()
            .with_color(color),
    )];
    let ranges = vec![Some(SpanVisualRange::new(4.0, 9.0))];

    draw_span_decorations(&mut image, &spans, &ranges, 2, 6, 10.0);

    assert_eq!(image.get_pixel(6, 18).0, color.0);
    assert_eq!(image.get_pixel(6, 13).0, color.0);
}
