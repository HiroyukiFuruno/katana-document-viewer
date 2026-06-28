use super::*;

#[test]
fn paint_math_block_uses_raw_text_when_no_rendered_image() -> Result<(), Box<dyn std::error::Error>>
{
    let block = SurfaceMathBlock::for_tests(None, "raw expression".to_string());
    let mut painter = system_text_painter();
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut image = image::RgbaImage::from_pixel(240, 96, WHITE_PIXEL);
    SurfacePainter::paint_math_block(&mut image, &block, 4, &mut painter, &palette);
    assert!(has_painted_pixel(&image));
    Ok(())
}

#[test]
fn paint_raw_math_and_rendered_math_share_same_entrypoint() -> Result<(), Box<dyn std::error::Error>>
{
    let mut image = image::RgbaImage::from_pixel(240, 64, WHITE_PIXEL);
    let raw_math = SurfaceMathBlock::new("another-invalid-math-expression", None);
    let mut painter = system_text_painter();
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());

    SurfacePainter::paint_raw_math_text(&mut image, &raw_math, 4, &mut painter, &palette);
    assert!(has_painted_pixel(&image));
    Ok(())
}

#[test]
fn paint_rendered_math_and_diagram_rendered_share_entrypoint_output() {
    let rendered = SurfaceSvgImage::from_image(image::RgbaImage::from_pixel(
        12,
        8,
        Rgba([12, 34, 56, SAMPLE_ALPHA]),
    ));
    let mut image = image::RgbaImage::from_pixel(820, 64, WHITE_PIXEL);
    let mut painter = system_text_painter();
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());

    SurfacePainter::paint_rendered_math(&mut image, &rendered, 4);
    assert!(has_painted_pixel(&image));

    let diagram = SurfaceDiagramBlock::rendered("<svg><text>ok</text></svg>");
    SurfacePainter::paint_diagram(&mut image, &diagram, 24, &mut painter, &palette);
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
        "raw".to_string(),
    );
    let mut painter = system_text_painter();
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut image = image::RgbaImage::from_pixel(820, 64, WHITE_PIXEL);

    SurfacePainter::paint_math_block(&mut image, &block, 4, &mut painter, &palette);

    assert!(has_painted_pixel(&image));
}

#[test]
fn paint_diagram_and_image_blocks() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = image::RgbaImage::from_pixel(820, 120, WHITE_PIXEL);
    let mut painter = system_text_painter();
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let diagram = SurfaceDiagramBlock::rendered("<svg><rect>");
    SurfacePainter::paint_diagram(&mut image, &diagram, 4, &mut painter, &palette);

    let path = std::env::temp_dir().join("kdv-surface-painter-image.png");
    image::RgbaImage::from_pixel(24, 10, Rgba([4, 5, 6, SAMPLE_ALPHA])).save(&path)?;
    let block = SurfaceImageBlock::from_path(&path, None, "alt".to_string())
        .ok_or(std::io::Error::other("surface image block"))?;
    SurfacePainter::paint_image(&mut image, &block, 54);

    assert!(has_painted_pixel(&image));
    Ok(())
}

#[test]
fn paint_raw_math_text_uses_painter_path() -> Result<(), Box<dyn std::error::Error>> {
    let block = SurfaceMathBlock::new("yet-another-invalid", None);
    let mut painter = system_text_painter();
    let palette = SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light());
    let mut image = image::RgbaImage::from_pixel(240, 64, WHITE_PIXEL);
    SurfacePainter::paint_raw_math_text(&mut image, &block, 4, &mut painter, &palette);
    assert!(has_painted_pixel(&image));
    Ok(())
}
