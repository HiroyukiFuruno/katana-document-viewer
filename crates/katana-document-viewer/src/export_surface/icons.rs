#[path = "icons_constants.rs"]
mod icons_constants;
#[path = "icons_marks.rs"]
mod icons_marks;
#[path = "icons_primitives.rs"]
mod icons_primitives;
#[path = "icons_shapes.rs"]
mod icons_shapes;

use image::RgbaImage;

pub(super) fn alert_title_icon_y(line_y: u32, line_height: u32) -> u32 {
    icons_marks::alert_title_icon_y(line_y, line_height)
}

pub(super) fn draw_check_mark(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    icons_marks::draw_check_mark(image, x, y, color);
}

pub(super) fn draw_diagonal_mark(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    icons_marks::draw_diagonal_mark(image, x, y, color);
}

pub(super) fn draw_filled_circle(
    image: &mut RgbaImage,
    center_x: u32,
    center_y: u32,
    radius: u32,
    color: image::Rgba<u8>,
) {
    icons_primitives::draw_filled_circle(image, center_x, center_y, radius, color);
}

pub(super) fn draw_stroked_circle(
    image: &mut RgbaImage,
    center_x: u32,
    center_y: u32,
    radius: u32,
    color: image::Rgba<u8>,
) {
    icons_primitives::draw_stroked_circle(image, center_x, center_y, radius, color);
}

pub(super) fn draw_note_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    icons_shapes::draw_note_icon(image, x, y, color);
}

pub(super) fn draw_tip_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    icons_shapes::draw_tip_icon(image, x, y, color);
}

pub(super) fn draw_important_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    icons_shapes::draw_important_icon(image, x, y, color);
}

pub(super) fn draw_warning_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    icons_shapes::draw_warning_icon(image, x, y, color);
}

pub(super) fn draw_caution_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    icons_shapes::draw_caution_icon(image, x, y, color);
}

#[cfg(test)]
#[path = "icons_tests.rs"]
mod tests;
