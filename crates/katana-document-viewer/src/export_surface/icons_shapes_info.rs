use image::RgbaImage;

use super::super::icons_constants::{
    IMPORTANT_CENTER_X, IMPORTANT_DOT_Y, IMPORTANT_LEFT_X, IMPORTANT_LOW_X, IMPORTANT_LOW_Y,
    IMPORTANT_MIDDLE_BOTTOM_Y, IMPORTANT_MIDDLE_TOP_Y, IMPORTANT_RIGHT_X, IMPORTANT_RIGHT_Y,
    IMPORTANT_TOP_Y, NOTE_ICON_CENTER, NOTE_ICON_DOT_Y, NOTE_ICON_RADIUS,
    NOTE_ICON_STROKE_BOTTOM_Y, NOTE_ICON_STROKE_TOP_Y, STROKE_PIXEL_SIZE, TIP_ICON_BOTTOM_END_X,
    TIP_ICON_BOTTOM_START_X, TIP_ICON_BOTTOM_Y, TIP_ICON_LINE_END_X, TIP_ICON_LINE_START_X,
    TIP_ICON_LINE_Y, TIP_ICON_RADIUS, TIP_ICON_Y,
};
use super::super::icons_primitives::{
    draw_filled_circle, draw_stroked_circle, draw_stroked_circle_arc, draw_stroked_line,
};

pub(super) fn draw_note_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    draw_stroked_circle(
        image,
        x + NOTE_ICON_CENTER,
        y + NOTE_ICON_CENTER,
        NOTE_ICON_RADIUS,
        color,
    );
    draw_vertical_stroke(
        image,
        x + NOTE_ICON_CENTER,
        y + NOTE_ICON_STROKE_TOP_Y,
        y + NOTE_ICON_STROKE_BOTTOM_Y,
        color,
    );
    draw_filled_circle(
        image,
        x + NOTE_ICON_CENTER,
        y + NOTE_ICON_DOT_Y,
        STROKE_PIXEL_SIZE,
        color,
    );
}

pub(super) fn draw_tip_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    draw_stroked_circle_arc(
        image,
        x + NOTE_ICON_CENTER,
        y + TIP_ICON_Y,
        TIP_ICON_RADIUS,
        color,
    );
    draw_stroked_line(
        image,
        x + TIP_ICON_LINE_START_X,
        y + TIP_ICON_LINE_Y,
        x + TIP_ICON_LINE_END_X,
        y + TIP_ICON_LINE_Y,
        color,
    );
    draw_stroked_line(
        image,
        x + TIP_ICON_BOTTOM_START_X,
        y + TIP_ICON_BOTTOM_Y,
        x + TIP_ICON_BOTTOM_END_X,
        y + TIP_ICON_BOTTOM_Y,
        color,
    );
}

pub(super) fn draw_important_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    draw_important_outline(image, x, y, color);
    draw_vertical_stroke(
        image,
        x + IMPORTANT_CENTER_X,
        y + IMPORTANT_MIDDLE_TOP_Y,
        y + IMPORTANT_MIDDLE_BOTTOM_Y,
        color,
    );
    draw_filled_circle(
        image,
        x + IMPORTANT_CENTER_X,
        y + IMPORTANT_DOT_Y,
        STROKE_PIXEL_SIZE,
        color,
    );
}

fn draw_important_outline(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    draw_outline(
        image,
        x,
        y,
        color,
        &[
            (IMPORTANT_LEFT_X, IMPORTANT_TOP_Y),
            (IMPORTANT_RIGHT_X, IMPORTANT_TOP_Y),
            (IMPORTANT_RIGHT_X, IMPORTANT_RIGHT_Y),
            (IMPORTANT_LOW_X, IMPORTANT_LOW_Y),
            (IMPORTANT_LEFT_X, IMPORTANT_TOP_Y),
        ],
    );
}

fn draw_outline(
    image: &mut RgbaImage,
    x: u32,
    y: u32,
    color: image::Rgba<u8>,
    points: &[(u32, u32)],
) {
    for pair in points.windows(2) {
        draw_stroked_line(
            image,
            x + pair[0].0,
            y + pair[0].1,
            x + pair[1].0,
            y + pair[1].1,
            color,
        );
    }
}

fn draw_vertical_stroke(
    image: &mut RgbaImage,
    x: u32,
    start_y: u32,
    end_y: u32,
    color: image::Rgba<u8>,
) {
    draw_stroked_line(image, x, start_y, x, end_y, color);
}

#[cfg(test)]
#[path = "icons_shapes_info_tests.rs"]
mod tests;
