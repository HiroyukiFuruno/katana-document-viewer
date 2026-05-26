use super::{SurfaceLine, SurfaceLineAlignment, SurfaceLineLevel};
use crate::export_surface_helpers::{LIST_INDENT, PAGE_PADDING, QUOTE_INDENT};

const HEADING_1_FONT_SIZE: f32 = 40.0;
const HEADING_2_FONT_SIZE: f32 = 34.0;
const HEADING_DEFAULT_FONT_SIZE: f32 = 28.0;
const BODY_FONT_SIZE: f32 = 24.0;
const CODE_FONT_SIZE: f32 = 22.0;

const HEADING_1_LINE_HEIGHT: u32 = 92;
const HEADING_2_LINE_HEIGHT: u32 = 78;
const HEADING_DEFAULT_LINE_HEIGHT: u32 = 66;
const BODY_LINE_HEIGHT: u32 = 46;
const CODE_LINE_HEIGHT: u32 = 34;

impl SurfaceLine {
    #[cfg(test)]
    pub(crate) fn heading(level: u8, text: String) -> Self {
        Self {
            spans: vec![crate::export_surface_span::SurfaceTextSpan::plain(
                text.clone(),
            )],
            text,
            level: SurfaceLineLevel::Heading(level),
            quote_depth: 0,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Left,
        }
    }

    pub(crate) fn heading_spans(
        level: u8,
        spans: Vec<crate::export_surface_span::SurfaceTextSpan>,
    ) -> Self {
        Self {
            text: spans.iter().map(|span| span.text.as_str()).collect(),
            spans,
            level: SurfaceLineLevel::Heading(level),
            quote_depth: 0,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Left,
        }
    }

    #[cfg(test)]
    pub(crate) fn body(text: String) -> Self {
        Self::body_with_quote(text, 0)
    }

    pub(crate) fn body_with_quote(text: String, quote_depth: u32) -> Self {
        Self {
            spans: vec![crate::export_surface_span::SurfaceTextSpan::plain(
                text.clone(),
            )],
            text,
            level: SurfaceLineLevel::Body,
            quote_depth,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Left,
        }
    }

    pub(crate) fn body_centered(text: String) -> Self {
        Self {
            spans: vec![crate::export_surface_span::SurfaceTextSpan::plain(
                text.clone(),
            )],
            text,
            level: SurfaceLineLevel::Body,
            quote_depth: 0,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Center,
        }
    }

    pub(crate) fn centered_spans(spans: Vec<crate::export_surface_span::SurfaceTextSpan>) -> Self {
        Self {
            text: spans.iter().map(|span| span.text.as_str()).collect(),
            spans,
            level: SurfaceLineLevel::Body,
            quote_depth: 0,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Center,
        }
    }

    pub(crate) fn body_spans(
        spans: Vec<crate::export_surface_span::SurfaceTextSpan>,
        quote_depth: u32,
    ) -> Self {
        Self {
            text: spans.iter().map(|span| span.text.as_str()).collect(),
            spans,
            level: SurfaceLineLevel::Body,
            quote_depth,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Left,
        }
    }

    pub(crate) fn body_spans_with_indent(
        spans: Vec<crate::export_surface_span::SurfaceTextSpan>,
        quote_depth: u32,
        indent_depth: u32,
    ) -> Self {
        Self {
            text: spans.iter().map(|span| span.text.as_str()).collect(),
            spans,
            level: SurfaceLineLevel::Body,
            quote_depth,
            indent_depth,
            alignment: SurfaceLineAlignment::Left,
        }
    }

    pub(crate) fn code_spans(spans: Vec<crate::export_surface_span::SurfaceTextSpan>) -> Self {
        Self {
            text: spans.iter().map(|span| span.text.as_str()).collect(),
            spans,
            level: SurfaceLineLevel::Code,
            quote_depth: 0,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Left,
        }
    }

    pub(crate) fn x(&self) -> u32 {
        PAGE_PADDING + self.quote_depth * QUOTE_INDENT + self.indent_depth * LIST_INDENT
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self.level {
            SurfaceLineLevel::Heading(1) => HEADING_1_FONT_SIZE,
            SurfaceLineLevel::Heading(2) => HEADING_2_FONT_SIZE,
            SurfaceLineLevel::Heading(_) => HEADING_DEFAULT_FONT_SIZE,
            SurfaceLineLevel::Body => BODY_FONT_SIZE,
            SurfaceLineLevel::Code => CODE_FONT_SIZE,
        }
    }

    pub(crate) fn line_height(&self) -> u32 {
        match self.level {
            SurfaceLineLevel::Heading(1) => HEADING_1_LINE_HEIGHT,
            SurfaceLineLevel::Heading(2) => HEADING_2_LINE_HEIGHT,
            SurfaceLineLevel::Heading(_) => HEADING_DEFAULT_LINE_HEIGHT,
            SurfaceLineLevel::Body => BODY_LINE_HEIGHT,
            SurfaceLineLevel::Code => CODE_LINE_HEIGHT,
        }
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

    #[cfg(test)]
    pub(crate) fn debug_style_tags(&self) -> Vec<String> {
        super::export_surface_line_debug::debug_style_tags(self)
    }

    pub(crate) fn indent_depth(&self) -> u32 {
        self.indent_depth
    }
}

#[cfg(test)]
#[path = "export_surface_line_impl_tests.rs"]
mod tests;
