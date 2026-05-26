use super::{SurfaceAlertBlock, SurfaceBlock, SurfaceDiagramBlock, SurfaceImageBlock};
use image::Rgba;
use image::RgbaImage;

#[test]
fn block_image_height_uses_image_height_with_vertical_margin()
-> Result<(), Box<dyn std::error::Error>> {
    let image = RgbaImage::from_pixel(11, 7, Rgba([0, 0, 0, 255]));
    let path = temp_image_path("kdv-blocks-image-1.png");
    image.save(&path)?;

    let image_block = SurfaceImageBlock::from_path(&path, None, "alt".to_string())
        .ok_or(std::io::Error::other("image block"))?;
    let block = SurfaceBlock::Image(image_block);

    assert_eq!(block.height(), image.height() + 18 * 2);
    assert_eq!(block.text_for_tests(), "alt");
    Ok(())
}

#[test]
fn block_text_contains_fallback_for_rendered_diagram() {
    let block = SurfaceBlock::Diagram(SurfaceDiagramBlock::rendered("<svg></svg>"));

    assert_eq!(block.text_for_tests(), "Rendered diagram");
}

#[test]
fn block_debug_reports_missing_diagram_image() {
    let block = SurfaceBlock::Diagram(SurfaceDiagramBlock::rendered("not svg"));

    assert_eq!(block.debug_for_tests(), "diagram:missing");
}

#[test]
fn block_debug_uses_expected_kind_prefixes() -> Result<(), Box<dyn std::error::Error>> {
    let image = RgbaImage::from_pixel(4, 4, Rgba([1, 2, 3, 255]));
    let path = temp_image_path("kdv-blocks-image-2.png");
    image.save(&path)?;
    let image = SurfaceImageBlock::from_path(&path, None, "a".to_string())
        .ok_or(std::io::Error::other("debug image block"))?;
    let image_block = SurfaceBlock::Image(image);
    let alert_block = SurfaceBlock::Alert(SurfaceAlertBlock::new("TIP", vec!["ok".to_string()], 0));

    assert!(image_block.debug_for_tests().starts_with("image"));
    assert!(alert_block.debug_for_tests().starts_with("alert:"));
    Ok(())
}

fn temp_image_path(file_name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(file_name)
}
