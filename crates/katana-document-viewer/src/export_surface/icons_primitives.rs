use crate::export_surface_helpers::SurfaceHelpers;
use image::RgbaImage;

use super::icons_constants::{
    STROKE_ARC_COLOR, STROKE_ARC_PAD_HEIGHT, STROKE_ARC_PAD_WIDTH, STROKE_ARC_PADDING_XY,
    STROKE_OUTLINE_INNER_SUB, STROKE_PIXEL_SIZE,
};

use super::icons_constants::CAUTION_MID_Y;

pub(super) fn draw_stroked_line(
    image: &mut RgbaImage,
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
    color: image::Rgba<u8>,
) {
    let mut state = StrokedLineState::new(start_x, start_y, end_x, end_y);
    loop {
        draw_stroked_point(image, state.x, state.y, color);
        if state.is_finished() {
            break;
        }
        state.advance();
    }
}

struct StrokedLineState {
    x: i32,
    y: i32,
    end_x: i32,
    end_y: i32,
    dx: i32,
    sx: i32,
    dy: i32,
    sy: i32,
    error: i32,
}

impl StrokedLineState {
    fn new(start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> Self {
        let x = start_x as i32;
        let y = start_y as i32;
        let end_x = end_x as i32;
        let end_y = end_y as i32;
        let dx = (end_x - x).abs();
        let dy = -(end_y - y).abs();
        Self {
            x,
            y,
            end_x,
            end_y,
            dx,
            sx: if x < end_x { 1 } else { -1 },
            dy,
            sy: if y < end_y { 1 } else { -1 },
            error: dx + dy,
        }
    }

    fn is_finished(&self) -> bool {
        self.x == self.end_x && self.y == self.end_y
    }

    fn advance(&mut self) {
        let doubled_error = 2 * self.error;
        if doubled_error >= self.dy {
            self.error += self.dy;
            self.x += self.sx;
        }
        if doubled_error <= self.dx {
            self.error += self.dx;
            self.y += self.sy;
        }
    }
}

fn draw_stroked_point(image: &mut RgbaImage, x: i32, y: i32, color: image::Rgba<u8>) {
    for offset_y in -1..=1 {
        for offset_x in -1..=1 {
            if let (Ok(pixel_x), Ok(pixel_y)) =
                (u32::try_from(x + offset_x), u32::try_from(y + offset_y))
            {
                SurfaceHelpers::fill_rect(
                    image,
                    pixel_x,
                    pixel_y,
                    STROKE_PIXEL_SIZE,
                    STROKE_PIXEL_SIZE,
                    color,
                );
            }
        }
    }
}

pub(super) fn draw_filled_circle(
    image: &mut RgbaImage,
    center_x: u32,
    center_y: u32,
    radius: u32,
    color: image::Rgba<u8>,
) {
    let radius_squared = (radius * radius) as i32;
    for y in center_y.saturating_sub(radius)..=center_y + radius {
        for x in center_x.saturating_sub(radius)..=center_x + radius {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            if dx * dx + dy * dy <= radius_squared {
                SurfaceHelpers::fill_rect(image, x, y, STROKE_PIXEL_SIZE, STROKE_PIXEL_SIZE, color);
            }
        }
    }
}

pub(super) fn draw_stroked_circle(
    image: &mut RgbaImage,
    center_x: u32,
    center_y: u32,
    radius: u32,
    color: image::Rgba<u8>,
) {
    let inner = radius.saturating_sub(STROKE_OUTLINE_INNER_SUB);
    let outer_squared = (radius * radius) as i32;
    let inner_squared = (inner * inner) as i32;
    for y in center_y.saturating_sub(radius)..=center_y + radius {
        for x in center_x.saturating_sub(radius)..=center_x + radius {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            let distance = dx * dx + dy * dy;
            if distance <= outer_squared && distance >= inner_squared {
                SurfaceHelpers::fill_rect(image, x, y, STROKE_PIXEL_SIZE, STROKE_PIXEL_SIZE, color);
            }
        }
    }
}

pub(super) fn draw_stroked_circle_arc(
    image: &mut RgbaImage,
    center_x: u32,
    center_y: u32,
    radius: u32,
    color: image::Rgba<u8>,
) {
    draw_stroked_circle(image, center_x, center_y, radius, color);
    SurfaceHelpers::fill_rect(
        image,
        center_x.saturating_sub(radius + STROKE_ARC_PADDING_XY),
        center_y + CAUTION_MID_Y,
        radius * 2 + STROKE_ARC_PAD_WIDTH,
        radius + STROKE_ARC_PAD_HEIGHT,
        STROKE_ARC_COLOR,
    );
}
