use crate::export_semantics::EvaluatedMarkdownFragment;
use crate::forge::BuildGraph;
use crate::theme::KdvThemeSnapshot;

use super::super::markup::{SurfaceDetailsParts, SurfaceHtmlMarkup};
use super::{SurfaceBlock, SurfaceBlockFactory};

impl SurfaceBlockFactory {
    pub(super) fn append_details(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        fragment: &str,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) -> bool {
        let Some(parts) = SurfaceDetailsParts::parse(fragment) else {
            return false;
        };
        Self::append_details_summary(blocks, &parts, quote_depth, list_depth);
        Self::append_details_body(blocks, graph, &parts, quote_depth, list_depth, theme);
        true
    }

    fn append_details_summary(
        blocks: &mut Vec<SurfaceBlock>,
        parts: &SurfaceDetailsParts,
        quote_depth: u32,
        list_depth: u32,
    ) {
        Self::append_wrapped(
            blocks,
            SurfaceHtmlMarkup::normalize_text(parts.summary),
            quote_depth,
            list_depth,
        );
    }

    fn append_details_body(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        parts: &SurfaceDetailsParts,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        let fragment = EvaluatedMarkdownFragment::evaluate("surface-details.md", parts.body.trim());
        if !fragment.has_nodes() {
            Self::append_wrapped(
                blocks,
                SurfaceHtmlMarkup::normalize_text(parts.body),
                quote_depth,
                list_depth,
            );
            return;
        }
        for node in fragment.nodes() {
            Self::append_node(blocks, graph, node, quote_depth, list_depth, theme);
        }
    }
}
