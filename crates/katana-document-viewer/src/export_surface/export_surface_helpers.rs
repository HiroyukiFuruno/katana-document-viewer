mod export_surface_helpers_canvas;
mod export_surface_helpers_wrapped_text;

pub(crate) use self::export_surface_helpers_wrapped_text::WrappedText;

pub(crate) struct SurfaceHelpers;

pub(crate) const SURFACE_WIDTH: u32 = 1280;
pub(crate) const SURFACE_MIN_HEIGHT: u32 = 720;
pub(crate) const SURFACE_PAGE_HEIGHT: u32 = 1810;
pub(crate) const PAGE_PADDING: u32 = 56;
pub(crate) const BODY_MAX_CHARS: usize = 58;
pub(crate) const QUOTE_INDENT: u32 = 32;
pub(crate) const LIST_INDENT: u32 = 42;
pub(crate) const SURFACE_CONTENT_WIDTH: u32 = SURFACE_WIDTH - PAGE_PADDING * 2;
