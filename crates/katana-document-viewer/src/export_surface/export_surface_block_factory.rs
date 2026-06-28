use crate::export_surface_helpers::SurfaceHelpers;
use crate::export_surface_line::{SurfaceLine, SurfaceTypographyConfig};
use crate::export_surface_span::SurfaceInlineSpans;
use crate::forge::BuildGraph;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{KmmNode, KmmNodeKind};

use super::SurfaceBlock;

#[path = "export_surface_block_factory/alert.rs"]
mod alert;
#[path = "export_surface_block_factory/block_quote.rs"]
mod block_quote;
#[path = "export_surface_block_factory/code.rs"]
mod code;
#[path = "export_surface_block_factory/dispatch.rs"]
mod dispatch;
#[path = "export_surface_block_factory/footnote.rs"]
mod footnote;
#[path = "export_surface_block_factory/html.rs"]
mod html;
#[path = "export_surface_block_factory/html_details_block.rs"]
mod html_details_block;
#[path = "export_surface_block_factory/image.rs"]
mod image;
#[path = "export_surface_block_factory/list.rs"]
mod list;
#[path = "export_surface_block_factory/raw.rs"]
mod raw;
#[path = "export_surface_block_factory/table.rs"]
mod table;
#[path = "export_surface_block_factory/text.rs"]
mod text;

pub(crate) struct SurfaceBlockFactory;

impl SurfaceBlockFactory {
    pub(crate) fn create(graph: &BuildGraph, theme: &KdvThemeSnapshot) -> Vec<SurfaceBlock> {
        Self::create_with_typography(graph, theme, SurfaceTypographyConfig::default())
    }

    pub(crate) fn create_with_typography(
        graph: &BuildGraph,
        theme: &KdvThemeSnapshot,
        typography: SurfaceTypographyConfig,
    ) -> Vec<SurfaceBlock> {
        let mut blocks = Vec::new();
        let mut footnotes = Vec::new();
        for node in &graph.snapshot.document.nodes {
            if let Some(line) = Self::footnote_line(node, 0, theme) {
                footnotes.push(line);
                continue;
            }
            let mut node_blocks = Vec::new();
            Self::append_node(&mut node_blocks, graph, node, 0, 0, theme);
            if node_blocks.is_empty() {
                continue;
            }
            blocks.extend(node_blocks);
        }
        if !footnotes.is_empty() {
            blocks.push(SurfaceBlock::Rule);
            blocks.extend(footnotes.into_iter().map(SurfaceBlock::Line));
        }
        for block in &mut blocks {
            block.apply_typography(typography);
        }
        blocks
    }

    pub(crate) fn node_height_with_typography(
        graph: &BuildGraph,
        node: &KmmNode,
        theme: &KdvThemeSnapshot,
        typography: SurfaceTypographyConfig,
    ) -> u32 {
        let mut blocks = Vec::new();
        Self::append_node(&mut blocks, graph, node, 0, 0, theme);
        for block in &mut blocks {
            block.apply_typography(typography);
        }
        SurfaceHelpers::block_stack_height(blocks.iter().map(SurfaceBlock::height))
    }

    pub(super) fn append_node(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        Self::append_node_with_parts(blocks, graph, node, quote_depth, list_depth, theme);
    }

    fn append_heading(
        blocks: &mut Vec<SurfaceBlock>,
        heading: &katana_markdown_model::HeadingNode,
        theme: &KdvThemeSnapshot,
    ) {
        let spans = SurfaceInlineSpans::from_markdown(&heading.text, theme);
        blocks.push(SurfaceBlock::Line(SurfaceLine::heading_spans(
            heading.level,
            spans,
        )));
    }

    pub(super) fn append_children(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        for child in &node.children {
            Self::append_node(blocks, graph, child, quote_depth, list_depth, theme);
        }
    }
}
