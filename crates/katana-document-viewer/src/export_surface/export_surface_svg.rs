use image::RgbaImage;
use resvg::usvg;

mod export_surface_svg_preprocess;
mod export_surface_svg_raster;

use export_surface_svg_preprocess::preprocess_for_rasterizer;
use export_surface_svg_raster::RasterTarget;

pub(crate) struct SurfaceSvgImage {
    pub(crate) image: RgbaImage,
}

pub(crate) struct SurfaceSvgRasterizer;

impl SurfaceSvgRasterizer {
    pub(crate) fn rasterize(svg_text: &str, max_width: u32) -> Option<SurfaceSvgImage> {
        Self::rasterize_with_root_font_size(svg_text, max_width, None)
    }

    pub(crate) fn rasterize_with_root_font_size(
        svg_text: &str,
        max_width: u32,
        root_font_size: Option<f32>,
    ) -> Option<SurfaceSvgImage> {
        let compatible_svg = preprocess_for_rasterizer(
            svg_text,
            root_font_size.filter(|size| size.is_finite() && *size > 0.0),
        );
        let tree = usvg::Tree::from_str(
            &compatible_svg,
            &export_surface_svg_raster::rasterizer_options(),
        )
        .ok()?;
        let target = RasterTarget::new(tree.size(), max_width);
        let pixmap = target.render(&tree)?;
        let image = RgbaImage::from_raw(target.width(), target.height(), pixmap.take())?;
        Some(SurfaceSvgImage { image })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FONT_SCALE_TOLERANCE: f32 = 0.2;

    #[test]
    fn rasterize_keep_diagram_scale_under_original_max() -> Result<(), Box<dyn std::error::Error>> {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20"><rect width="20" height="20"/></svg>"#;
        let image = SurfaceSvgRasterizer::rasterize(svg, 100)
            .ok_or_else(|| std::io::Error::other("diagram-like svg should rasterize"))?;

        assert_eq!(image.image.width(), 20);
        Ok(())
    }

    #[test]
    fn rasterize_keeps_small_svg_from_over_scaling() -> Result<(), Box<dyn std::error::Error>> {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20"><rect width="20" height="20"/></svg>"#;
        let image = SurfaceSvgRasterizer::rasterize(svg, 200)
            .ok_or_else(|| std::io::Error::other("svg should rasterize"))?;

        assert_eq!(image.image.width(), 20);
        Ok(())
    }

    #[test]
    fn rasterize_keeps_root_font_size_as_css_unit_for_ex_sizing()
    -> Result<(), Box<dyn std::error::Error>> {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="8.704ex" height="1.912ex"><text x="0" y="10">E = mc^2</text></svg>"#;
        let root_font_size = 24.0;
        let small =
            SurfaceSvgRasterizer::rasterize_with_root_font_size(svg, 1000, Some(root_font_size))
                .ok_or_else(|| std::io::Error::other("math-like svg should rasterize"))?;
        let large = SurfaceSvgRasterizer::rasterize_with_root_font_size(
            svg,
            1000,
            Some(root_font_size * 2.0),
        )
        .ok_or_else(|| std::io::Error::other("math-like svg should rasterize"))?;

        assert!(large.image.width() > small.image.width());
        assert!(large.image.height() > small.image.height());
        assert!(
            ((large.image.width() as f32 / small.image.width() as f32) - 2.0).abs()
                < FONT_SCALE_TOLERANCE,
        );
        Ok(())
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

        assert!(
            processed.contains("style=\"color:#000; font-size:16px;\"")
                || processed.contains("style=\"color:#000;font-size:16px;\"")
                || processed.contains("style=\"font-size:16px; color:#000;\""),
            "existing svg style should keep original entries: {processed}"
        );
    }
}
