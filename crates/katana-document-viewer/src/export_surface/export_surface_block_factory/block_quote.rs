use crate::forge::BuildGraph;
use crate::theme::KdvThemeSnapshot;

use crate::export_surface_helpers::SurfaceHelpers;
use katana_markdown_model::KmmNode;

use super::super::SurfaceBlock;
use super::super::markup::{legacy_note_children, legacy_note_quote};
use super::SurfaceBlockFactory;
impl SurfaceBlockFactory {
    pub(super) fn append_block_quote(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        if Self::append_legacy_note_block_quote(blocks, node, quote_depth, list_depth) {
            return;
        }
        if Self::append_nested_block_quote(blocks, node, quote_depth, list_depth) {
            return;
        }
        Self::append_children(blocks, graph, node, quote_depth + 1, list_depth, theme);
    }

    fn append_legacy_note_block_quote(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        quote_depth: u32,
        list_depth: u32,
    ) -> bool {
        if let Some((title, body)) = legacy_note_children(&node.children) {
            Self::append_wrapped(
                blocks,
                format!("{title} {body}"),
                quote_depth + 1,
                list_depth,
            );
            return true;
        }
        if let Some((title, body)) = legacy_note_quote(&node.source.raw.text) {
            Self::append_wrapped(
                blocks,
                format!("{title} {body}"),
                quote_depth + 1,
                list_depth,
            );
            return true;
        }
        false
    }

    fn append_nested_block_quote(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        quote_depth: u32,
        list_depth: u32,
    ) -> bool {
        if !SurfaceHelpers::is_nested_blockquote(&node.source.raw.text) {
            return false;
        }
        for (text, depth) in
            SurfaceHelpers::nested_blockquote_lines(&node.source.raw.text, quote_depth)
        {
            Self::append_wrapped(blocks, text, depth, list_depth);
        }
        true
    }
}
