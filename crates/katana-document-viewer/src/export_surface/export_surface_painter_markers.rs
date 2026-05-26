use super::{
    CODE_LINE_BULLET_FILLED_RADIUS, CODE_LINE_BULLET_FILLED_Y_OFFSET, CODE_LINE_BULLET_RECT_SIZE,
    CODE_LINE_BULLET_RECT_X_OFFSET, CODE_LINE_BULLET_RECT_Y_OFFSET,
    CODE_LINE_BULLET_STROKED_RADIUS, CODE_LINE_BULLET_STROKED_Y_OFFSET, CODE_LINE_BULLET_X_OFFSET,
    LIST_MARKER_COLUMN_WIDTH, SurfaceHelpers, SurfaceLineMarker, SurfaceMarkerPaintRequest,
    SurfacePaintPalette, SurfacePainter, SurfaceTextLayout, SurfaceTextPainter, draw_filled_circle,
    draw_stroked_circle,
};
use image::RgbaImage;

const BULLET_MARKER_VARIANT_COUNT: u32 = 3;
impl SurfacePainter {
    pub(super) fn paint_line_marker(
        image: &mut RgbaImage,
        request: SurfaceMarkerPaintRequest<'_>,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        match request.marker {
            SurfaceLineMarker::Bullet => Self::paint_material_bullet(
                image,
                request.x,
                request.y,
                request.indent_depth,
                palette,
            ),
            SurfaceLineMarker::Ordered(value) => Self::paint_text_marker(
                image,
                value,
                request.x,
                request.y,
                request.size,
                painter,
                palette,
            ),
            SurfaceLineMarker::Task(marker) => {
                Self::paint_task_marker(image, *marker, request.x, request.y, palette)
            }
        }
    }

    pub(super) fn paint_material_bullet(
        image: &mut RgbaImage,
        x: u32,
        y: u32,
        indent_depth: u32,
        palette: &SurfacePaintPalette,
    ) {
        match indent_depth % BULLET_MARKER_VARIANT_COUNT {
            0 => Self::paint_filled_bullet(image, x, y, palette),
            1 => Self::paint_stroked_bullet(image, x, y, palette),
            _ => Self::paint_square_bullet(image, x, y, palette),
        }
    }

    pub(super) fn paint_filled_bullet(
        image: &mut RgbaImage,
        x: u32,
        y: u32,
        palette: &SurfacePaintPalette,
    ) {
        draw_filled_circle(
            image,
            x + CODE_LINE_BULLET_X_OFFSET,
            y + CODE_LINE_BULLET_FILLED_Y_OFFSET,
            CODE_LINE_BULLET_FILLED_RADIUS,
            palette.text,
        )
    }

    pub(super) fn paint_stroked_bullet(
        image: &mut RgbaImage,
        x: u32,
        y: u32,
        palette: &SurfacePaintPalette,
    ) {
        draw_stroked_circle(
            image,
            x + CODE_LINE_BULLET_X_OFFSET,
            y + CODE_LINE_BULLET_STROKED_Y_OFFSET,
            CODE_LINE_BULLET_STROKED_RADIUS,
            palette.text,
        )
    }

    pub(super) fn paint_square_bullet(
        image: &mut RgbaImage,
        x: u32,
        y: u32,
        palette: &SurfacePaintPalette,
    ) {
        SurfaceHelpers::fill_rect(
            image,
            x + CODE_LINE_BULLET_RECT_X_OFFSET,
            y + CODE_LINE_BULLET_RECT_Y_OFFSET,
            CODE_LINE_BULLET_RECT_SIZE,
            CODE_LINE_BULLET_RECT_SIZE,
            palette.text,
        )
    }

    pub(super) fn paint_text_marker(
        image: &mut RgbaImage,
        text: &str,
        x: u32,
        y: u32,
        size: f32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        match painter {
            Some(it) => it.draw_text(
                image,
                text,
                SurfaceTextLayout {
                    x,
                    y,
                    size,
                    color: palette.text,
                    max_width: Some(LIST_MARKER_COLUMN_WIDTH as f32),
                },
            ),
            None => SurfaceHelpers::draw_fallback_text(image, x, y, text, palette.text),
        }
    }
}
#[cfg(test)]
#[path = "export_surface_painter_markers_tests.rs"]
mod tests;
