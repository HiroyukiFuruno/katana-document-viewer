use super::super::RULE_HEIGHT;
use super::*;
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

fn has_painted_pixel(image: &image::RgbaImage) -> bool {
    image.pixels().any(|pixel| *pixel != WHITE_PIXEL)
}

#[test]
fn paint_code_block_uses_quote_and_code_layout() {
    let lines = vec![crate::export_surface_line::SurfaceLine::code_spans(vec![
        crate::export_surface_span::SurfaceTextSpan::plain("hello".to_string()),
    ])];
    let block = SurfaceCodeBlock::new(lines, 1, 0);
    let mut painter = None;
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut image = image::RgbaImage::from_pixel(240, 120, WHITE_PIXEL);
    SurfacePainter::paint_code_block(&mut image, &block, 4, &mut painter, &palette);
    assert_ne!(image.get_pixel(PAGE_PADDING, 4).0, WHITE_PIXEL.0);
    assert_ne!(
        image
            .get_pixel(PAGE_PADDING + CODE_HORIZONTAL_PADDING + 1, 24)
            .0,
        WHITE_PIXEL.0
    );
}

#[test]
fn paint_code_lines_and_code_line_with_fallback() {
    let block = SurfaceCodeBlock::new(
        vec![
            crate::export_surface_line::SurfaceLine::body("a".to_string()),
            crate::export_surface_line::SurfaceLine::body("b".to_string()),
        ],
        0,
        0,
    );
    let mut painter = None;
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut image = image::RgbaImage::from_pixel(200, 120, WHITE_PIXEL);
    let lines = block.lines.as_slice();
    SurfacePainter::paint_code_lines(&mut image, lines, 16, &mut painter, &palette);
    assert!(has_painted_pixel(&image));
}

#[test]
fn line_text_x_switches_for_code_and_center_lines() {
    let code = crate::export_surface_line::SurfaceLine::code_spans(vec![
        crate::export_surface_span::SurfaceTextSpan::plain("code".to_string()),
    ]);
    let centered = crate::export_surface_line::SurfaceLine::body_centered("center".to_string());
    let expected_centered = PAGE_PADDING
        + SURFACE_CONTENT_WIDTH.saturating_sub(6 * LINE_CENTERED_TEXT_GUESS_CHAR_WIDTH) / 2;
    assert_eq!(
        SurfacePainter::line_text_x(&code),
        PAGE_PADDING + CODE_HORIZONTAL_PADDING
    );
    assert_eq!(SurfacePainter::line_text_x(&centered), expected_centered);
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
fn paint_math_block_uses_fallback_text_when_no_rendered_image()
-> Result<(), Box<dyn std::error::Error>> {
    let block = SurfaceMathBlock::for_tests(None, "raw expression".to_string());
    let mut painter = None;
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut image = image::RgbaImage::from_pixel(240, 96, WHITE_PIXEL);
    SurfacePainter::paint_math_block(&mut image, &block, 4, &mut painter, &palette);
    assert!(has_painted_pixel(&image));
    Ok(())
}

#[test]
fn paint_rule_draws_horizontal_stroke() {
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut image = image::RgbaImage::from_pixel(240, 60, WHITE_PIXEL);
    SurfacePainter::paint_rule(&mut image, 8, &palette);
    let line_y = 8 + RULE_HEIGHT / 2;
    assert_ne!(image.get_pixel(PAGE_PADDING, line_y).0, WHITE_PIXEL.0);
}

#[test]
fn paint_fallback_painter_and_rendered_math_share_same_entrypoint()
-> Result<(), Box<dyn std::error::Error>> {
    let mut image = image::RgbaImage::from_pixel(240, 64, WHITE_PIXEL);
    let fallback_math = SurfaceMathBlock::new("another-invalid-math-expression", None);
    let mut painter = Some(
        crate::export_surface_font::SurfaceTextPainter::from_system_fonts()
            .ok_or("system font should be available")?,
    );
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());

    SurfacePainter::paint_fallback_math(&mut image, &fallback_math, 4, &mut painter, &palette);
    assert!(has_painted_pixel(&image));
    Ok(())
}

#[test]
fn paint_rendered_math_and_diagram_rendered_share_entrypoint_output() {
    let rendered = SurfaceSvgImage {
        image: image::RgbaImage::from_pixel(12, 8, Rgba([12, 34, 56, SAMPLE_ALPHA])),
    };
    let mut image = image::RgbaImage::from_pixel(820, 64, WHITE_PIXEL);
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());

    SurfacePainter::paint_rendered_math(&mut image, &rendered, 4);
    assert!(has_painted_pixel(&image));

    let diagram = SurfaceDiagramBlock::rendered("<svg><text>ok</text></svg>");
    SurfacePainter::paint_diagram(&mut image, &diagram, 24, &palette);
    assert!(has_painted_pixel(&image));
}

#[test]
fn paint_math_block_uses_rendered_image_entrypoint() {
    let block = SurfaceMathBlock::for_tests(
        Some(image::RgbaImage::from_pixel(
            12,
            8,
            Rgba([12, 34, 56, SAMPLE_ALPHA]),
        )),
        "fallback".to_string(),
    );
    let mut painter = None;
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut image = image::RgbaImage::from_pixel(820, 64, WHITE_PIXEL);

    SurfacePainter::paint_math_block(&mut image, &block, 4, &mut painter, &palette);

    assert!(has_painted_pixel(&image));
}

#[test]
fn paint_diagram_fallback_and_image_blocks() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = image::RgbaImage::from_pixel(820, 120, WHITE_PIXEL);
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let diagram = SurfaceDiagramBlock::rendered("<svg><rect>");
    SurfacePainter::paint_diagram(&mut image, &diagram, 4, &palette);

    let path = std::env::temp_dir().join("kdv-surface-painter-image.png");
    image::RgbaImage::from_pixel(24, 10, Rgba([4, 5, 6, SAMPLE_ALPHA])).save(&path)?;
    let block = SurfaceImageBlock::from_path(&path, None, "alt".to_string())
        .ok_or(std::io::Error::other("surface image block"))?;
    SurfacePainter::paint_image(&mut image, &block, 54);

    assert!(has_painted_pixel(&image));
    Ok(())
}

#[test]
fn paint_fallback_math_uses_painter_path() -> Result<(), Box<dyn std::error::Error>> {
    let block = SurfaceMathBlock::new("yet-another-invalid", None);
    let mut painter = Some(
        crate::export_surface_font::SurfaceTextPainter::from_system_fonts()
            .ok_or("system font should be available")?,
    );
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut image = image::RgbaImage::from_pixel(240, 64, WHITE_PIXEL);
    SurfacePainter::paint_fallback_math(&mut image, &block, 4, &mut painter, &palette);
    assert!(has_painted_pixel(&image));
    Ok(())
}
