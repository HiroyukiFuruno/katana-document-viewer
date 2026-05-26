use image::{Rgba, RgbaImage};

use super::*;

const CANVAS_SIZE: u32 = 20;
const DARK_RED: Rgba<u8> = Rgba([255, 40, 64, 255]);
const ALPHA_INDEX: usize = 3;

fn has_painted_pixels(image: &RgbaImage) -> bool {
    image.pixels().any(|pixel| pixel.0[ALPHA_INDEX] > 0)
}

#[test]
fn wrapper_functions_delegate_to_shape_drawers() {
    let mut image = RgbaImage::from_pixel(CANVAS_SIZE, CANVAS_SIZE, Rgba([0, 0, 0, 0]));

    draw_note_icon(&mut image, 0, 0, DARK_RED);
    assert!(has_painted_pixels(&image));

    draw_tip_icon(&mut image, 0, 0, DARK_RED);
    assert!(has_painted_pixels(&image));

    draw_important_icon(&mut image, 0, 0, DARK_RED);
    assert!(has_painted_pixels(&image));

    draw_warning_icon(&mut image, 0, 0, DARK_RED);
    assert!(has_painted_pixels(&image));

    draw_caution_icon(&mut image, 0, 0, DARK_RED);
    assert!(has_painted_pixels(&image));
}
