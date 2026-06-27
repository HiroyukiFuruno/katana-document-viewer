use crate::export_surface::{SurfaceBlock, SurfaceBlockFactory, SurfacePainter};
use crate::export_surface_helpers::{SURFACE_WIDTH, SurfaceHelpers};
use crate::export_surface_line::SurfaceTypographyConfig;
use crate::{BuildGraph, KdvThemeSnapshot, PreviewConfig, ViewerViewport};
use image::RgbaImage;

const RGBA_CHANNELS: usize = 4;
pub const KDV_INTERACTIVE_PREVIEW_SURFACE_PADDING_PX: u16 = 12;
pub const KDV_INTERACTIVE_PREVIEW_SURFACE_HORIZONTAL_PADDING_PX: u16 =
    KDV_INTERACTIVE_PREVIEW_SURFACE_PADDING_PX;
pub const KDV_VIEWER_SURFACE_PADDING_PX: u16 = 56;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KdvPreviewSurface {
    pub width: u32,
    pub height: u32,
    pub origin_y: u32,
    pub content_height: u32,
    pub rgba: Vec<u8>,
}

pub struct KdvPreviewSurfaceFactory;
pub struct KdvPdfSurfaceFactory;

impl KdvPreviewSurfaceFactory {
    pub fn create_from_config(
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        config: &PreviewConfig,
    ) -> KdvPreviewSurface {
        Self::create_with_typography(
            graph,
            theme,
            config.viewport,
            config.scroll_offset,
            preview_typography(config),
        )
    }

    pub fn create(
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        _viewport: ViewerViewport,
        _scroll_offset: f32,
    ) -> KdvPreviewSurface {
        Self::create_with_typography(
            graph,
            theme,
            _viewport,
            _scroll_offset,
            SurfaceTypographyConfig::default(),
        )
    }

    fn create_with_typography(
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        _viewport: ViewerViewport,
        _scroll_offset: f32,
        typography: SurfaceTypographyConfig,
    ) -> KdvPreviewSurface {
        let blocks = SurfaceBlockFactory::create_with_typography(graph, theme, typography);
        let content_height = Self::content_height(&blocks);
        let mut image = Self::base_image(theme, content_height);
        SurfacePainter::paint(&mut image, &blocks, theme);
        KdvPreviewSurface {
            width: SURFACE_WIDTH,
            height: content_height,
            origin_y: 0,
            content_height,
            rgba: image.into_raw(),
        }
    }

    fn content_height(blocks: &[SurfaceBlock]) -> u32 {
        SurfaceHelpers::surface_block_height(blocks.iter().map(SurfaceBlock::height))
    }

    fn base_image(theme: &KdvThemeSnapshot, height: u32) -> RgbaImage {
        let background = SurfaceHelpers::parse_color(&theme.background);
        RgbaImage::from_pixel(SURFACE_WIDTH, height, background)
    }
}

fn preview_typography(config: &PreviewConfig) -> SurfaceTypographyConfig {
    match config.base_font_size {
        Some(base_font_size) => SurfaceTypographyConfig::from_body_font_size(base_font_size),
        None => SurfaceTypographyConfig::default(),
    }
}

impl KdvPdfSurfaceFactory {
    pub fn create(graph: &BuildGraph, theme: &KdvThemeSnapshot) -> KdvPreviewSurface {
        let blocks = SurfaceBlockFactory::create(graph, theme);
        let (pages, _, _) = SurfacePainter::paint_pages(&blocks, theme);
        let height = pages.iter().map(RgbaImage::height).sum();
        let mut rgba = Vec::with_capacity(SURFACE_WIDTH as usize * height as usize * RGBA_CHANNELS);
        for page in pages {
            rgba.extend_from_slice(page.as_raw());
        }
        KdvPreviewSurface {
            width: SURFACE_WIDTH,
            height,
            origin_y: 0,
            content_height: height,
            rgba,
        }
    }
}

#[cfg(test)]
#[path = "preview_surface_tests.rs"]
mod tests;
