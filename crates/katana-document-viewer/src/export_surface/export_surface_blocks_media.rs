use super::{
    CODE_BLOCK_MARGIN, CODE_EMPTY_BLOCK_MIN_HEIGHT, CODE_VERTICAL_PADDING, DIAGRAM_FALLBACK_HEIGHT,
    DIAGRAM_MAX_WIDTH, DIAGRAM_VERTICAL_MARGIN, MATH_FALLBACK_HEIGHT, MATH_VERTICAL_MARGIN,
};
use crate::export_surface_helpers::SURFACE_CONTENT_WIDTH;
use crate::export_surface_line::SurfaceLine;
use crate::export_surface_span::SurfaceTextSpan;
use crate::export_surface_svg::{SurfaceSvgImage, SurfaceSvgRasterizer};
use crate::render_runtime::{KrrMathMode, KrrRenderOutput, KrrRenderRuntimeAdapter};
use image::RgbaImage;
use katana_render_runtime::RenderThemeSnapshot;

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
        let content_height = self.lines.iter().map(SurfaceLine::line_height).sum::<u32>()
            + CODE_VERTICAL_PADDING * 2;
        content_height.max(CODE_EMPTY_BLOCK_MIN_HEIGHT)
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
}

impl SurfaceMathBlock {
    pub(crate) fn new(expression: &str, theme: Option<RenderThemeSnapshot>) -> Self {
        let output = KrrRenderRuntimeAdapter::render_math_tex_with_theme(
            expression,
            KrrMathMode::Display,
            theme,
        );
        let image = output.svg_payload().and_then(|svg| {
            SurfaceSvgRasterizer::rasterize_with_root_font_size(
                svg,
                super::super::MATH_MAX_WIDTH,
                Some(super::super::BODY_FONT_SIZE),
            )
        });
        Self {
            image,
            fallback_text: math_fallback_text(expression, &output),
        }
    }

    pub(crate) fn height(&self) -> u32 {
        self.image
            .as_ref()
            .map(|rendered| rendered.image.height() + MATH_VERTICAL_MARGIN * 2)
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

    #[cfg(test)]
    pub(crate) fn for_tests(image: Option<RgbaImage>, fallback_text: String) -> Self {
        Self {
            image: image.map(|image| SurfaceSvgImage { image }),
            fallback_text,
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
            image: SurfaceSvgRasterizer::rasterize(svg, DIAGRAM_MAX_WIDTH),
            fallback_text: "Rendered diagram".to_string(),
        }
    }

    pub(crate) fn height(&self) -> u32 {
        let content_height = self
            .image
            .as_ref()
            .map(|rendered| rendered.image.height())
            .unwrap_or(DIAGRAM_FALLBACK_HEIGHT);
        content_height + DIAGRAM_VERTICAL_MARGIN * 2
    }

    pub(crate) fn fallback_text(&self) -> &str {
        &self.fallback_text
    }
}

pub(crate) struct SurfaceImageBlock {
    pub(crate) image: RgbaImage,
    _alt: String,
}

impl SurfaceImageBlock {
    pub(crate) fn from_path(
        path: &std::path::Path,
        requested_width: Option<u32>,
        alt: String,
    ) -> Option<Self> {
        let image = image::open(path).ok()?.to_rgba8();
        let image = scaled_image(image, requested_width);
        Some(Self { image, _alt: alt })
    }

    pub(crate) fn height(&self) -> u32 {
        self.image.height() + super::super::IMAGE_VERTICAL_MARGIN * 2
    }

    #[cfg(test)]
    pub(crate) fn alt_for_tests(&self) -> String {
        self._alt.clone()
    }
}

fn scaled_image(image: RgbaImage, requested_width: Option<u32>) -> RgbaImage {
    let max_width = requested_width
        .unwrap_or(image.width())
        .min(SURFACE_CONTENT_WIDTH);
    if image.width() <= max_width {
        return image;
    }
    let height = (image.height() as f32 * max_width as f32 / image.width() as f32)
        .round()
        .max(1.0) as u32;
    image::imageops::resize(
        &image,
        max_width,
        height,
        image::imageops::FilterType::Lanczos3,
    )
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
