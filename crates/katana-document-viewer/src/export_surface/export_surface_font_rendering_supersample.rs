use cosmic_text::Color;
use image::{Rgba, RgbaImage};

pub(crate) const TEXT_SUPERSAMPLE_SCALE: f32 = 2.0;

const TEXT_SUPERSAMPLE_SAMPLES: u32 = 4;
const OPAQUE_ALPHA: u8 = 255;
const COLOR_CHANNEL_COUNT: usize = 3;
const RED_CHANNEL: usize = 0;
const GREEN_CHANNEL: usize = 1;
const BLUE_CHANNEL: usize = 2;
const ALPHA_CHANNEL: usize = 3;

pub(crate) struct SurfaceTextSupersamples {
    samples: Vec<SurfaceTextSample>,
}

impl SurfaceTextSupersamples {
    pub(crate) fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub(crate) fn push_glyph(
        &mut self,
        glyph_x: i32,
        glyph_y: i32,
        width: u32,
        height: u32,
        color: Color,
    ) {
        let rgba = color.as_rgba();
        if rgba[ALPHA_CHANNEL] == 0 {
            return;
        }
        for dy in 0..height {
            for dx in 0..width {
                self.samples.push(SurfaceTextSample {
                    x: logical_sample_position(glyph_x + dx as i32),
                    y: logical_sample_position(glyph_y + dy as i32),
                    rgba,
                });
            }
        }
    }

    pub(crate) fn draw(mut self, image: &mut RgbaImage, origin_x: u32, origin_y: u32) {
        self.samples
            .sort_unstable_by_key(|sample| (sample.y, sample.x));
        let mut index = 0;
        while index < self.samples.len() {
            let current = self.samples[index];
            let group_start = index;
            let mut alpha_sum = 0_u32;
            while index < self.samples.len()
                && self.samples[index].x == current.x
                && self.samples[index].y == current.y
            {
                alpha_sum += u32::from(self.samples[index].rgba[ALPHA_CHANNEL]);
                index += 1;
            }
            let alpha = ((alpha_sum as f32 / TEXT_SUPERSAMPLE_SAMPLES as f32).round() as u32)
                .min(u32::from(OPAQUE_ALPHA)) as u8;
            if alpha == 0 {
                continue;
            }
            blend_pixel(
                image,
                origin_x as i32 + current.x,
                origin_y as i32 + current.y,
                averaged_color(&self.samples[group_start..index], alpha),
            );
        }
    }
}

#[derive(Clone, Copy)]
struct SurfaceTextSample {
    x: i32,
    y: i32,
    rgba: [u8; 4],
}

fn logical_sample_position(value: i32) -> i32 {
    (value as f32 / TEXT_SUPERSAMPLE_SCALE).floor() as i32
}

fn averaged_color(samples: &[SurfaceTextSample], alpha: u8) -> Rgba<u8> {
    let mut red_sum = 0_u32;
    let mut green_sum = 0_u32;
    let mut blue_sum = 0_u32;
    let mut alpha_sum = 0_u32;
    for sample in samples {
        let sample_alpha = u32::from(sample.rgba[ALPHA_CHANNEL]);
        red_sum += u32::from(sample.rgba[RED_CHANNEL]) * sample_alpha;
        green_sum += u32::from(sample.rgba[GREEN_CHANNEL]) * sample_alpha;
        blue_sum += u32::from(sample.rgba[BLUE_CHANNEL]) * sample_alpha;
        alpha_sum += sample_alpha;
    }
    if alpha_sum == 0 {
        return Rgba([0, 0, 0, 0]);
    }
    Rgba([
        (red_sum / alpha_sum) as u8,
        (green_sum / alpha_sum) as u8,
        (blue_sum / alpha_sum) as u8,
        alpha,
    ])
}

fn blend_pixel(image: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>) {
    if x < 0 || y < 0 || x >= image.width() as i32 || y >= image.height() as i32 {
        return;
    }
    let alpha = u16::from(color[ALPHA_CHANNEL]);
    let inverse_alpha = u16::from(OPAQUE_ALPHA) - alpha;
    let pixel = image.get_pixel_mut(x as u32, y as u32);
    for channel in 0..COLOR_CHANNEL_COUNT {
        let source = u16::from(color[channel]);
        let target = u16::from(pixel[channel]);
        pixel[channel] =
            ((source * alpha + target * inverse_alpha) / u16::from(OPAQUE_ALPHA)) as u8;
    }
    pixel[ALPHA_CHANNEL] = OPAQUE_ALPHA;
}
