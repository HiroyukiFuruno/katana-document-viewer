use crate::export_surface_helpers::SurfaceHelpers;
use image::RgbaImage;

use super::super::{ALERT_ICON_SIZE, TASK_MARKER_SIZE};
use super::icons_constants::STROKE_PIXEL_SIZE;
use super::icons_constants::{
    ALERT_TITLE_TEXT_TOP_MARGIN, CHECK_MARK_DOT_COUNT, CHECK_MARK_DOT_OFFSET1,
    CHECK_MARK_DOT_OFFSET2, CHECK_MARK_DOT_OFFSET3, CHECK_MARK_DOT_SIZE, CHECK_MARK_DOT_Y_BOTTOM,
    CHECK_MARK_DOT_Y_MIDDLE, CHECK_MARK_DOT_Y_TOP, CHECKMARK_DIAGONAL_PADDING,
    TASK_MARKER_DIAGONAL_X_OFFSET, TASK_MARKER_DIAGONAL_Y_OFFSET,
};

pub(super) fn draw_check_mark(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    for offset in 0..CHECK_MARK_DOT_COUNT {
        SurfaceHelpers::fill_rect(
            image,
            x + CHECK_MARK_DOT_OFFSET1 + offset,
            y + CHECK_MARK_DOT_Y_TOP + offset,
            CHECK_MARK_DOT_SIZE,
            CHECK_MARK_DOT_SIZE,
            color,
        );
        SurfaceHelpers::fill_rect(
            image,
            x + CHECK_MARK_DOT_OFFSET2 + offset,
            y + CHECK_MARK_DOT_Y_MIDDLE - offset,
            CHECK_MARK_DOT_SIZE,
            CHECK_MARK_DOT_SIZE,
            color,
        );
        SurfaceHelpers::fill_rect(
            image,
            x + CHECK_MARK_DOT_OFFSET3 + offset,
            y + CHECK_MARK_DOT_Y_BOTTOM - offset,
            CHECK_MARK_DOT_SIZE,
            CHECK_MARK_DOT_SIZE,
            color,
        );
    }
}

pub(super) fn draw_diagonal_mark(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    for offset in 0..TASK_MARKER_SIZE - CHECKMARK_DIAGONAL_PADDING {
        SurfaceHelpers::fill_rect(
            image,
            x + TASK_MARKER_DIAGONAL_X_OFFSET + offset,
            y + TASK_MARKER_SIZE - TASK_MARKER_DIAGONAL_Y_OFFSET - offset,
            STROKE_PIXEL_SIZE,
            STROKE_PIXEL_SIZE,
            color,
        );
    }
}

pub(super) fn alert_title_icon_y(line_y: u32, line_height: u32) -> u32 {
    let centered_y = line_height.saturating_sub(ALERT_ICON_SIZE) / 2;
    line_y + centered_y.saturating_sub(ALERT_TITLE_TEXT_TOP_MARGIN)
}
