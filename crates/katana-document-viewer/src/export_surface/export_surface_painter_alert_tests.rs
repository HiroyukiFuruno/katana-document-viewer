use super::*;
use crate::theme::KdvThemeSnapshot;
use image::Rgba;

fn palette() -> SurfacePaintPalette {
    SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light())
}

#[test]
fn paint_alert_icon_covers_warning_and_default_paths() {
    let mut image = image::RgbaImage::from_pixel(80, 40, Rgba([255, 255, 255, 255]));

    SurfacePainter::paint_alert_icon(&mut image, "CAUTION", 4, 4);
    SurfacePainter::paint_alert_icon(&mut image, "NOTE", 32, 4);

    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
}

#[test]
fn paint_alert_line_uses_fallback_when_painter_missing() {
    let mut image = image::RgbaImage::from_pixel(160, 80, Rgba([255, 255, 255, 255]));
    let mut painter = None;
    let line = SurfaceLine::body("alert fallback".to_string());

    SurfacePainter::paint_alert_line(&mut image, &line, 8, 8, &mut painter, &palette());

    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
}
