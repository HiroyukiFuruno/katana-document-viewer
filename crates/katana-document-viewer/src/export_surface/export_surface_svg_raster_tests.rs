use resvg::usvg;

use super::{
    FALLBACK_MONOSPACE_FONT_FAMILY, FALLBACK_PROPORTIONAL_FONT_FAMILY, RasterTarget,
    rasterizer_options,
};

fn parse_tree(svg: &str) -> Result<usvg::Tree, usvg::Error> {
    usvg::Tree::from_str(svg, &rasterizer_options())
}

#[test]
fn rasterizer_options_match_katana_embedded_font_fallbacks() {
    let options = rasterizer_options();

    assert_eq!(FALLBACK_PROPORTIONAL_FONT_FAMILY, options.font_family);
    assert!(has_font_family(
        &options.fontdb,
        FALLBACK_PROPORTIONAL_FONT_FAMILY
    ));
    assert!(has_font_family(
        &options.fontdb,
        FALLBACK_MONOSPACE_FONT_FAMILY
    ));
    assert!(has_font_family(&options.fontdb, "Noto Emoji"));
}

#[test]
fn raster_target_preserves_size_when_max_width_is_bigger() -> Result<(), Box<dyn std::error::Error>>
{
    let tree = parse_tree(
        r#"<svg xmlns='http://www.w3.org/2000/svg' width='20' height='10'><rect width='20' height='10'/></svg>"#,
    )?;

    let target = RasterTarget::new(tree.size(), 40);

    assert_eq!(target.width(), 20);
    assert_eq!(target.height(), 10);
    Ok(())
}

#[test]
fn raster_target_uses_content_scale_for_retina_surface() -> Result<(), Box<dyn std::error::Error>> {
    let tree = parse_tree(
        r#"<svg xmlns='http://www.w3.org/2000/svg' width='20' height='10'><rect width='20' height='10'/></svg>"#,
    )?;

    let target = RasterTarget::new_with_content_scale(tree.size(), 40, 200);

    assert_eq!(target.width(), 40);
    assert_eq!(target.height(), 20);
    Ok(())
}

#[test]
fn raster_target_keeps_katana_retina_pixels_before_display_scaling()
-> Result<(), Box<dyn std::error::Error>> {
    let tree = parse_tree(
        r#"<svg xmlns='http://www.w3.org/2000/svg' width='200' height='80'><rect width='200' height='80'/></svg>"#,
    )?;

    let target = RasterTarget::new_with_content_scale(tree.size(), 50, 200);

    assert_eq!(target.width(), 400);
    assert_eq!(target.height(), 160);
    Ok(())
}

#[test]
fn raster_target_keeps_export_surface_max_width_scaling() -> Result<(), Box<dyn std::error::Error>>
{
    let tree = parse_tree(
        r#"<svg xmlns='http://www.w3.org/2000/svg' width='200' height='80'><rect width='200' height='80'/></svg>"#,
    )?;

    let target = RasterTarget::new(tree.size(), 50);

    assert_eq!(target.width(), 50);
    assert_eq!(target.height(), 20);
    Ok(())
}

#[test]
fn export_surface_raster_target_preserves_layout_width_with_retina_pixels()
-> Result<(), Box<dyn std::error::Error>> {
    let tree = parse_tree(
        r#"<svg xmlns='http://www.w3.org/2000/svg' width='200' height='80'><rect width='200' height='80'/></svg>"#,
    )?;

    let target = RasterTarget::new_export_surface(tree.size(), 50, 200);

    assert_eq!(target.display_width(), 50);
    assert_eq!(target.display_height(), 20);
    assert_eq!(target.width(), 100);
    assert_eq!(target.height(), 40);
    Ok(())
}

#[test]
fn raster_target_clamps_at_maximum_edge() -> Result<(), Box<dyn std::error::Error>> {
    let tree = parse_tree(
        r#"<svg xmlns='http://www.w3.org/2000/svg' width='10000' height='2000'><rect width='10000' height='2000'/></svg>"#,
    )?;

    let target = RasterTarget::new(tree.size(), 12000);

    assert_eq!(target.width(), 8192);
    assert_eq!(target.height(), 1639);
    Ok(())
}

#[test]
fn raster_target_render_is_not_empty_canvas() -> Result<(), Box<dyn std::error::Error>> {
    let tree = parse_tree(
        r#"<svg xmlns='http://www.w3.org/2000/svg' width='2' height='3'><rect x='0' y='0' width='2' height='3' fill='black'/></svg>"#,
    )?;

    let target = RasterTarget::new(tree.size(), 2);
    let pixmap = target
        .render(&tree)
        .ok_or_else(|| std::io::Error::other("render returns output"))?;

    assert_eq!(pixmap.width(), 2);
    assert_eq!(pixmap.height(), 3);
    Ok(())
}

#[test]
fn raster_target_minimum_size_is_one_pixel() -> Result<(), Box<dyn std::error::Error>> {
    let tree = parse_tree(
        r#"<svg xmlns='http://www.w3.org/2000/svg' width='0.001' height='0.001'><rect/></svg>"#,
    )?;

    let target = RasterTarget::new(tree.size(), 100);

    assert_eq!(target.width(), 1);
    assert_eq!(target.height(), 1);
    Ok(())
}

fn has_font_family(fontdb: &usvg::fontdb::Database, family: &str) -> bool {
    fontdb.faces().any(|face| {
        face.families
            .iter()
            .any(|(candidate, _)| candidate == family)
    })
}
