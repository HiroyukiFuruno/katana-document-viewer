use super::image_surface_cache::ViewerImageSurfaceCache;
use super::{ViewerImageSurface, ViewerImageSurfaceError, ViewerImageSurfaceFactory};
use crate::artifact::{Artifact, ArtifactFormat};

impl ViewerImageSurfaceFactory {
    const RASTER_CONTENT_SCALE: u32 = 100;
    const SURFACE_RENDERER_TOKEN: &'static str =
        "image-surface-resvg-usvg-system-embedded-epaint-fonts-v3";
    const SVG_CONTENT_SCALE: u32 = 200;
    const STORYBOOK_RETINA_CANVAS_SCALE: f32 = 2.0;

    pub fn from_math_artifact(
        artifact: &Artifact,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        match artifact.manifest.format {
            ArtifactFormat::Svg => Self::from_svg_artifact_with_root_font_size(
                artifact,
                crate::export_surface::MATH_MAX_WIDTH,
                Some(crate::export_surface::BODY_FONT_SIZE),
            ),
            format => Err(ViewerImageSurfaceError::UnsupportedFormat(format)),
        }
    }

    pub fn from_artifact(
        artifact: &Artifact,
        max_width: u32,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        match artifact.manifest.format {
            ArtifactFormat::Svg => Self::from_svg_artifact(artifact, max_width),
            ArtifactFormat::Png
            | ArtifactFormat::Jpeg
            | ArtifactFormat::Gif
            | ArtifactFormat::Webp
            | ArtifactFormat::Bmp => Self::from_raster_artifact(artifact),
            format => Err(ViewerImageSurfaceError::UnsupportedFormat(format)),
        }
    }

    pub fn from_diagram_artifact(
        artifact: &Artifact,
        max_width: u32,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        Self::from_diagram_artifact_with_optional_background(artifact, max_width, None)
    }

    pub fn from_export_surface_diagram_artifact(
        artifact: &Artifact,
        max_width: u32,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        match artifact.manifest.format {
            ArtifactFormat::Svg => Self::from_export_surface_svg_artifact(artifact, max_width),
            _ => Self::from_artifact(artifact, max_width),
        }
    }

    pub fn from_diagram_artifact_with_background(
        artifact: &Artifact,
        max_width: u32,
        background: [u8; 4],
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        Self::from_diagram_artifact_with_optional_background(artifact, max_width, Some(background))
    }

    pub fn from_fullscreen_diagram_artifact(
        artifact: &Artifact,
        max_width: u32,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        Self::from_fullscreen_diagram_artifact_with_optional_background(artifact, max_width, None)
    }

    pub fn from_fullscreen_diagram_artifact_with_background(
        artifact: &Artifact,
        max_width: u32,
        background: [u8; 4],
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        Self::from_fullscreen_diagram_artifact_with_optional_background(
            artifact,
            max_width,
            Some(background),
        )
    }

    fn from_diagram_artifact_with_optional_background(
        artifact: &Artifact,
        max_width: u32,
        background: Option<[u8; 4]>,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        match artifact.manifest.format {
            ArtifactFormat::Svg => {
                let max_width = diagram_display_max_width(max_width);
                let svg = std::str::from_utf8(&artifact.bytes.bytes)
                    .map_err(|_| ViewerImageSurfaceError::InvalidSvgEncoding)?;
                let content_scale = Self::diagram_preview_content_scale(svg, max_width, None)
                    .unwrap_or(Self::SVG_CONTENT_SCALE);
                Self::from_svg_artifact_with_root_font_size_and_content_scale(
                    artifact,
                    max_width,
                    None,
                    content_scale,
                    background,
                )
                .map(|surface| fit_diagram_display_size(surface, max_width))
            }
            _ => Self::from_artifact(artifact, max_width),
        }
    }

    fn from_fullscreen_diagram_artifact_with_optional_background(
        artifact: &Artifact,
        max_width: u32,
        background: Option<[u8; 4]>,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        match artifact.manifest.format {
            ArtifactFormat::Svg => {
                let max_width = diagram_display_max_width(max_width);
                let svg = std::str::from_utf8(&artifact.bytes.bytes)
                    .map_err(|_| ViewerImageSurfaceError::InvalidSvgEncoding)?;
                let content_scale = Self::diagram_fullscreen_content_scale(svg, max_width, None)
                    .unwrap_or(Self::SVG_CONTENT_SCALE);
                Self::from_svg_artifact_with_root_font_size_and_content_scale(
                    artifact,
                    max_width,
                    None,
                    content_scale,
                    background,
                )
            }
            _ => Self::from_artifact(artifact, max_width),
        }
    }

    fn diagram_preview_content_scale(
        svg: &str,
        max_width: u32,
        root_font_size: Option<f32>,
    ) -> Option<u32> {
        let (display_width, _) =
            crate::export_surface_svg::SurfaceSvgRasterizer::display_size(svg, root_font_size)?;
        let preview_width = display_width * diagram_display_scale(display_width, max_width);
        let rounded_preview_width = preview_width.round().max(1.0);
        let target_physical_width = rounded_preview_width * Self::STORYBOOK_RETINA_CANVAS_SCALE;
        let aligned_scale = (target_physical_width * 100.0 / display_width).floor() as u32;
        Some(aligned_scale.max(Self::SVG_CONTENT_SCALE))
    }

    fn diagram_fullscreen_content_scale(
        svg: &str,
        max_width: u32,
        root_font_size: Option<f32>,
    ) -> Option<u32> {
        let (display_width, _) =
            crate::export_surface_svg::SurfaceSvgRasterizer::display_size(svg, root_font_size)?;
        let target_width = diagram_display_max_width(max_width) as f32;
        let target_physical_width = target_width * Self::STORYBOOK_RETINA_CANVAS_SCALE;
        let aligned_scale = (target_physical_width * 100.0 / display_width).ceil() as u32;
        Some(aligned_scale.max(Self::SVG_CONTENT_SCALE))
    }

    pub fn from_svg_str(
        fingerprint: impl Into<String>,
        svg: &str,
        max_width: u32,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        Self::from_svg_str_with_root_font_size(fingerprint, svg, max_width, None)
    }

    pub fn from_svg_str_with_root_font_size(
        fingerprint: impl Into<String>,
        svg: &str,
        max_width: u32,
        root_font_size: Option<f32>,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        Self::from_svg_str_with_root_font_size_and_content_scale(
            fingerprint,
            svg,
            max_width,
            root_font_size,
            Self::SVG_CONTENT_SCALE,
            None,
        )
    }

    fn from_svg_str_with_root_font_size_and_content_scale(
        fingerprint: impl Into<String>,
        svg: &str,
        max_width: u32,
        root_font_size: Option<f32>,
        content_scale: u32,
        background: Option<[u8; 4]>,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        let fingerprint = Self::svg_surface_fingerprint(
            fingerprint.into(),
            svg,
            max_width,
            root_font_size,
            content_scale,
            background,
        );
        if let Some(surface) = ViewerImageSurfaceCache::get(&fingerprint) {
            return Ok(surface);
        }
        let image = Self::rasterize_svg(svg, max_width, root_font_size, content_scale)?;
        Ok(Self::cache_surface(
            fingerprint,
            image,
            content_scale,
            background,
        ))
    }

    fn from_svg_artifact(
        artifact: &Artifact,
        max_width: u32,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        Self::from_svg_artifact_with_root_font_size(artifact, max_width, None)
    }

    fn from_svg_artifact_with_root_font_size(
        artifact: &Artifact,
        max_width: u32,
        root_font_size: Option<f32>,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        Self::from_svg_artifact_with_root_font_size_and_content_scale(
            artifact,
            max_width,
            root_font_size,
            Self::SVG_CONTENT_SCALE,
            None,
        )
    }

    fn from_svg_artifact_with_root_font_size_and_content_scale(
        artifact: &Artifact,
        max_width: u32,
        root_font_size: Option<f32>,
        content_scale: u32,
        background: Option<[u8; 4]>,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        let svg = std::str::from_utf8(&artifact.bytes.bytes)
            .map_err(|_| ViewerImageSurfaceError::InvalidSvgEncoding)?;
        Self::from_svg_str_with_root_font_size_and_content_scale(
            artifact.manifest.id.0.clone(),
            svg,
            max_width,
            root_font_size,
            content_scale,
            background,
        )
    }

    fn from_export_surface_svg_artifact(
        artifact: &Artifact,
        max_width: u32,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        let svg = std::str::from_utf8(&artifact.bytes.bytes)
            .map_err(|_| ViewerImageSurfaceError::InvalidSvgEncoding)?;
        let fingerprint = Self::surface_fingerprint(
            artifact.manifest.id.0.clone(),
            svg.as_bytes(),
            max_width,
            None,
            Self::RASTER_CONTENT_SCALE,
            None,
        );
        if let Some(surface) = ViewerImageSurfaceCache::get(&fingerprint) {
            return Ok(surface);
        }
        let image = crate::export_surface_svg::SurfaceSvgRasterizer::rasterize(svg, max_width)
            .ok_or(ViewerImageSurfaceError::InvalidSvg)?
            .image;
        Ok(ViewerImageSurfaceCache::put(ViewerImageSurface {
            fingerprint,
            width: image.width(),
            height: image.height(),
            display_width: image.width() as f32,
            display_height: image.height() as f32,
            content_scale: Self::RASTER_CONTENT_SCALE,
            rgba: image.into_raw(),
        }))
    }

    fn from_raster_artifact(
        artifact: &Artifact,
    ) -> Result<ViewerImageSurface, ViewerImageSurfaceError> {
        let image = image::load_from_memory(&artifact.bytes.bytes)
            .map_err(|error| ViewerImageSurfaceError::InvalidRaster(error.to_string()))?
            .to_rgba8();
        let fingerprint = Self::surface_fingerprint(
            artifact.manifest.id.0.clone(),
            &artifact.bytes.bytes,
            image.width(),
            None,
            Self::RASTER_CONTENT_SCALE,
            None,
        );
        if let Some(surface) = ViewerImageSurfaceCache::get(&fingerprint) {
            return Ok(surface);
        }
        Ok(ViewerImageSurfaceCache::put(ViewerImageSurface {
            fingerprint,
            width: image.width(),
            height: image.height(),
            display_width: image.width() as f32,
            display_height: image.height() as f32,
            content_scale: Self::RASTER_CONTENT_SCALE,
            rgba: image.into_raw(),
        }))
    }

    pub(crate) fn surface_fingerprint(
        base: String,
        bytes: &[u8],
        max_width: u32,
        root_font_size: Option<f32>,
        content_scale: u32,
        background: Option<[u8; 4]>,
    ) -> String {
        format!(
            "{}:bytes={}:max_width={}:root_font={}:scale={}:background={}:renderer={}",
            base,
            Self::hash(bytes),
            max_width,
            root_font_size
                .map(|value| value.to_bits().to_string())
                .unwrap_or_else(|| "none".to_string()),
            content_scale,
            background
                .map(background_fingerprint)
                .unwrap_or_else(|| "transparent".to_string()),
            Self::SURFACE_RENDERER_TOKEN
        )
    }

    fn hash(bytes: &[u8]) -> String {
        let mut hash = 0xcbf29ce484222325_u64;
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("{hash:x}")
    }
}

fn background_fingerprint(background: [u8; 4]) -> String {
    format!(
        "{:02x}{:02x}{:02x}{:02x}",
        background[0], background[1], background[2], background[3]
    )
}

fn fit_diagram_display_size(mut surface: ViewerImageSurface, max_width: u32) -> ViewerImageSurface {
    if !surface.display_width.is_finite()
        || !surface.display_height.is_finite()
        || surface.display_width <= 0.0
        || surface.display_height <= 0.0
    {
        return surface;
    }
    let scale = diagram_display_scale(surface.display_width, max_width);
    surface.display_width = (surface.display_width * scale).max(1.0);
    surface.display_height = (surface.display_height * scale).max(1.0);
    surface
}

fn diagram_display_scale(display_width: f32, max_width: u32) -> f32 {
    let max_width = diagram_display_max_width(max_width) as f32;
    let fit_scale = (max_width / display_width).min(1.0);
    fit_scale.min(crate::viewer::VIEWER_DIAGRAM_DISPLAY_SCALE)
}

fn diagram_display_max_width(max_width: u32) -> u32 {
    max_width.clamp(1, crate::viewer::VIEWER_DIAGRAM_DISPLAY_MAX_WIDTH)
}

#[cfg(test)]
#[path = "image_surface_factory_private_tests.rs"]
mod tests;
