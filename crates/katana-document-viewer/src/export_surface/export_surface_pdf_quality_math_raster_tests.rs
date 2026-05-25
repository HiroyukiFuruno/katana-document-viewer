use crate::export_surface::DocumentSurfaceFactory;
use crate::export_surface::test_modules::test_support::SurfaceTestSupport;
use crate::export_surface_svg::SurfaceSvgImage;
use crate::export_surface_svg::SurfaceSvgRasterizer;
use crate::render_runtime::{KrrMathMode, KrrRenderRuntimeAdapter};

const MATH_RASTER_TEST_MAX_WIDTH: u32 = 240;
const FONT_SCALE_TOLERANCE: f32 = 0.2;

#[test]
fn pdf_math_is_not_upscaled_beyond_natural_svg_size() -> Result<(), Box<dyn std::error::Error>> {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="160" height="32"><rect width="160" height="32"/></svg>"#;

    let image = SurfaceSvgRasterizer::rasterize(svg, 760)
        .ok_or_else(|| std::io::Error::other("math-like svg should rasterize"))?;

    assert_eq!(image.image.width(), 160);
    assert_eq!(image.image.height(), 32);
    Ok(())
}

#[test]
fn pdf_math_rasterization_scales_with_root_css_font_size_setting()
-> Result<(), Box<dyn std::error::Error>> {
    let case = MathRasterizationScaleCase {};
    case.assert_scale()
}

#[test]
fn pdf_surface_renders_math_emoji_and_code_combo() -> Result<(), Box<dyn std::error::Error>> {
    let graph = SurfaceTestSupport::graph_from_markdown(
        "math.md",
        "# math\n\n$E=mc^2$\n\n```math\nx^2\n```".to_string(),
    )?;
    let _ = DocumentSurfaceFactory::create(&graph, &crate::KdvThemeSnapshot::katana_light());
    Ok(())
}

struct MathRasterizationScaleCase;

impl MathRasterizationScaleCase {
    fn assert_scale(&self) -> Result<(), Box<dyn std::error::Error>> {
        let inline = Self::render_inline_math_svg()?;
        let (small, large) = Self::rasterize_sizes(inline)?;
        Self::assert_width_and_height_scale(small.image.width(), large.image.width(), "width");
        Self::assert_width_and_height_scale(small.image.height(), large.image.height(), "height");
        Self::assert_ratio_near_double(small.image.width(), large.image.width())?;
        Ok(())
    }

    fn render_inline_math_svg() -> Result<String, Box<dyn std::error::Error>> {
        let render = KrrRenderRuntimeAdapter::render_math_tex_with_theme(
            r"E = mc^2",
            KrrMathMode::Inline,
            Some(crate::KdvThemeSnapshot::katana_light().krr_math_theme()),
        );
        let inline = render
            .svg_payload()
            .ok_or("inline math SVG should be produced")?;
        Ok(inline.to_string())
    }

    fn rasterize_sizes(
        inline: String,
    ) -> Result<(SurfaceSvgImage, SurfaceSvgImage), Box<dyn std::error::Error>> {
        let small = Self::rasterize_svg_with_font_size(
            &inline,
            Some(crate::export_surface_line::BODY_FONT_SIZE),
        )?;
        let large = Self::rasterize_svg_with_font_size(
            &inline,
            Some(crate::export_surface_line::BODY_FONT_SIZE * 2.0),
        )?;
        Ok((small, large))
    }

    fn rasterize_svg_with_font_size(
        inline: &str,
        root_font_size: Option<f32>,
    ) -> Result<SurfaceSvgImage, Box<dyn std::error::Error>> {
        let small = SurfaceSvgRasterizer::rasterize_with_root_font_size(
            inline,
            MATH_RASTER_TEST_MAX_WIDTH,
            root_font_size,
        )
        .ok_or("inline math should rasterize with specified root font size")?;
        Ok(small)
    }

    fn assert_width_and_height_scale(small: u32, large: u32, dimension: &str) {
        assert!(
            large > small,
            "inline math with doubled root font-size should produce larger rasterized {dimension}"
        );
    }

    fn assert_ratio_near_double(
        base_width: u32,
        scaled_width: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let width_ratio = scaled_width as f32 / base_width as f32;
        if (width_ratio - 2.0).abs() >= FONT_SCALE_TOLERANCE {
            return Err(format!(
                "font-size scaling for ex/em units should remain near linear: {width_ratio}"
            )
            .into());
        }
        Ok(())
    }
}
