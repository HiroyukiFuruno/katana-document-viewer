use super::{
    PAGE_PADDING, RULE_HEIGHT, RULE_STROKE_WIDTH, SURFACE_CONTENT_WIDTH, SurfaceHelpers,
    SurfacePaintPalette, SurfacePainter,
};
use image::RgbaImage;

impl SurfacePainter {
    pub(super) fn paint_rule(image: &mut RgbaImage, y: u32, palette: &SurfacePaintPalette) {
        let line_y = y + RULE_HEIGHT / 2;
        SurfaceHelpers::fill_rect(
            image,
            PAGE_PADDING,
            line_y,
            SURFACE_CONTENT_WIDTH,
            RULE_STROKE_WIDTH,
            palette.table_border,
        );
    }
}
