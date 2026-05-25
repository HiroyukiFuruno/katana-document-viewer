mod export_surface_line_debug;
mod export_surface_line_impl;
mod export_surface_line_markers;
use crate::export_surface_span::SurfaceTextSpan;

pub(crate) const BODY_FONT_SIZE: f32 = 24.0;
pub(crate) const LIST_MARKER_COLUMN_WIDTH: u32 = 36;

pub(crate) struct SurfaceLine {
    pub(crate) text: String,
    pub(crate) spans: Vec<SurfaceTextSpan>,
    level: SurfaceLineLevel,
    quote_depth: u32,
    indent_depth: u32,
    alignment: SurfaceLineAlignment,
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
