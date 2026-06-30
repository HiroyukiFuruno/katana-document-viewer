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
fn paint_alert_line_uses_system_painter() {
    let mut image = image::RgbaImage::from_pixel(160, 80, Rgba([255, 255, 255, 255]));
    let mut painter = crate::export_surface_font::SurfaceTextPainter::from_system_fonts();
    let line = SurfaceLine::body("alert text".to_string());

    SurfacePainter::paint_alert_line(&mut image, &line, 8, 8, &mut painter, &palette());

    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
}

#[test]
fn paint_alert_background_keeps_body_area_unfilled_and_uses_left_rule() {
    let background = Rgba([12, 13, 14, 255]);
    let mut image = image::RgbaImage::from_pixel(240, 120, background);
    let alert = SurfaceAlertBlock::new(
        "WARNING",
        vec!["warning".to_string()],
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    SurfacePainter::paint_alert_background(&mut image, &alert, 0);

    assert_eq!(
        image.get_pixel(PAGE_PADDING, ALERT_PANEL_PADDING_Y),
        &alert_color("WARNING")
    );
    assert_eq!(
        image.get_pixel(
            PAGE_PADDING + ALERT_PANEL_BORDER_WIDTH + 12,
            ALERT_PANEL_PADDING_Y + 8
        ),
        &background,
        "alert body area must not be filled as a panel"
    );
}
