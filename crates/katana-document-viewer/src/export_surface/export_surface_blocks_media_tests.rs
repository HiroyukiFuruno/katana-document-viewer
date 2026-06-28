use super::{SurfaceCodeBlock, SurfaceDiagramBlock, SurfaceMathBlock, SurfaceSpanMetrics};
use crate::ViewerCodeBlockMetrics;
use crate::export_surface::SurfaceImageBlock;
use crate::export_surface_helpers::SURFACE_CONTENT_WIDTH;
use crate::export_surface_line::SurfaceTypographyConfig;
use crate::export_surface_span::SurfaceTextSpan;
use crate::render_runtime::{KrrRenderOutput, KrrRenderPayload};
use image::Rgba;

#[test]
fn code_block_uses_minimum_box_height_for_empty_lines() {
    let block = SurfaceCodeBlock::new(Vec::new(), 0, 0);

    assert_eq!(
        block.height(),
        ViewerCodeBlockMetrics::block_height_from_line_count_with_scale_px(1, 1.0)
    );
    assert_eq!(block.text_for_tests(), "");
}

#[test]
fn math_block_uses_rendered_text_and_fallback_text() {
    let rendered = SurfaceMathBlock {
        image: Some(super::SurfaceSvgImage::from_image(
            image::RgbaImage::from_pixel(10, 7, Rgba([1, 2, 3, 255])),
        )),
        fallback_text: "fallback".to_string(),
        typography: SurfaceTypographyConfig::default(),
    };

    assert_eq!(rendered.text(), "math-svg:rendered");
    assert_eq!(rendered.height(), 7 + 36);

    let fallback = SurfaceMathBlock {
        image: None,
        fallback_text: "raw expression".to_string(),
        typography: SurfaceTypographyConfig::default(),
    };
    assert_eq!(fallback.text(), "raw expression");
    assert_eq!(fallback.height(), 74);
}

#[test]
fn diagram_block_uses_fallback_size_and_text() {
    let rendered = SurfaceDiagramBlock::rendered("<svg><text>ok</text></svg>");
    assert_eq!(rendered.fallback_text(), "Rendered diagram");
    assert!(rendered.height() >= 38);

    let fallback = SurfaceDiagramBlock {
        image: None,
        fallback_text: "fallback diagram".to_string(),
    };
    assert_eq!(fallback.fallback_text(), "fallback diagram");
    assert_eq!(fallback.height(), 120 + 36);
}

#[test]
fn image_block_from_path_scales_large_image_to_surface_width()
-> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = std::env::temp_dir();
    let large_path = temp_dir.join("kdv-large-media.png");
    let small_path = temp_dir.join("kdv-small-media.png");
    image::RgbaImage::from_pixel(1400, 40, Rgba([3, 4, 5, 255])).save(&large_path)?;
    image::RgbaImage::from_pixel(32, 14, Rgba([6, 7, 8, 255])).save(&small_path)?;

    let scaled = SurfaceImageBlock::from_path(&large_path, None, "large".to_string())
        .ok_or(std::io::Error::other("large image block"))?;
    let tiny = SurfaceImageBlock::from_path(&small_path, None, "small".to_string())
        .ok_or(std::io::Error::other("small image block"))?;

    assert_eq!(tiny.image.width(), 32);
    assert_eq!(scaled.image.width(), SURFACE_CONTENT_WIDTH);
    assert_eq!(scaled.height(), scaled.image.height() + 18 * 2);
    assert_eq!(scaled.alt_for_tests(), "large");
    Ok(())
}

#[test]
fn image_block_from_path_rasterizes_svg_file() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::temp_dir().join("kdv-local-svg-image.svg");
    std::fs::write(
        &path,
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="32" height="18"><rect width="32" height="18" fill="#111111"/></svg>"##,
    )?;

    let block = SurfaceImageBlock::from_path(&path, None, "svg".to_string())
        .ok_or(std::io::Error::other("svg image block"))?;

    assert_eq!(block.image.width(), 32);
    assert_eq!(block.image.height(), 18);
    assert_eq!(block.alt_for_tests(), "svg");
    Ok(())
}

#[test]
fn span_metrics_uses_surface_line_estimation() {
    let width =
        SurfaceSpanMetrics::estimated_width(&SurfaceTextSpan::plain("abc".to_string()), 14.0);
    assert!(width > 0);
}

#[test]
fn math_fallback_text_prefers_trimmed_expression_when_svg_rendered() {
    let output = KrrRenderOutput::svg("<svg/>".to_string());

    assert_eq!(super::math_fallback_text("  x + 1  ", &output), "x + 1");
}

#[test]
fn math_fallback_text_uses_raw_payload_when_svg_is_missing() {
    let output = KrrRenderOutput {
        payload: KrrRenderPayload::Raw("raw math".to_string()),
        diagnostics: Vec::new(),
    };

    assert_eq!(super::math_fallback_text("ignored", &output), "raw math");
}
