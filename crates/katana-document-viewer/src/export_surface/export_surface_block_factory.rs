use crate::export_surface_line::SurfaceLine;
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
        let mut blocks = Vec::new();
        let mut footnotes = Vec::new();
        for node in &graph.snapshot.document.nodes {
            if let Some(line) = Self::footnote_line(node, 0, theme) {
                footnotes.push(line);
                continue;
            }
            Self::append_node(&mut blocks, graph, node, 0, 0, theme);
        }
        if !footnotes.is_empty() {
            blocks.push(SurfaceBlock::Rule);
            blocks.extend(footnotes.into_iter().map(SurfaceBlock::Line));
        }
        blocks
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
