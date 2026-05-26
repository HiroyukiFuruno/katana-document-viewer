use crate::export_surface_text::SurfaceTextParser;
use katana_markdown_model::TableNode;

use super::super::{SurfaceBlock, SurfaceTableBlock, SurfaceTableLayout};
use super::SurfaceBlockFactory;

impl SurfaceBlockFactory {
    pub(super) fn append_table(
        blocks: &mut Vec<SurfaceBlock>,
        table: &TableNode,
        fallback_text: &str,
        quote_depth: u32,
        list_depth: u32,
    ) {
        if !SurfaceTableLayout::has_contract(table) {
            Self::append_table_fallback(blocks, fallback_text, quote_depth, list_depth);
            return;
        }
        if quote_depth > 0 {
            Self::append_table_as_text(blocks, table, quote_depth, list_depth);
            return;
        }
        blocks.push(SurfaceBlock::Table(SurfaceTableBlock::new(table)));
    }

    fn append_table_fallback(
        blocks: &mut Vec<SurfaceBlock>,
        fallback_text: &str,
        quote_depth: u32,
        list_depth: u32,
    ) {
        Self::append_wrapped(
            blocks,
            SurfaceTextParser::decode_basic_entities(fallback_text),
            quote_depth,
            list_depth,
        );
    }

    fn append_table_as_text(
        blocks: &mut Vec<SurfaceBlock>,
        table: &TableNode,
        quote_depth: u32,
        list_depth: u32,
    ) {
        for line in SurfaceTableBlock::new(table).text().lines() {
            Self::append_wrapped(blocks, line.to_string(), quote_depth, list_depth);
        }
    }
}

#[cfg(test)]
#[path = "table_tests.rs"]
mod tests;
