use image::{Rgba, RgbaImage};

use super::*;

const CANVAS_SIZE: u32 = 24;
const DARK_BLUE: Rgba<u8> = Rgba([6, 24, 82, 255]);
const ALPHA_INDEX: usize = 3;

fn counted_pixels(image: &RgbaImage) -> u32 {
    image
        .pixels()
        .filter(|pixel| pixel.0[ALPHA_INDEX] > 0)
        .count() as u32
}

#[test]
fn draws_note_icon_pixels() {
    let mut image = RgbaImage::from_pixel(CANVAS_SIZE, CANVAS_SIZE, Rgba([0, 0, 0, 0]));

    draw_note_icon(&mut image, 2, 3, DARK_BLUE);

    assert!(counted_pixels(&image) > 0);
}

#[test]
fn draws_tip_icon_pixels() {
    let mut image = RgbaImage::from_pixel(CANVAS_SIZE, CANVAS_SIZE, Rgba([0, 0, 0, 0]));

    draw_tip_icon(&mut image, 1, 1, DARK_BLUE);

    assert!(counted_pixels(&image) > 0);
}

#[test]
fn draws_important_icon_pixels() {
    let mut image = RgbaImage::from_pixel(CANVAS_SIZE, CANVAS_SIZE, Rgba([0, 0, 0, 0]));

    draw_important_icon(&mut image, 2, 2, DARK_BLUE);

    assert!(counted_pixels(&image) > 0);
}
