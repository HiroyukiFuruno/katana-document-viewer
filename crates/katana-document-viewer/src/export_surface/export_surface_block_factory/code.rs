use crate::export_html_ops::ExportHtmlOps;
use crate::export_surface_code::SurfaceCodeHighlighter;
use crate::export_surface_line::SurfaceLine;
use crate::forge::{BuildGraph, RenderedDiagram};
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{CodeBlockRole, DiagramKind, KmmNode};

use super::super::{SurfaceBlock, SurfaceCodeBlock, SurfaceDiagramBlock, SurfaceMathBlock};
use super::SurfaceBlockFactory;

impl SurfaceBlockFactory {
    pub(super) fn append_code(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
        role: &CodeBlockRole,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        match role {
            CodeBlockRole::Diagram { kind } => Self::append_diagram(blocks, graph, node, kind),
            CodeBlockRole::Math => Self::append_fenced_math(blocks, node, quote_depth, theme),
            CodeBlockRole::Plain { language } => {
                Self::append_plain_code(blocks, node, language.as_deref(), quote_depth, list_depth);
            }
        }
    }

    fn append_fenced_math(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        _quote_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        Self::append_math_lines(
            blocks,
            &ExportHtmlOps::fenced_body(&node.source.raw.text),
            theme,
        );
    }

    fn append_plain_code(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        language: Option<&str>,
        quote_depth: u32,
        list_depth: u32,
    ) {
        let lines = SurfaceCodeHighlighter::highlight(
            language,
            &ExportHtmlOps::fenced_body(&node.source.raw.text),
        )
        .into_iter()
        .map(SurfaceLine::code_spans)
        .collect::<Vec<_>>();
        blocks.push(SurfaceBlock::Code(SurfaceCodeBlock::new(
            lines,
            quote_depth,
            list_depth,
        )));
    }

    pub(super) fn append_math_lines(
        blocks: &mut Vec<SurfaceBlock>,
        expression: &str,
        theme: &KdvThemeSnapshot,
    ) {
        blocks.push(SurfaceBlock::Math(SurfaceMathBlock::new(
            expression,
            Some(theme.krr_math_theme()),
        )));
    }

    fn append_diagram(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
        kind: &DiagramKind,
    ) {
        if let Some(diagram) = Self::rendered_diagram(graph, node) {
            blocks.push(SurfaceBlock::Diagram(SurfaceDiagramBlock::rendered(
                &diagram.svg,
            )));
            return;
        }
        Self::append_wrapped(
            blocks,
            format!("Diagram rendering unavailable: {kind:?}"),
            0,
            0,
        );
    }

    fn rendered_diagram<'a>(graph: &'a BuildGraph, node: &KmmNode) -> Option<&'a RenderedDiagram> {
        graph
            .rendered_diagrams
            .iter()
            .find(|diagram| diagram.node_id == node.id.0)
    }
}
