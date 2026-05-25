use crate::forge::BuildGraph;
use crate::theme::KdvThemeSnapshot;
use image::RgbaImage;

mod export_surface_block_factory;
mod export_surface_blocks;
mod export_surface_painter;
mod icons;
mod markup;
mod page_plan;

use self::export_surface_block_factory::SurfaceBlockFactory;
use self::export_surface_painter::SurfacePainter;

pub(crate) use self::export_surface_blocks::{
    SurfaceAlertBlock, SurfaceBadge, SurfaceBadgeRowBlock, SurfaceBlock, SurfaceCodeBlock,
    SurfaceDiagramBlock, SurfaceImageBlock, SurfaceMathBlock, SurfaceSpanMetrics,
    SurfaceTableBlock, SurfaceTableCellPaint, SurfaceTableLayout,
};
pub(crate) use self::export_surface_painter::SurfacePaintPalette;

use crate::export_surface_helpers::{SURFACE_WIDTH, SurfaceHelpers};
pub(crate) use crate::export_surface_line::BODY_FONT_SIZE;

pub(crate) const TASK_MARKER_SIZE: u32 = 18;
pub(crate) const ALERT_ICON_SIZE: u32 = 20;
pub(crate) const IMAGE_VERTICAL_MARGIN: u32 = 18;
pub(crate) const MATH_MAX_WIDTH: u32 = 760;

pub(crate) struct DocumentSurface {
    pub(crate) image: RgbaImage,
    pub(crate) pages: Vec<RgbaImage>,
    pub(crate) link_annotations: Vec<SurfaceLinkAnnotation>,
    pub(crate) link_anchors: Vec<SurfaceLinkAnchor>,
}

pub(crate) struct SurfaceLinkAnnotation {
    pub(crate) page_index: usize,
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) target: String,
}

pub(crate) struct SurfaceLinkAnchor {
    pub(crate) id: String,
    pub(crate) page_index: usize,
    pub(crate) x: u32,
    pub(crate) y: u32,
}

pub(crate) struct DocumentSurfaceFactory;

impl DocumentSurfaceFactory {
    pub(crate) fn create(graph: &BuildGraph, theme: &KdvThemeSnapshot) -> DocumentSurface {
        let blocks = SurfaceBlockFactory::create(graph, theme);
        let height = SurfaceHelpers::surface_block_height(blocks.iter().map(SurfaceBlock::height));
        let background = SurfaceHelpers::parse_color(&theme.background);
        let mut image = RgbaImage::from_pixel(SURFACE_WIDTH, height, background);
        SurfacePainter::paint(&mut image, &blocks, theme);
        let (pages, link_annotations, link_anchors) = SurfacePainter::paint_pages(&blocks, theme);
        DocumentSurface {
            image,
            pages,
            link_annotations,
            link_anchors,
        }
    }
}

#[cfg(test)]
#[path = "export_surface_test_modules.rs"]
mod test_modules;
