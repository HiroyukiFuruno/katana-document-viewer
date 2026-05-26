use crate::export_surface_line::SurfaceLine;
use crate::export_surface_span::{SurfaceInlineSpans, SurfaceTextSpan, SurfaceTextStyle};
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{KmmNode, KmmNodeKind};

use super::super::SurfaceBlock;
use super::SurfaceBlockFactory;

impl SurfaceBlockFactory {
    pub(super) fn append_footnote_definition(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        quote_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        if let Some(line) = Self::footnote_line(node, quote_depth, theme) {
            blocks.push(SurfaceBlock::Line(line));
        }
    }

    pub(super) fn footnote_line(
        node: &KmmNode,
        quote_depth: u32,
        theme: &KdvThemeSnapshot,
    ) -> Option<SurfaceLine> {
        let KmmNodeKind::FootnoteDefinition(definition) = &node.kind else {
            return None;
        };
        let mut spans = vec![SurfaceTextSpan::plain(format!("{}. ", definition.label))];
        Self::append_footnote_body(&mut spans, node, &definition.text, theme);
        spans.push(SurfaceTextSpan::plain(" "));
        spans.push(SurfaceTextSpan::linked(
            "↩",
            format!("#fnref-{}", definition.label),
            SurfaceTextStyle::default().link(),
        ));
        Some(SurfaceLine::body_spans(spans, quote_depth))
    }

    fn append_footnote_body(
        spans: &mut Vec<SurfaceTextSpan>,
        node: &KmmNode,
        text: &str,
        theme: &KdvThemeSnapshot,
    ) {
        if node.children.is_empty() {
            spans.push(SurfaceTextSpan::plain(text.to_string()));
            return;
        }
        spans.extend(SurfaceInlineSpans::from_nodes(&node.children, theme));
    }
}

#[cfg(test)]
#[path = "footnote_tests.rs"]
mod tests;
