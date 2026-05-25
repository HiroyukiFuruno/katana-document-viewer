use crate::export_surface_helpers::{
    BODY_MAX_CHARS, LIST_INDENT, QUOTE_INDENT, SURFACE_CONTENT_WIDTH, WrappedText,
};
use crate::export_surface_line::{BODY_FONT_SIZE, SurfaceLine};
use crate::export_surface_span::{SurfaceInlineSpans, SurfaceTextSpan};
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::KmmNode;

use super::super::SurfaceBlock;
use super::SurfaceBlockFactory;

const MIN_LINE_WRAP_WIDTH: u32 = 120;

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

    fn line_width(quote_depth: u32, list_depth: u32) -> u32 {
        SURFACE_CONTENT_WIDTH
            .saturating_sub(quote_depth * QUOTE_INDENT)
            .saturating_sub(list_depth * LIST_INDENT)
    }

    fn append_rich_line_spans(
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

pub(super) struct SurfaceInlineLineWrapper;

impl SurfaceInlineLineWrapper {
    pub(super) fn wrap(spans: Vec<SurfaceTextSpan>, max_width: u32) -> Vec<Vec<SurfaceTextSpan>> {
        let max_width = max_width.max(MIN_LINE_WRAP_WIDTH);
        let mut state = LineWrapState::new(max_width);
        for segment in spans.into_iter().flat_map(Self::segments) {
            state.push(segment);
        }
        state.finish()
    }

    fn segments(span: SurfaceTextSpan) -> Vec<SurfaceTextSpan> {
        if span.inline_image.is_some() {
            return vec![span];
        }
        Self::text_segments(span)
    }

    fn text_segments(span: SurfaceTextSpan) -> Vec<SurfaceTextSpan> {
        let mut segments = Vec::new();
        let mut current = String::new();
        for character in span.text.chars() {
            current.push(character);
            if character.is_whitespace() {
                segments.push(Self::with_text(&span, std::mem::take(&mut current)));
            }
        }
        if !current.is_empty() {
            segments.push(Self::with_text(&span, current));
        }
        segments
    }

    fn with_text(span: &SurfaceTextSpan, text: String) -> SurfaceTextSpan {
        let mut segment = span.clone();
        segment.text = text;
        segment
    }
}

struct LineWrapState {
    max_width: u32,
    lines: Vec<Vec<SurfaceTextSpan>>,
    current_line: Vec<SurfaceTextSpan>,
    current_width: u32,
}

impl LineWrapState {
    fn new(max_width: u32) -> Self {
        Self {
            max_width,
            lines: Vec::new(),
            current_line: Vec::new(),
            current_width: 0,
        }
    }

    fn push(&mut self, segment: SurfaceTextSpan) {
        let segment_width = segment.estimated_width(BODY_FONT_SIZE);
        if self.should_start_new_line(segment_width) {
            self.start_new_line();
        }
        if self.should_skip_leading_space(&segment) {
            return;
        }
        self.current_width += segment_width;
        self.current_line.push(segment);
    }

    fn finish(mut self) -> Vec<Vec<SurfaceTextSpan>> {
        self.start_new_line();
        if self.lines.is_empty() {
            vec![vec![SurfaceTextSpan::plain(String::new())]]
        } else {
            self.lines
        }
    }

    fn should_start_new_line(&self, segment_width: u32) -> bool {
        self.current_width > 0 && self.current_width + segment_width > self.max_width
    }

    fn should_skip_leading_space(&self, segment: &SurfaceTextSpan) -> bool {
        self.current_width == 0 && segment.text.trim().is_empty()
    }

    fn start_new_line(&mut self) {
        if self.current_line.is_empty() {
            return;
        }
        self.lines.push(std::mem::take(&mut self.current_line));
        self.current_width = 0;
    }
}
