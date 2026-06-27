use super::super::RULE_HEIGHT;
use super::*;
use crate::export_surface::{SurfaceDiagramBlock, SurfaceImageBlock};
use crate::theme::KdvThemeSnapshot;
use image::Rgba;

const OPAQUE_CHANNEL: u8 = 255;
const WHITE_PIXEL: Rgba<u8> = Rgba([
    OPAQUE_CHANNEL,
    OPAQUE_CHANNEL,
    OPAQUE_CHANNEL,
    OPAQUE_CHANNEL,
]);
const SAMPLE_ALPHA: u8 = OPAQUE_CHANNEL;
const CODE_LINE_EXPECTED_X: u32 = 80;
const CODE_LINE_BOX_Y: u32 = 16;

fn has_painted_pixel(image: &image::RgbaImage) -> bool {
    image.pixels().any(|pixel| *pixel != WHITE_PIXEL)
}

fn system_text_painter() -> crate::export_surface_font::SurfaceTextPainter {
    crate::export_surface_font::SurfaceTextPainter::from_system_fonts()
}

fn measured_line_width(line: &crate::export_surface_line::SurfaceLine) -> u32 {
    let mut painter = system_text_painter();
    painter.measure_spans_width(&line.spans, line.font_size(), SURFACE_CONTENT_WIDTH as f32)
}

fn measured_centered_x(line: &crate::export_surface_line::SurfaceLine) -> u32 {
    let width = measured_line_width(line);
    PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(width) / 2
}

fn measured_right_x(line: &crate::export_surface_line::SurfaceLine) -> u32 {
    let width = measured_line_width(line);
    PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(width)
}

#[test]
fn paint_code_block_uses_quote_and_code_layout() {
    let lines = vec![crate::export_surface_line::SurfaceLine::code_spans(vec![
        crate::export_surface_span::SurfaceTextSpan::plain("hello".to_string()),
    ])];
    let block = SurfaceCodeBlock::new(lines, 1, 0);
    let mut painter = system_text_painter();
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut image = image::RgbaImage::from_pixel(240, 120, WHITE_PIXEL);
    SurfacePainter::paint_code_block(&mut image, &block, 4, &mut painter, &palette);
    let (box_x, _, _) = SurfacePainter::code_block_geometry(&block, 4);
    assert_ne!(image.get_pixel(PAGE_PADDING, 4).0, WHITE_PIXEL.0);
    assert_ne!(
        image.get_pixel(box_x + CODE_HORIZONTAL_PADDING + 1, 24).0,
        WHITE_PIXEL.0
    );
}

#[test]
fn paint_code_lines_and_code_line_use_system_painter() {
    let block = SurfaceCodeBlock::new(
        vec![
            crate::export_surface_line::SurfaceLine::body("a".to_string()),
            crate::export_surface_line::SurfaceLine::body("b".to_string()),
        ],
        0,
        0,
    );
    let mut painter = system_text_painter();
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut image = image::RgbaImage::from_pixel(200, 120, WHITE_PIXEL);
    let lines = block.lines.as_slice();
    SurfacePainter::paint_code_lines(
        &mut image,
        lines,
        CODE_LINE_EXPECTED_X,
        CODE_LINE_BOX_Y,
        &mut painter,
        &palette,
    );
    assert!(has_painted_pixel(&image));
}

#[test]
fn line_text_x_switches_for_code_and_center_lines() {
    let code = crate::export_surface_line::SurfaceLine::code_spans(vec![
        crate::export_surface_span::SurfaceTextSpan::plain("code".to_string()),
    ]);
    let centered = crate::export_surface_line::SurfaceLine::body_centered("center".to_string());
    let right = crate::export_surface_line::SurfaceLine::right_spans(vec![
        crate::export_surface_span::SurfaceTextSpan::plain("right".to_string()),
    ]);
    let expected_centered = PAGE_PADDING
        + SURFACE_CONTENT_WIDTH.saturating_sub(6 * LINE_CENTERED_TEXT_GUESS_CHAR_WIDTH) / 2;
    assert_eq!(
        SurfacePainter::line_text_x(&code),
        PAGE_PADDING + CODE_HORIZONTAL_PADDING
    );
    assert_eq!(SurfacePainter::line_text_x(&centered), expected_centered);
    assert_eq!(
        SurfacePainter::line_text_x_for_paint(&code, &mut system_text_painter()),
        PAGE_PADDING + CODE_HORIZONTAL_PADDING
    );
    assert_eq!(
        SurfacePainter::line_text_x_for_paint(&centered, &mut system_text_painter()),
        measured_centered_x(&centered)
    );
    assert_eq!(
        SurfacePainter::line_text_x_for_paint(&right, &mut system_text_painter()),
        measured_right_x(&right)
    );
}

#[test]
fn code_block_geometry_returns_expected_values() {
    let line = crate::export_surface_line::SurfaceLine::code_spans(vec![
        crate::export_surface_span::SurfaceTextSpan::plain("x".to_string()),
    ]);
    let block = SurfaceCodeBlock::new(vec![line], 2, 1);
    let box_x = PAGE_PADDING + 2 * QUOTE_INDENT + LIST_MARKER_COLUMN_WIDTH;
    assert_eq!(
        SurfacePainter::code_block_geometry(&block, 20),
        (box_x, SURFACE_WIDTH - (box_x + PAGE_PADDING), 34)
    );
}

#[test]
fn paint_rule_draws_horizontal_stroke() {
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut image = image::RgbaImage::from_pixel(240, 60, WHITE_PIXEL);
    SurfacePainter::paint_rule(&mut image, 8, &palette);
    let line_y = 8 + RULE_HEIGHT / 2;
    assert_ne!(image.get_pixel(PAGE_PADDING, line_y).0, WHITE_PIXEL.0);
}

#[path = "export_surface_painter_code_media_math_tests.rs"]
mod math_tests;
