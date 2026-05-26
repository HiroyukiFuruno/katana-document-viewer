use image::{Rgba, RgbaImage};

use super::*;

const SIZE: u32 = 16;
const PIXEL: Rgba<u8> = Rgba([5, 5, 5, 255]);
const ALPHA_INDEX: usize = 3;

fn painted_pixels(image: &RgbaImage) -> u32 {
    image
        .pixels()
        .filter(|pixel| pixel.0[ALPHA_INDEX] > 0)
        .count() as u32
}

#[test]
fn draws_straight_and_diagonal_lines() {
    let mut image = RgbaImage::from_pixel(SIZE, SIZE, Rgba([0, 0, 0, 0]));

    draw_stroked_line(&mut image, 1, 1, 10, 1, PIXEL);
    draw_stroked_line(&mut image, 5, 5, 2, 2, PIXEL);

    assert!(painted_pixels(&image) > 0);
    let center = image.get_pixel(5, 5);
    assert_eq!(center.0, PIXEL.0);
}

#[test]
fn draws_zero_length_line() {
    let mut image = RgbaImage::from_pixel(SIZE, SIZE, Rgba([0, 0, 0, 0]));

    draw_stroked_line(&mut image, 3, 3, 3, 3, PIXEL);

    assert_eq!(painted_pixels(&image), 9);
}

#[test]
fn draws_circles_and_arc() {
    let mut image = RgbaImage::from_pixel(SIZE, SIZE, Rgba([0, 0, 0, 0]));

    draw_filled_circle(&mut image, 8, 8, 3, PIXEL);
    let filled_count = painted_pixels(&image);
    draw_stroked_circle(&mut image, 8, 8, 5, PIXEL);
    let stroked_count = painted_pixels(&image);
    draw_stroked_circle_arc(&mut image, 8, 8, 3, PIXEL);

    assert!(filled_count > 0);
    assert!(stroked_count >= filled_count);
    assert!(painted_pixels(&image) > stroked_count);
}

#[test]
fn draws_zero_radius_stroked_circle() {
    let mut image = RgbaImage::from_pixel(SIZE, SIZE, Rgba([0, 0, 0, 0]));

    draw_stroked_circle(&mut image, 8, 8, 0, PIXEL);

    assert_eq!(painted_pixels(&image), 1);
    assert_eq!(image.get_pixel(8, 8).0, PIXEL.0);
}

#[test]
fn draws_stroked_line_from_edge() {
    let mut image = RgbaImage::from_pixel(SIZE, SIZE, Rgba([0, 0, 0, 0]));

    draw_stroked_line(&mut image, 0, 4, 0, 8, PIXEL);

    assert!(painted_pixels(&image) > 0);
    assert!(image.get_pixel(0, 4).0[ALPHA_INDEX] > 0);
}
