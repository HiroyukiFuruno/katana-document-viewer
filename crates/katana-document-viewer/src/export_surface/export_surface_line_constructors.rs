use super::{SurfaceLine, SurfaceLineAlignment, SurfaceLineLevel};
use crate::export_surface_span::SurfaceTextSpan;

impl SurfaceLine {
    #[cfg(test)]
    pub(crate) fn heading(level: u8, text: String) -> Self {
        Self {
            spans: vec![SurfaceTextSpan::plain(text.clone())],
            text,
            level: SurfaceLineLevel::Heading(level),
            quote_depth: 0,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Left,
            font_scale: 1.0,
        }
    }

    pub(crate) fn heading_spans(level: u8, spans: Vec<SurfaceTextSpan>) -> Self {
        Self::heading_spans_with_alignment(level, spans, SurfaceLineAlignment::Left)
    }

    fn heading_spans_with_alignment(
        level: u8,
        spans: Vec<SurfaceTextSpan>,
        alignment: SurfaceLineAlignment,
    ) -> Self {
        Self {
            text: spans.iter().map(|span| span.text.as_str()).collect(),
            spans,
            level: SurfaceLineLevel::Heading(level),
            quote_depth: 0,
            indent_depth: 0,
            alignment,
            font_scale: 1.0,
        }
    }

    #[cfg(test)]
    pub(crate) fn body(text: String) -> Self {
        Self::body_with_quote(text, 0)
    }

    pub(crate) fn body_with_quote(text: String, quote_depth: u32) -> Self {
        Self::body_spans_with_indent(vec![SurfaceTextSpan::plain(text)], quote_depth, 0)
    }

    pub(crate) fn body_centered(text: String) -> Self {
        Self::centered_spans(vec![SurfaceTextSpan::plain(text)])
    }

    pub(crate) fn centered_spans(spans: Vec<SurfaceTextSpan>) -> Self {
        Self::body_spans_with_alignment(spans, SurfaceLineAlignment::Center)
    }

    pub(crate) fn right_spans(spans: Vec<SurfaceTextSpan>) -> Self {
        Self::body_spans_with_alignment(spans, SurfaceLineAlignment::Right)
    }

    pub(crate) fn body_spans(spans: Vec<SurfaceTextSpan>, quote_depth: u32) -> Self {
        Self::body_spans_with_indent(spans, quote_depth, 0)
    }

    pub(crate) fn body_spans_with_indent(
        spans: Vec<SurfaceTextSpan>,
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
            font_scale: 1.0,
        }
    }

    pub(crate) fn code_spans(spans: Vec<SurfaceTextSpan>) -> Self {
        Self {
            text: spans.iter().map(|span| span.text.as_str()).collect(),
            spans,
            level: SurfaceLineLevel::Code,
            quote_depth: 0,
            indent_depth: 0,
            alignment: SurfaceLineAlignment::Left,
            font_scale: 1.0,
        }
    }

    fn body_spans_with_alignment(
        spans: Vec<SurfaceTextSpan>,
        alignment: SurfaceLineAlignment,
    ) -> Self {
        Self {
            text: spans.iter().map(|span| span.text.as_str()).collect(),
            spans,
            level: SurfaceLineLevel::Body,
            quote_depth: 0,
            indent_depth: 0,
            alignment,
            font_scale: 1.0,
        }
    }
}
