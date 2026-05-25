use image::{Rgba, RgbaImage};

use super::SurfaceGlyphPixel;

const DEFAULT_ALPHA: u8 = 255;
const COLOR_CHANNEL_COUNT: usize = 3;

pub(super) fn draw_glyph_pixel(image: &mut RgbaImage, glyph: SurfaceGlyphPixel) {
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
