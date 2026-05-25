#[path = "icons_shapes_info.rs"]
mod icons_shapes_info;
#[path = "icons_shapes_risk.rs"]
mod icons_shapes_risk;

use image::RgbaImage;

pub(super) fn draw_note_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    icons_shapes_info::draw_note_icon(image, x, y, color);
}

pub(super) fn draw_tip_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    icons_shapes_info::draw_tip_icon(image, x, y, color);
}

pub(super) fn draw_important_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    icons_shapes_info::draw_important_icon(image, x, y, color);
}

pub(super) fn draw_warning_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    icons_shapes_risk::draw_warning_icon(image, x, y, color);
}

pub(super) fn draw_caution_icon(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    icons_shapes_risk::draw_caution_icon(image, x, y, color);
}
