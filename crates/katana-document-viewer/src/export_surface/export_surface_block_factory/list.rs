use crate::export_surface_helpers::{LIST_INDENT, QUOTE_INDENT, SURFACE_CONTENT_WIDTH};
use crate::export_surface_line::{BODY_FONT_SIZE, LIST_MARKER_COLUMN_WIDTH, SurfaceLine};
use crate::export_surface_span::{SurfaceInlineSpans, SurfaceTextSpan};
use crate::forge::BuildGraph;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{ListItemNode, ListNode};

use super::super::SurfaceBlock;
use super::super::markup::list_marker_text;
use super::SurfaceBlockFactory;
use super::text::SurfaceInlineLineWrapper;

impl SurfaceBlockFactory {
    pub(super) fn append_list(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        list: &ListNode,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        for item in &list.items {
            Self::append_list_item(
                blocks,
                graph,
                item,
                list.ordered,
                quote_depth,
                list_depth,
                theme,
            );
        }
    }

    fn append_list_item(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        item: &ListItemNode,
        ordered: bool,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        let body_spans = SurfaceInlineSpans::from_nodes(&item.body, theme);
        if !body_spans.iter().any(|span| !span.text.trim().is_empty()) {
            return;
        }
        let marker = SurfaceTextSpan::plain(list_marker_text(item, ordered));
        Self::append_wrapped_list_item(blocks, marker, body_spans, quote_depth, list_depth);
        Self::append_list_item_children(blocks, graph, item, quote_depth, list_depth, theme);
    }

    fn append_list_item_children(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        item: &ListItemNode,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        for child in &item.children {
            Self::append_node(blocks, graph, child, quote_depth, list_depth + 1, theme);
        }
    }

    pub(super) fn append_wrapped_list_item(
        blocks: &mut Vec<SurfaceBlock>,
        marker: SurfaceTextSpan,
        body_spans: Vec<SurfaceTextSpan>,
        quote_depth: u32,
        list_depth: u32,
    ) {
        let available_width = Self::list_item_body_width(quote_depth, list_depth);
        let body_width = Self::span_width(&body_spans);
        if body_width <= available_width {
            Self::append_wrapped_list_item_as_single_line(
                blocks,
                marker,
                body_spans,
                quote_depth,
                list_depth,
            );
            return;
        }
        Self::append_wrapped_list_item_as_multiple_lines(
            blocks,
            marker,
            body_spans,
            available_width,
            quote_depth,
            list_depth,
        );
    }

    fn list_item_body_width(quote_depth: u32, list_depth: u32) -> u32 {
        SURFACE_CONTENT_WIDTH
            .saturating_sub(quote_depth * QUOTE_INDENT)
            .saturating_sub(list_depth * LIST_INDENT)
            .saturating_sub(LIST_MARKER_COLUMN_WIDTH)
    }

    fn span_width(spans: &[SurfaceTextSpan]) -> u32 {
        spans
            .iter()
            .map(|span| span.estimated_width(BODY_FONT_SIZE))
            .sum::<u32>()
    }

    fn append_wrapped_list_item_as_single_line(
        blocks: &mut Vec<SurfaceBlock>,
        marker: SurfaceTextSpan,
        body_spans: Vec<SurfaceTextSpan>,
        quote_depth: u32,
        list_depth: u32,
    ) {
        let spans = Self::list_item_first_line(marker, body_spans);
        blocks.push(SurfaceBlock::Line(SurfaceLine::body_spans_with_indent(
            spans,
            quote_depth,
            list_depth,
        )));
    }

    fn append_wrapped_list_item_as_multiple_lines(
        blocks: &mut Vec<SurfaceBlock>,
        marker: SurfaceTextSpan,
        body_spans: Vec<SurfaceTextSpan>,
        available_width: u32,
        quote_depth: u32,
        list_depth: u32,
    ) {
        let mut lines = SurfaceInlineLineWrapper::wrap(body_spans, available_width);
        for (index, line_spans) in lines.drain(..).enumerate() {
            let line = if index == 0 {
                Self::list_item_first_line(marker.clone(), line_spans)
            } else {
                line_spans
            };
            Self::append_list_line(blocks, line, quote_depth, list_depth);
        }
    }

    fn list_item_first_line(
        marker: SurfaceTextSpan,
        body_spans: Vec<SurfaceTextSpan>,
    ) -> Vec<SurfaceTextSpan> {
        let mut spans = Vec::with_capacity(body_spans.len().saturating_add(1));
        spans.push(marker);
        spans.extend(body_spans);
        spans
    }

    fn append_list_line(
        blocks: &mut Vec<SurfaceBlock>,
        spans: Vec<SurfaceTextSpan>,
        quote_depth: u32,
        list_depth: u32,
    ) {
        blocks.push(SurfaceBlock::Line(SurfaceLine::body_spans_with_indent(
            spans,
            quote_depth,
            list_depth,
        )));
    }
}

#[cfg(test)]
#[path = "list_tests.rs"]
mod tests;
