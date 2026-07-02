use super::{
    CODE_HORIZONTAL_PADDING, CODE_VERTICAL_PADDING, DIAGRAM_VERTICAL_MARGIN, IMAGE_VERTICAL_MARGIN,
    PAGE_PADDING, SURFACE_CONTENT_WIDTH, SurfaceDiagramBlock, SurfaceHelpers, SurfaceImageBlock,
    SurfacePaintPalette, SurfacePainter, SurfaceTextLayout, SurfaceTextPainter,
};
use image::RgbaImage;

impl SurfacePainter {
    pub(super) fn paint_diagram(
        image: &mut RgbaImage,
        diagram: &SurfaceDiagramBlock,
        y: u32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        let Some(rendered) = &diagram.image else {
            Self::paint_pending_diagram(image, diagram, y, painter, palette);
            return;
        };
        let display_width = rendered.display_width_px();
        let x = PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(display_width) / 2;
        SurfaceHelpers::paste_rgba_resized(
            image,
            &rendered.image,
            x,
            y + DIAGRAM_VERTICAL_MARGIN,
            display_width,
            rendered.display_height_px(),
        );
    }

    pub(super) fn paint_image(image: &mut RgbaImage, block: &SurfaceImageBlock, y: u32) {
        let x = PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(block.display_width) / 2;
        SurfaceHelpers::paste_rgba_resized(
            image,
            &block.image,
            x,
            y + IMAGE_VERTICAL_MARGIN,
            block.display_width,
            block.display_height,
        );
    }

    fn paint_pending_diagram(
        image: &mut RgbaImage,
        diagram: &SurfaceDiagramBlock,
        y: u32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        let box_height = diagram.height().saturating_sub(DIAGRAM_VERTICAL_MARGIN * 2);
        Self::paint_pending_diagram_box(image, y, box_height, palette);
        Self::paint_pending_diagram_label(image, diagram, y, painter, palette);
    }

    fn paint_pending_diagram_box(
        image: &mut RgbaImage,
        y: u32,
        box_height: u32,
        palette: &SurfacePaintPalette,
    ) {
        SurfaceHelpers::fill_rect(
            image,
            PAGE_PADDING,
            y + DIAGRAM_VERTICAL_MARGIN,
            SURFACE_CONTENT_WIDTH,
            box_height,
            palette.code_background,
        );
        SurfaceHelpers::stroke_rect(
            image,
            PAGE_PADDING,
            y + DIAGRAM_VERTICAL_MARGIN,
            SURFACE_CONTENT_WIDTH,
            box_height,
            palette.code_border,
        );
    }

    fn paint_pending_diagram_label(
        image: &mut RgbaImage,
        diagram: &SurfaceDiagramBlock,
        y: u32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        painter.draw_text(
            image,
            diagram.fallback_text(),
            SurfaceTextLayout {
                x: PAGE_PADDING + CODE_HORIZONTAL_PADDING,
                y: y + DIAGRAM_VERTICAL_MARGIN + CODE_VERTICAL_PADDING,
                size: 22.0,
                color: palette.text,
                max_width: Some(SURFACE_CONTENT_WIDTH as f32),
            },
        );
    }
}
