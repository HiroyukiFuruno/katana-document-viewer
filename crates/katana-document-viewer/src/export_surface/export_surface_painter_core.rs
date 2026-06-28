use super::{
    KdvThemeSnapshot, PAGE_PADDING, SURFACE_PAGE_HEIGHT, SURFACE_WIDTH, SurfaceBlock,
    SurfaceHelpers, SurfacePageLinkMetadata, SurfacePagePaintRequest, SurfacePagePlan,
    SurfacePaintPages, SurfacePaintPalette, SurfacePainter, SurfaceTextPainter,
};
use image::RgbaImage;

impl SurfacePainter {
    pub(crate) fn paint(image: &mut RgbaImage, blocks: &[SurfaceBlock], theme: &KdvThemeSnapshot) {
        Self::paint_at(image, blocks, theme, PAGE_PADDING);
    }

    fn paint_at(
        image: &mut RgbaImage,
        blocks: &[SurfaceBlock],
        theme: &KdvThemeSnapshot,
        start_y: u32,
    ) {
        let palette = SurfacePaintPalette::from_theme(theme);
        SurfaceTextPainter::with_system_fonts(|painter| {
            let mut y = start_y;
            for block in blocks {
                Self::paint_block(image, block, y, painter, &palette);
                y += block.height();
            }
        });
    }

    pub(crate) fn paint_pages(
        blocks: &[SurfaceBlock],
        theme: &KdvThemeSnapshot,
    ) -> SurfacePaintPages {
        let palette = SurfacePaintPalette::from_theme(theme);
        let background = SurfaceHelpers::parse_color(&theme.background);
        let mut pages = Vec::new();
        let mut link_annotations = Vec::new();
        let mut link_anchors = Vec::new();
        let plan = SurfacePagePlan::from_blocks(blocks);
        SurfaceTextPainter::with_system_fonts(|painter| {
            for (page_index, block_indexes) in plan.pages.iter().enumerate() {
                let request = SurfacePagePaintRequest {
                    blocks,
                    block_indexes,
                    page_index,
                    painter,
                    palette: &palette,
                    links: SurfacePageLinkMetadata {
                        annotations: &mut link_annotations,
                        anchors: &mut link_anchors,
                    },
                };
                pages.push(Self::paint_page(request, background));
            }
        });
        (pages, link_annotations, link_anchors)
    }

    pub(super) fn paint_page(
        request: SurfacePagePaintRequest<'_>,
        background: image::Rgba<u8>,
    ) -> RgbaImage {
        let mut page = RgbaImage::from_pixel(SURFACE_WIDTH, SURFACE_PAGE_HEIGHT, background);
        Self::paint_blocks_on_page(&mut page, request);
        page
    }

    pub(super) fn paint_blocks_on_page(page: &mut RgbaImage, request: SurfacePagePaintRequest<'_>) {
        let mut y = PAGE_PADDING;
        for block_index in request.block_indexes {
            let block = &request.blocks[*block_index];
            Self::paint_block(page, block, y, request.painter, request.palette);
            Self::append_link_metadata(
                request.links.annotations,
                request.links.anchors,
                block,
                request.page_index,
                y,
            );
            y += block.height();
        }
    }

    pub(super) fn paint_block(
        image: &mut RgbaImage,
        block: &SurfaceBlock,
        y: u32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        match block {
            SurfaceBlock::Line(line) => Self::paint_line(image, line, y, painter, palette),
            SurfaceBlock::Code(code) => Self::paint_code_block(image, code, y, painter, palette),
            SurfaceBlock::Math(math) => Self::paint_math_block(image, math, y, painter, palette),
            SurfaceBlock::Table(table) => Self::paint_table(image, table, y, painter, palette),
            SurfaceBlock::Diagram(diagram) => {
                Self::paint_diagram(image, diagram, y, painter, palette)
            }
            SurfaceBlock::Image(local_image) => Self::paint_image(image, local_image, y),
            SurfaceBlock::BadgeRow(row) => Self::paint_badge_row(image, row, y, painter, palette),
            SurfaceBlock::Alert(alert) => Self::paint_alert(image, alert, y, painter, palette),
            SurfaceBlock::Rule => Self::paint_rule(image, y, palette),
        }
    }
}

#[cfg(test)]
#[path = "export_surface_painter_core_tests.rs"]
mod tests;
