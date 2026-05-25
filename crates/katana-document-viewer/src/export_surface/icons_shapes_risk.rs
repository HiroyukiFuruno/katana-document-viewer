use image::RgbaImage;

use super::super::icons_constants::{
    CAUTION_BOT_MID_X, CAUTION_BOTTOM_Y, CAUTION_CENTER_BOTTOM_Y, CAUTION_CENTER_TOP_Y,
    CAUTION_DOT_X, CAUTION_DOT_Y, CAUTION_LEFT_X, CAUTION_MID_Y, CAUTION_RIGHT_BOTTOM_Y,
    CAUTION_RIGHT_MID_Y, CAUTION_RIGHT_X, CAUTION_TOP_LEFT_X, CAUTION_TOP_MID_X, CAUTION_TOP_Y,
    NOTE_ICON_CENTER, STROKE_PIXEL_SIZE, WARNING_BOTTOM_Y, WARNING_DOT_Y, WARNING_LEFT_X,
    WARNING_MIDDLE_BOTTOM_Y, WARNING_MIDDLE_TOP_Y, WARNING_RIGHT_X, WARNING_TOP_Y,
};
use super::super::icons_primitives::{draw_filled_circle, draw_stroked_line};

pub(super) fn draw_warning_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    draw_warning_outline(image, x, y, color);
    draw_vertical_stroke(
        image,
        x + NOTE_ICON_CENTER,
        y + WARNING_MIDDLE_TOP_Y,
        y + WARNING_MIDDLE_BOTTOM_Y,
        color,
    );
    draw_filled_circle(
        image,
        x + NOTE_ICON_CENTER,
        y + WARNING_DOT_Y,
        STROKE_PIXEL_SIZE,
        color,
    );
}

pub(super) fn draw_caution_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    draw_caution_outline(image, x, y, color);
    draw_vertical_stroke(
        image,
        x + CAUTION_DOT_X,
        y + CAUTION_CENTER_TOP_Y,
        y + CAUTION_CENTER_BOTTOM_Y,
        color,
    );
    draw_filled_circle(
        image,
        x + CAUTION_DOT_X,
        y + CAUTION_DOT_Y,
        STROKE_PIXEL_SIZE,
        color,
    );
}

fn draw_warning_outline(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    draw_outline(
        image,
        x,
        y,
        color,
        &[
            (WARNING_LEFT_X, WARNING_BOTTOM_Y),
            (NOTE_ICON_CENTER, WARNING_TOP_Y),
            (WARNING_RIGHT_X, WARNING_BOTTOM_Y),
            (WARNING_LEFT_X, WARNING_BOTTOM_Y),
        ],
    );
}

fn draw_caution_outline(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    draw_outline(
        image,
        x,
        y,
        color,
        &[
            (CAUTION_TOP_LEFT_X, CAUTION_TOP_Y),
            (CAUTION_TOP_MID_X, CAUTION_TOP_Y),
            (CAUTION_RIGHT_X, CAUTION_RIGHT_MID_Y),
            (CAUTION_RIGHT_X, CAUTION_RIGHT_BOTTOM_Y),
            (CAUTION_BOT_MID_X, CAUTION_BOTTOM_Y),
            (CAUTION_LEFT_X, CAUTION_MID_Y),
            (CAUTION_TOP_LEFT_X, CAUTION_TOP_Y),
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
