use super::*;
use crate::export_surface_helpers::PAGE_PADDING;
use crate::export_surface_span::SurfaceTextSpan;
use crate::theme::KdvThemeSnapshot;
use image::Rgba;

const TEST_TEXT_SIZE: f32 = 24.0;

#[test]
fn paint_line_without_marker_uses_painter() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = image::RgbaImage::from_pixel(160, 80, Rgba([255, 255, 255, 255]));
    let mut painter = system_text_painter();
    let palette = super::super::SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let line = SurfaceLine::body("paint with painter".to_string());
    let text_x = 8;
    let text_y = 10;
    SurfacePainter::paint_line_without_marker(
        &mut image,
        &line,
        text_x,
        text_y,
        24.0,
        &mut painter,
        &palette,
    );
    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );

    Ok(())
}

#[test]
fn paint_line_without_marker_draws_text_with_system_painter() {
    let mut image = image::RgbaImage::from_pixel(160, 80, Rgba([255, 255, 255, 255]));
    let mut painter = system_text_painter();
    let palette = super::super::SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let line = SurfaceLine::body("system painter".to_string());
    SurfacePainter::paint_line_without_marker(
        &mut image,
        &line,
        8,
        12,
        24.0,
        &mut painter,
        &palette,
    );
    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
}

#[test]
fn paint_line_list_marker_and_content_is_drawn() {
    let mut image = image::RgbaImage::from_pixel(220, 80, Rgba([255, 255, 255, 255]));
    let mut painter = system_text_painter();
    let palette = super::super::SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let spans = vec![
        SurfaceTextSpan::plain("1."),
        SurfaceTextSpan::plain(" item text"),
    ];
    let line = SurfaceLine::body_spans(spans, 0);
    SurfacePainter::paint_line(&mut image, &line, 0, &mut painter, &palette);
    assert!(line.list_marker().is_some());
    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
}

#[test]
fn paint_aligned_list_line_pushes_list_text_horizontally() {
    let mut image = image::RgbaImage::from_pixel(220, 80, Rgba([255, 255, 255, 255]));
    let mut painter = system_text_painter();
    let palette = super::super::SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let spans = vec![SurfaceTextSpan::plain("A"), SurfaceTextSpan::plain(" B")];
    let line = SurfaceLine::body_spans(spans, 0);
    SurfacePainter::paint_aligned_list_line(
        &mut image,
        &line,
        10,
        10,
        24.0,
        &mut painter,
        &palette,
    );
    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
}

#[test]
fn paint_line_without_marker_routes_in_progress_task_alignment() {
    let mut image = image::RgbaImage::from_pixel(220, 80, Rgba([255, 255, 255, 255]));
    let mut painter = system_text_painter();
    let palette = super::super::SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let spans = vec![
        SurfaceTextSpan::plain("◩"),
        SurfaceTextSpan::plain(" in progress"),
    ];
    let line = SurfaceLine::body_spans(spans, 0);

    SurfacePainter::paint_line_without_marker(
        &mut image,
        &line,
        10,
        10,
        24.0,
        &mut painter,
        &palette,
    );

    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
}

#[test]
fn paint_line_with_quote_bars_draws_quote_markers() {
    let mut image = image::RgbaImage::from_pixel(160, 80, Rgba([255, 255, 255, 255]));
    let mut painter = system_text_painter();
    let palette = super::super::SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let line = SurfaceLine::body_with_quote("quoted line".to_string(), 2);
    SurfacePainter::paint_line(&mut image, &line, 2, &mut painter, &palette);
    assert!(image.get_pixel(PAGE_PADDING, 2).0 != [255, 255, 255, 255]);
}

#[test]
fn paint_line_without_marker_draws_body_center_and_code() {
    let mut image = image::RgbaImage::from_pixel(220, 120, Rgba([255, 255, 255, 255]));
    let mut painter = system_text_painter();
    let palette = super::super::SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let normal = SurfaceLine::body("normal".to_string());
    let centered = SurfaceLine::body_centered("center".to_string());
    let code = SurfaceLine::code_spans(vec![SurfaceTextSpan::plain("code".to_string())]);

    paint_without_marker_case(&mut image, &normal, 8, &mut painter, &palette);
    paint_without_marker_case(&mut image, &centered, 40, &mut painter, &palette);
    paint_without_marker_case(&mut image, &code, 60, &mut painter, &palette);

    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
}

fn paint_without_marker_case(
    image: &mut image::RgbaImage,
    line: &SurfaceLine,
    y: u32,
    painter: &mut SurfaceTextPainter,
    palette: &SurfacePaintPalette,
) {
    SurfacePainter::paint_line_without_marker(
        image,
        line,
        line.x(),
        y,
        TEST_TEXT_SIZE,
        painter,
        palette,
    );
}

fn system_text_painter() -> SurfaceTextPainter {
    crate::export_surface_font::SurfaceTextPainter::from_system_fonts()
}
