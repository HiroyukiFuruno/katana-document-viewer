use super::{SurfaceLine, SurfaceLineAlignment, SurfaceLineLevel, SurfaceTypographyConfig};
use crate::export_surface_helpers::{LIST_INDENT, PAGE_PADDING, QUOTE_INDENT};

impl SurfaceLine {
    pub(crate) fn x(&self) -> u32 {
        PAGE_PADDING + self.quote_depth * QUOTE_INDENT + self.indent_depth * LIST_INDENT
    }

    pub(crate) fn font_size(&self) -> f32 {
        super::export_surface_line_metrics::font_size(&self.level, self.font_scale)
    }

    pub(crate) fn line_height(&self) -> u32 {
        super::export_surface_line_metrics::line_height(&self.level, self.font_scale)
    }

    pub(crate) fn text_y(&self, y: u32) -> u32 {
        y + self.top_margin()
    }

    pub(crate) fn quote_depth(&self) -> u32 {
        self.quote_depth
    }

    pub(crate) fn is_code(&self) -> bool {
        matches!(self.level, SurfaceLineLevel::Code)
    }

    pub(crate) fn is_heading(&self) -> bool {
        matches!(self.level, SurfaceLineLevel::Heading(_))
    }

    pub(crate) fn is_centered(&self) -> bool {
        matches!(self.alignment, SurfaceLineAlignment::Center)
    }

    pub(crate) fn is_right_aligned(&self) -> bool {
        matches!(self.alignment, SurfaceLineAlignment::Right)
    }

    #[cfg(test)]
    pub(crate) fn debug_style_tags(&self) -> Vec<String> {
        super::export_surface_line_debug::debug_style_tags(self)
    }

    pub(crate) fn indent_depth(&self) -> u32 {
        self.indent_depth
    }

    pub(crate) fn apply_typography(&mut self, typography: SurfaceTypographyConfig) {
        self.font_scale = match self.level {
            SurfaceLineLevel::Code => typography.code_scale(),
            SurfaceLineLevel::Heading(_) | SurfaceLineLevel::Body => typography.body_scale(),
        };
    }

    pub(super) fn scale_dimension(&self, value: u32) -> u32 {
        super::export_surface_line_metrics::scale_u32(value, self.font_scale)
    }

    pub(crate) fn font_scale(&self) -> f32 {
        self.font_scale
    }
}

#[cfg(test)]
#[path = "export_surface_line_impl_tests.rs"]
mod tests;
