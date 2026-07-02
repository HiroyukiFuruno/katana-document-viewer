use super::{
    CODE_BLOCK_MARGIN, DIAGRAM_FALLBACK_HEIGHT, DIAGRAM_MAX_WIDTH, DIAGRAM_VERTICAL_MARGIN,
    MATH_FALLBACK_HEIGHT, MATH_VERTICAL_MARGIN,
};
use crate::export_surface_line::{SurfaceLine, SurfaceTypographyConfig};
use crate::export_surface_span::SurfaceTextSpan;
use crate::export_surface_svg::{SurfaceSvgImage, SurfaceSvgRasterizer};
use crate::render_runtime::{KrrMathMode, KrrRenderOutput, KrrRenderRuntimeAdapter};
use crate::viewer::ViewerCodeBlockMetrics;
use katana_render_runtime::RenderThemeSnapshot;

const MATH_RAW_TEXT_SIZE: f32 = 28.0;

pub(crate) struct SurfaceCodeBlock {
    pub(crate) lines: Vec<SurfaceLine>,
    pub(crate) quote_depth: u32,
    pub(crate) indent_depth: u32,
}

impl SurfaceCodeBlock {
    pub(crate) fn new(lines: Vec<SurfaceLine>, quote_depth: u32, indent_depth: u32) -> Self {
        Self {
            lines,
            quote_depth,
            indent_depth,
        }
    }

    pub(crate) fn height(&self) -> u32 {
        self.box_height() + CODE_BLOCK_MARGIN * 2
    }

    pub(crate) fn box_height(&self) -> u32 {
        ViewerCodeBlockMetrics::box_height_from_line_count_with_scale_px(
            self.lines.len().max(1),
            self.code_scale(),
        )
    }

    pub(crate) fn apply_typography(&mut self, typography: SurfaceTypographyConfig) {
        for line in &mut self.lines {
            line.apply_typography(typography);
        }
    }

    fn code_scale(&self) -> f32 {
        self.lines.first().map_or(1.0, SurfaceLine::font_scale)
    }

    #[cfg(test)]
    pub(crate) fn text_for_tests(&self) -> String {
        self.lines
            .iter()
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[cfg(test)]
    pub(crate) fn debug_style_tags(&self) -> Vec<String> {
        self.lines
            .iter()
            .flat_map(SurfaceLine::debug_style_tags)
            .collect()
    }
}

pub(crate) struct SurfaceMathBlock {
    pub(crate) image: Option<SurfaceSvgImage>,
    fallback_text: String,
    typography: SurfaceTypographyConfig,
}

impl SurfaceMathBlock {
    pub(crate) fn new(expression: &str, theme: Option<RenderThemeSnapshot>) -> Self {
        let output = KrrRenderRuntimeAdapter::render_math_tex_with_theme(
            expression,
            KrrMathMode::Display,
            theme,
        );
        let image = output.svg_payload().and_then(|svg| {
            SurfaceSvgRasterizer::rasterize_export_surface_with_root_font_size(
                svg,
                super::super::MATH_MAX_WIDTH,
                Some(super::super::BODY_FONT_SIZE),
            )
        });
        Self {
            image,
            fallback_text: math_fallback_text(expression, &output),
            typography: SurfaceTypographyConfig::default(),
        }
    }

    pub(crate) fn height(&self) -> u32 {
        self.image
            .as_ref()
            .map(|rendered| rendered.display_height_px() + MATH_VERTICAL_MARGIN * 2)
            .unwrap_or(MATH_FALLBACK_HEIGHT)
    }

    #[cfg(test)]
    pub(crate) fn text(&self) -> String {
        if self.image.is_some() {
            return "math-svg:rendered".to_string();
        }
        self.fallback_text.clone()
    }

    pub(crate) fn fallback_text(&self) -> &str {
        &self.fallback_text
    }

    pub(crate) fn raw_text_size(&self) -> f32 {
        MATH_RAW_TEXT_SIZE * self.typography.body_scale()
    }

    pub(crate) fn apply_typography(&mut self, typography: SurfaceTypographyConfig) {
        self.typography = typography;
    }

    #[cfg(test)]
    pub(crate) fn for_tests(image: Option<image::RgbaImage>, fallback_text: String) -> Self {
        Self {
            image: image.map(SurfaceSvgImage::from_image),
            fallback_text,
            typography: SurfaceTypographyConfig::default(),
        }
    }
}

fn math_fallback_text(expression: &str, output: &KrrRenderOutput) -> String {
    if output.svg_payload().is_some() {
        return expression.trim().to_string();
    }
    output.raw_payload().to_string()
}

pub(crate) struct SurfaceDiagramBlock {
    pub(crate) image: Option<SurfaceSvgImage>,
    fallback_text: String,
}

impl SurfaceDiagramBlock {
    pub(crate) fn rendered(svg: &str) -> Self {
        Self {
            image: SurfaceSvgRasterizer::rasterize_for_export_surface(svg, DIAGRAM_MAX_WIDTH),
            fallback_text: "Rendered diagram".to_string(),
        }
    }

    pub(crate) fn raw(source: &str) -> Self {
        Self {
            image: None,
            fallback_text: source.trim().to_string(),
        }
    }

    pub(crate) fn height(&self) -> u32 {
        let content_height = self
            .image
            .as_ref()
            .map(SurfaceSvgImage::display_height_px)
            .unwrap_or(DIAGRAM_FALLBACK_HEIGHT);
        content_height + DIAGRAM_VERTICAL_MARGIN * 2
    }

    pub(crate) fn fallback_text(&self) -> &str {
        &self.fallback_text
    }
}

pub(crate) struct SurfaceSpanMetrics;

impl SurfaceSpanMetrics {
    pub(crate) fn estimated_width(span: &SurfaceTextSpan, font_size: f32) -> u32 {
        span.estimated_width(font_size)
    }
}

#[cfg(test)]
#[path = "export_surface_blocks_media_tests.rs"]
mod tests;
