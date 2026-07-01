mod export_surface_line_constructors;
mod export_surface_line_debug;
mod export_surface_line_impl;
mod export_surface_line_markers;
mod export_surface_line_metrics;
use crate::export_surface_span::SurfaceTextSpan;

pub(crate) const BODY_FONT_SIZE: f32 = 24.0;
pub(crate) const CODE_FONT_SIZE: f32 = 22.0;
pub(crate) const LIST_MARKER_COLUMN_WIDTH: u32 = 36;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SurfaceTypographyConfig {
    body_font_size: f32,
    code_font_size: f32,
}

impl SurfaceTypographyConfig {
    pub(crate) const fn new(body_font_size: f32, code_font_size: f32) -> Self {
        Self {
            body_font_size,
            code_font_size,
        }
    }

    pub(crate) fn from_body_font_size(body_font_size: f32) -> Self {
        let code_font_size = (body_font_size - 2.0).max(10.0);
        Self::new(body_font_size, code_font_size)
    }

    pub(crate) fn body_scale(self) -> f32 {
        scale_for(self.body_font_size, BODY_FONT_SIZE)
    }

    pub(crate) fn code_scale(self) -> f32 {
        scale_for(self.code_font_size, CODE_FONT_SIZE)
    }
}

impl Default for SurfaceTypographyConfig {
    fn default() -> Self {
        Self::new(BODY_FONT_SIZE, CODE_FONT_SIZE)
    }
}

pub(crate) struct SurfaceLine {
    pub(crate) text: String,
    pub(crate) spans: Vec<SurfaceTextSpan>,
    anchor_id: Option<String>,
    level: SurfaceLineLevel,
    quote_depth: u32,
    indent_depth: u32,
    alignment: SurfaceLineAlignment,
    font_scale: f32,
}

enum SurfaceLineLevel {
    Heading(u8),
    Body,
    Code,
}

#[derive(Clone, Copy)]
enum SurfaceLineAlignment {
    Left,
    Center,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SurfaceLineMarker {
    Bullet,
    Ordered(String),
    Task(SurfaceTaskMarker),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SurfaceTaskMarker {
    Done,
    Empty,
    Blocked,
    InProgress,
}

fn scale_for(value: f32, base: f32) -> f32 {
    if value.is_finite() && value > 0.0 {
        return value / base;
    }
    1.0
}
