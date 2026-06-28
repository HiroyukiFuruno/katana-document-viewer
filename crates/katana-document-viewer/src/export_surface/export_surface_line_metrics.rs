use super::{BODY_FONT_SIZE, CODE_FONT_SIZE, SurfaceLineLevel};
use crate::viewer::ViewerCodeBlockMetrics;

const HEADING_1_FONT_SIZE: f32 = 40.0;
const HEADING_2_FONT_SIZE: f32 = 34.0;
const HEADING_DEFAULT_FONT_SIZE: f32 = 28.0;
const HEADING_1_LINE_HEIGHT: u32 = 92;
const HEADING_2_LINE_HEIGHT: u32 = 78;
const HEADING_DEFAULT_LINE_HEIGHT: u32 = 66;
const BODY_LINE_HEIGHT: u32 = 46;
const COMPACT_BODY_FONT_SIZE: f32 = 14.0;
const COMPACT_HEADING_1_LINE_HEIGHT: u32 = 40;
const COMPACT_HEADING_2_LINE_HEIGHT: u32 = 34;
const COMPACT_HEADING_DEFAULT_LINE_HEIGHT: u32 = 30;
const COMPACT_BODY_LINE_HEIGHT: u32 = 23;

pub(super) fn font_size(level: &SurfaceLineLevel, font_scale: f32) -> f32 {
    match level {
        SurfaceLineLevel::Heading(1) => HEADING_1_FONT_SIZE,
        SurfaceLineLevel::Heading(2) => HEADING_2_FONT_SIZE,
        SurfaceLineLevel::Heading(_) => HEADING_DEFAULT_FONT_SIZE,
        SurfaceLineLevel::Body => BODY_FONT_SIZE,
        SurfaceLineLevel::Code => CODE_FONT_SIZE,
    }
    .mul_add(font_scale, 0.0)
}

pub(super) fn line_height(level: &SurfaceLineLevel, font_scale: f32) -> u32 {
    match level {
        SurfaceLineLevel::Heading(1) => scaled_text_line_height(
            HEADING_1_LINE_HEIGHT,
            COMPACT_HEADING_1_LINE_HEIGHT,
            font_scale,
        ),
        SurfaceLineLevel::Heading(2) => scaled_text_line_height(
            HEADING_2_LINE_HEIGHT,
            COMPACT_HEADING_2_LINE_HEIGHT,
            font_scale,
        ),
        SurfaceLineLevel::Heading(_) => scaled_text_line_height(
            HEADING_DEFAULT_LINE_HEIGHT,
            COMPACT_HEADING_DEFAULT_LINE_HEIGHT,
            font_scale,
        ),
        SurfaceLineLevel::Body => {
            scaled_text_line_height(BODY_LINE_HEIGHT, COMPACT_BODY_LINE_HEIGHT, font_scale)
        }
        SurfaceLineLevel::Code => ViewerCodeBlockMetrics::line_height_from_scale_px(font_scale),
    }
}

pub(super) fn scale_u32(value: u32, scale: f32) -> u32 {
    if value == 0 {
        return 0;
    }
    if !scale.is_finite() || scale <= 0.0 {
        return value;
    }
    ((value as f32) * scale).round().max(1.0) as u32
}

fn scaled_text_line_height(default_height: u32, compact_height: u32, font_scale: f32) -> u32 {
    if !font_scale.is_finite() || font_scale <= 0.0 {
        return default_height;
    }
    let font_size = BODY_FONT_SIZE * font_scale;
    if font_size <= COMPACT_BODY_FONT_SIZE {
        return compact_height;
    }
    if font_size >= BODY_FONT_SIZE {
        return scale_u32(default_height, font_scale);
    }
    let t = (font_size - COMPACT_BODY_FONT_SIZE) / (BODY_FONT_SIZE - COMPACT_BODY_FONT_SIZE);
    (compact_height as f32 + (default_height - compact_height) as f32 * t)
        .round()
        .max(1.0) as u32
}
