use cosmic_text::Color;
use image::{Rgba, RgbaImage};

use super::{COLOR_CHANNEL_COUNT, DEFAULT_ALPHA, UNDERLINE_STROKE_THICKNESS};

pub(super) fn paste_inline_rgba(target: &mut RgbaImage, source: &RgbaImage, x: u32, y: u32) {
    for (source_x, source_y, pixel) in source.enumerate_pixels() {
        blend_pixel(
            target,
            x.saturating_add(source_x) as i32,
            y.saturating_add(source_y) as i32,
            *pixel,
        );
    }
}

pub(super) fn draw_horizontal_line(
    image: &mut RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    color: Rgba<u8>,
) {
    for dy in 0..UNDERLINE_STROKE_THICKNESS {
        for dx in 0..width {
            PixelWriter::put(image, x + dx, y + dy, color);
        }
    }
}

pub(super) fn fill_rect(
    image: &mut RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: Rgba<u8>,
) {
    for dy in 0..height {
        for dx in 0..width {
            PixelWriter::put(image, x + dx, y + dy, color);
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct SurfaceGlyphPixel {
    origin_x: u32,
    origin_y: u32,
    glyph_x: i32,
    glyph_y: i32,
    width: u32,
    height: u32,
    color: Color,
}

pub(super) fn fill_glyph_pixel(image: &mut RgbaImage, glyph: SurfaceGlyphPixel) {
    let [red, green, blue, alpha] = glyph.color.as_rgba();
    for dy in 0..glyph.height {
        for dx in 0..glyph.width {
            blend_pixel(
                image,
                glyph.origin_x as i32 + glyph.glyph_x + dx as i32,
                glyph.origin_y as i32 + glyph.glyph_y + dy as i32,
                Rgba([red, green, blue, alpha]),
            );
        }
    }
}

pub(super) fn blend_pixel(image: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>) {
    if x < 0 || y < 0 || x >= image.width() as i32 || y >= image.height() as i32 {
        return;
    }
    let alpha = f32::from(color[COLOR_CHANNEL_COUNT]) / f32::from(DEFAULT_ALPHA);
    let pixel = image.get_pixel_mut(x as u32, y as u32);
    for index in 0..COLOR_CHANNEL_COUNT {
        let source = f32::from(color[index]);
        let target = f32::from(pixel[index]);
        pixel[index] = (source * alpha + target * (1.0 - alpha)) as u8;
    }
    pixel[COLOR_CHANNEL_COUNT] = DEFAULT_ALPHA;
}

struct PixelWriter;

impl PixelWriter {
    fn put(image: &mut RgbaImage, target_x: u32, target_y: u32, color: Rgba<u8>) {
        if target_x < image.width() && target_y < image.height() {
            image.put_pixel(target_x, target_y, color);
        }
    }
}
