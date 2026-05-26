use super::*;
use image::{Rgba, RgbaImage};

const CANVAS_SIZE: u32 = 30;
const WHITE_PIXEL: Rgba<u8> = Rgba([255, 255, 255, 255]);
const BLACK_PIXEL: Rgba<u8> = Rgba([0, 0, 0, 255]);
const CHECK_X: u32 = 2;
const CHECK_Y: u32 = 2;
const DIAGONAL_X: u32 = 6;
const DIAGONAL_Y: u32 = 6;
const CIRCLE_X: u32 = 10;
const CIRCLE_Y: u32 = 10;
const FILLED_RADIUS: u32 = 4;
const STROKED_RADIUS: u32 = 5;
const NOTE_X: u32 = 16;
const NOTE_Y: u32 = 16;
const TIP_X: u32 = 2;
const TIP_Y: u32 = 16;
const IMPORTANT_X: u32 = 16;
const IMPORTANT_Y: u32 = 2;
const WARNING_X: u32 = 20;
const WARNING_Y: u32 = 6;
const CAUTION_X: u32 = 8;
const CAUTION_Y: u32 = 20;

fn image_has_non_default_pixel(image: &RgbaImage, default: Rgba<u8>) -> bool {
    image.pixels().any(|pixel| pixel.0 != default.0)
}

fn white_canvas() -> RgbaImage {
    RgbaImage::from_pixel(CANVAS_SIZE, CANVAS_SIZE, WHITE_PIXEL)
}

fn assert_modified(image: &RgbaImage) {
    assert!(image_has_non_default_pixel(image, WHITE_PIXEL));
}

#[test]
fn helper_wrappers_draw_task_marks() {
    let mut image = white_canvas();
    let color = BLACK_PIXEL;

    draw_check_mark(&mut image, CHECK_X, CHECK_Y, color);
    assert_modified(&image);

    draw_diagonal_mark(&mut image, DIAGONAL_X, DIAGONAL_Y, color);
    assert_modified(&image);
}

#[test]
fn helper_wrappers_draw_circle_marks() {
    let mut image = white_canvas();
    let color = BLACK_PIXEL;
    draw_filled_circle(&mut image, CIRCLE_X, CIRCLE_Y, FILLED_RADIUS, color);
    draw_stroked_circle(&mut image, CIRCLE_X, CIRCLE_Y, STROKED_RADIUS, color);
    assert_modified(&image);
}

#[test]
fn helper_wrappers_draw_alert_icons() {
    let mut image = white_canvas();
    let color = BLACK_PIXEL;
    draw_note_icon(&mut image, NOTE_X, NOTE_Y, color);
    draw_tip_icon(&mut image, TIP_X, TIP_Y, color);
    draw_important_icon(&mut image, IMPORTANT_X, IMPORTANT_Y, color);
    draw_warning_icon(&mut image, WARNING_X, WARNING_Y, color);
    draw_caution_icon(&mut image, CAUTION_X, CAUTION_Y, color);
    assert_modified(&image);
}
