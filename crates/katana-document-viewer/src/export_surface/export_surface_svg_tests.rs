use super::*;

const FONT_SCALE_TOLERANCE: f32 = 0.2;

#[test]
fn rasterize_keep_diagram_scale_under_original_max() {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20"><rect width="20" height="20"/></svg>"#;
    let image = SurfaceSvgRasterizer::rasterize(svg, 100);

    assert!(image.is_some());
    assert_eq!(image.map(|image| image.image.width()).unwrap_or(0), 20);
}

#[test]
fn rasterize_keeps_small_svg_from_over_scaling() {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20"><rect width="20" height="20"/></svg>"#;
    let image = SurfaceSvgRasterizer::rasterize(svg, 200);

    assert!(image.is_some());
    assert_eq!(image.map(|image| image.image.width()).unwrap_or(0), 20);
}

#[test]
fn export_surface_rasterize_keeps_layout_size_with_higher_density_pixels()
-> Result<(), Box<dyn std::error::Error>> {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="80"><rect width="200" height="80"/></svg>"#;
    let image = SurfaceSvgRasterizer::rasterize_for_export_surface(svg, 50)
        .ok_or(std::io::Error::other("export surface SVG should rasterize"))?;

    assert_eq!(image.display_width_px(), 50);
    assert_eq!(image.display_height_px(), 20);
    assert_eq!(image.image.width(), 100);
    assert_eq!(image.image.height(), 40);
    Ok(())
}

#[test]
fn rasterize_keeps_fractional_svg_display_size_for_viewer_layout() {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-8 -8 324.9855 524.3"><rect width="10" height="10"/></svg>"#;
    let image = SurfaceSvgRasterizer::rasterize(svg, 1000);
    assert!(image.is_some(), "svg should rasterize");
    let Some(image) = image else {
        return;
    };

    assert_eq!(324.9855, image.display_width);
    assert_eq!(524.3, image.display_height);
}

#[test]
fn rasterize_keeps_root_font_size_as_css_unit_for_ex_sizing() {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="8.704ex" height="1.912ex"><text x="0" y="10">E = mc^2</text></svg>"#;
    let root_font_size = 24.0;
    let small =
        SurfaceSvgRasterizer::rasterize_with_root_font_size(svg, 1000, Some(root_font_size));
    let large =
        SurfaceSvgRasterizer::rasterize_with_root_font_size(svg, 1000, Some(root_font_size * 2.0));
    assert!(small.is_some());
    assert!(large.is_some());
    let small = small
        .map(|image| image.image)
        .unwrap_or(RgbaImage::new(1, 1));
    let large = large
        .map(|image| image.image)
        .unwrap_or(RgbaImage::new(1, 1));

    assert!(large.width() > small.width());
    assert!(large.height() > small.height());
    assert!(((large.width() as f32 / small.width() as f32) - 2.0).abs() < FONT_SCALE_TOLERANCE,);
}

#[test]
fn preprocess_for_rasterizer_appends_root_font_size_to_svg_style_if_missing() {
    let raw = r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect/></svg>"#;
    let processed = super::preprocess_for_rasterizer(raw, Some(16.0));

    assert!(
        processed.contains("style=\"font-size:16px;\""),
        "missing css root font-size injection: {processed}"
    );
}

#[test]
fn preprocess_for_rasterizer_keeps_existing_svg_style() {
    let raw = r#"<svg xmlns="http://www.w3.org/2000/svg" style="color:#000" width="10" height="10"><rect/></svg>"#;
    let processed = super::preprocess_for_rasterizer(raw, Some(16.0));

    assert!(processed.contains("color:#000"));
    assert!(processed.contains("font-size:16px"));
}

#[test]
fn rasterize_rejects_invalid_svg() {
    assert!(SurfaceSvgRasterizer::rasterize("<svg><rect>", 100).is_none());
}

#[test]
fn logical_extent_falls_back_to_one_for_invalid_values() {
    assert_eq!(super::logical_extent(0.0), 1.0);
    assert_eq!(super::logical_extent(-12.0), 1.0);
    assert_eq!(super::logical_extent(f32::NAN), 1.0);
    assert_eq!(super::logical_extent(f32::INFINITY), 1.0);
}
