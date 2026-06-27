use super::*;
use crate::theme::KdvThemeSnapshot;
use image::Rgba;

fn palette() -> SurfacePaintPalette {
    SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light())
}

#[test]
fn paint_badge_label_uses_system_painter() {
    let mut image = image::RgbaImage::from_pixel(180, 64, Rgba([0, 0, 0, 255]));
    let badge = SurfaceBadge::linked(
        "label".to_string(),
        "message".to_string(),
        Rgba([1, 2, 3, 255]),
        None,
    );
    let mut painter = crate::export_surface_font::SurfaceTextPainter::from_system_fonts();

    SurfacePainter::paint_badge_label(&mut image, &badge, 4, 8, &mut painter);

    assert!(
        image
            .pixels()
            .any(|pixel| *pixel == Rgba([255, 255, 255, 255]))
    );
}

#[test]
fn paint_badge_row_and_background_handle_single_segment_badge() {
    let mut image = image::RgbaImage::from_pixel(820, 64, Rgba([255, 255, 255, 255]));
    let row = SurfaceBadgeRowBlock::new(vec![SurfaceBadge::single("single".to_string())]);
    let mut painter = crate::export_surface_font::SurfaceTextPainter::from_system_fonts();

    SurfacePainter::paint_badge_row(&mut image, &row, 0, &mut painter, &palette());

    assert_eq!(row.badges()[0].message_width(), 0);
    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
}
