use super::{
    SurfaceHelpers, SurfacePaintPalette, SurfacePainter, SurfaceTaskMarker, TASK_MARKER_BOX_OFFSET,
    TASK_MARKER_INLINE_OFFSET, TASK_MARKER_PROGRESS_STROKE, TASK_MARKER_SIZE, draw_check_mark,
    draw_diagonal_mark,
};
use image::RgbaImage;

const TASK_MARKER_BLOCKED_HORIZONTAL_INSET: u32 = 4;

impl SurfacePainter {
    pub(super) fn paint_task_marker(
        image: &mut RgbaImage,
        marker: SurfaceTaskMarker,
        x: u32,
        y: u32,
        palette: &SurfacePaintPalette,
    ) {
        let box_x = x + TASK_MARKER_INLINE_OFFSET;
        let box_y = y + TASK_MARKER_BOX_OFFSET;
        Self::paint_task_marker_box(image, marker, box_x, box_y, palette);
        Self::paint_task_marker_icon(image, marker, box_x, box_y, palette);
    }

    pub(super) fn paint_task_marker_box(
        image: &mut RgbaImage,
        marker: SurfaceTaskMarker,
        box_x: u32,
        box_y: u32,
        palette: &SurfacePaintPalette,
    ) {
        let fill = match marker {
            SurfaceTaskMarker::Empty => palette.task_empty_background,
            SurfaceTaskMarker::Done
            | SurfaceTaskMarker::Blocked
            | SurfaceTaskMarker::InProgress => palette.task_active_background,
        };
        SurfaceHelpers::fill_rect(
            image,
            box_x,
            box_y,
            TASK_MARKER_SIZE,
            TASK_MARKER_SIZE,
            fill,
        );
        SurfaceHelpers::stroke_rect(
            image,
            box_x,
            box_y,
            TASK_MARKER_SIZE,
            TASK_MARKER_SIZE,
            palette.table_border,
        );
    }

    pub(super) fn paint_task_marker_icon(
        image: &mut RgbaImage,
        marker: SurfaceTaskMarker,
        box_x: u32,
        box_y: u32,
        palette: &SurfacePaintPalette,
    ) {
        match marker {
            SurfaceTaskMarker::Done => {
                draw_check_mark(image, box_x, box_y, palette.task_done_accent)
            }
            SurfaceTaskMarker::Blocked => SurfaceHelpers::fill_rect(
                image,
                box_x + TASK_MARKER_BLOCKED_HORIZONTAL_INSET,
                box_y + TASK_MARKER_SIZE / 2,
                TASK_MARKER_SIZE - TASK_MARKER_BLOCKED_HORIZONTAL_INSET * 2,
                TASK_MARKER_PROGRESS_STROKE,
                palette.task_in_progress_accent,
            ),
            SurfaceTaskMarker::InProgress => {
                draw_diagonal_mark(image, box_x, box_y, palette.task_in_progress_accent)
            }
            SurfaceTaskMarker::Empty => {}
        }
    }
}
