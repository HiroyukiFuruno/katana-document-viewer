use image::{Rgba, RgbaImage};

const DEFAULT_ALPHA: u8 = 255;
const COLOR_CHANNEL_COUNT: usize = 3;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blend_pixel_ignores_out_of_bounds() {
        let mut image = RgbaImage::from_pixel(1, 1, Rgba([0, 0, 0, 0]));
        blend_pixel(&mut image, -1, 0, Rgba([255, 255, 255, 255]));
        blend_pixel(&mut image, 1, 0, Rgba([255, 255, 255, 255]));
        blend_pixel(&mut image, 0, 1, Rgba([255, 255, 255, 255]));

        assert_eq!(image.get_pixel(0, 0), &Rgba([0, 0, 0, 0]));
    }

    #[test]
    fn blend_pixel_blends_alpha() {
        let mut image = RgbaImage::from_pixel(1, 1, Rgba([10, 20, 30, 40]));
        blend_pixel(&mut image, 0, 0, Rgba([30, 40, 50, 128]));

        assert_eq!(
            image.get_pixel(0, 0),
            &Rgba([
                (30.0 * (128.0 / 255.0) + 10.0 * (127.0 / 255.0)) as u8,
                (40.0 * (128.0 / 255.0) + 20.0 * (127.0 / 255.0)) as u8,
                (50.0 * (128.0 / 255.0) + 30.0 * (127.0 / 255.0)) as u8,
                255,
            ])
        );
    }
}
