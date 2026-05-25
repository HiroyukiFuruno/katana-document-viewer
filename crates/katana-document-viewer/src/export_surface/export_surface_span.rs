use std::sync::Arc;

use image::Rgba;

#[path = "export_surface_span_defs.rs"]
mod export_surface_span_defs;
#[path = "export_surface_span_nodes.rs"]
mod export_surface_span_nodes;
#[path = "export_surface_span_nodes_helpers.rs"]
mod export_surface_span_nodes_helpers;

pub(crate) struct SurfaceInlineSpans;

pub(crate) const INLINE_MATH_MAX_WIDTH: u32 = 240;

#[derive(Clone, Debug)]
pub(crate) struct SurfaceInlineImage {
    image: Arc<image::RgbaImage>,
}

#[derive(Clone, Debug)]
pub(crate) struct SurfaceTextSpan {
    pub(crate) text: String,
    pub(crate) style: SurfaceTextStyle,
    pub(crate) link_target: Option<String>,
    pub(crate) inline_image: Option<SurfaceInlineImage>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SurfaceTextStyle {
    pub(crate) bold: bool,
    pub(crate) italic: bool,
    pub(crate) monospace: bool,
    pub(crate) underline: bool,
    pub(crate) strikethrough: bool,
    pub(crate) highlight: bool,
    pub(crate) inline_code: bool,
    pub(crate) color: Option<Rgba<u8>>,
}
