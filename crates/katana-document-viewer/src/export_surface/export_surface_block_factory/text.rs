use crate::export_surface_helpers::{
    BODY_MAX_CHARS, LIST_INDENT, QUOTE_INDENT, SURFACE_CONTENT_WIDTH, WrappedText,
};
use crate::export_surface_line::SurfaceLine;
use crate::export_surface_span::{SurfaceInlineSpans, SurfaceTextSpan};
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::KmmNode;

use super::super::SurfaceBlock;
use super::SurfaceBlockFactory;

#[path = "text/wrap.rs"]
mod wrap;
pub(crate) use wrap::SurfaceInlineLineWrapper;

impl SurfaceBlockFactory {
    pub(super) fn append_wrapped(
        blocks: &mut Vec<SurfaceBlock>,
        text: String,
        quote_depth: u32,
        list_depth: u32,
    ) {
        for chunk in WrappedText::new(&text, BODY_MAX_CHARS) {
            Self::append_wrapped_chunk(blocks, chunk, quote_depth, list_depth);
        }
    }

    fn append_wrapped_chunk(
        blocks: &mut Vec<SurfaceBlock>,
        chunk: String,
        quote_depth: u32,
        list_depth: u32,
    ) {
        if list_depth > 0 {
            Self::append_indented_text_line(blocks, chunk, quote_depth, list_depth);
            return;
        }
        blocks.push(SurfaceBlock::Line(SurfaceLine::body_with_quote(
            chunk,
            quote_depth,
        )));
    }

    fn append_indented_text_line(
        blocks: &mut Vec<SurfaceBlock>,
        text: String,
        quote_depth: u32,
        list_depth: u32,
    ) {
        blocks.push(SurfaceBlock::Line(SurfaceLine::body_spans_with_indent(
            vec![SurfaceTextSpan::plain(text)],
            quote_depth,
            list_depth,
        )));
    }

    pub(super) fn append_rich_line(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        let spans = SurfaceInlineSpans::from_node(node, theme);
        if spans.is_empty() {
            return;
        }
        for line_spans in
            SurfaceInlineLineWrapper::wrap(spans, Self::line_width(quote_depth, list_depth))
        {
            Self::append_rich_line_spans(blocks, line_spans, quote_depth, list_depth);
        }
    }

    pub(super) fn line_width(quote_depth: u32, list_depth: u32) -> u32 {
        SURFACE_CONTENT_WIDTH
            .saturating_sub(quote_depth * QUOTE_INDENT)
            .saturating_sub(list_depth * LIST_INDENT)
    }

    pub(super) fn append_rich_line_spans(
        blocks: &mut Vec<SurfaceBlock>,
        spans: Vec<SurfaceTextSpan>,
        quote_depth: u32,
        list_depth: u32,
    ) {
        if list_depth > 0 {
            blocks.push(SurfaceBlock::Line(SurfaceLine::body_spans_with_indent(
                spans,
                quote_depth,
                list_depth,
            )));
            return;
        }
        blocks.push(SurfaceBlock::Line(SurfaceLine::body_spans(
            spans,
            quote_depth,
        )));
    }
}

#[cfg(test)]
#[path = "text_tests.rs"]
mod tests;
