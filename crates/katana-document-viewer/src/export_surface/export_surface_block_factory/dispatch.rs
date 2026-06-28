use super::super::markup::SurfaceHtmlMarkup;
use super::{
    BuildGraph, KdvThemeSnapshot, KmmNode, KmmNodeKind, SurfaceBlock, SurfaceBlockFactory,
};
use katana_markdown_model::HtmlBlockRole;

#[derive(Clone, Copy)]
struct SurfaceAppendContext<'a> {
    graph: &'a BuildGraph,
    quote_depth: u32,
    list_depth: u32,
    theme: &'a KdvThemeSnapshot,
}

impl SurfaceBlockFactory {
    pub(super) fn append_node_with_parts(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        let context = SurfaceAppendContext {
            graph,
            quote_depth,
            list_depth,
            theme,
        };
        Self::append_node_with_context(blocks, node, context);
    }

    fn append_node_with_context(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        context: SurfaceAppendContext<'_>,
    ) {
        if Self::append_primary_node(blocks, node, context) {
            return;
        }
        if Self::append_remaining_structured_node(blocks, node, &node.kind, context) {
            return;
        }
        Self::append_remaining_fallback_node(blocks, node, &node.kind, context);
    }

    fn append_primary_node(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        context: SurfaceAppendContext<'_>,
    ) -> bool {
        if Self::append_primary_text_node(blocks, node, context) {
            return true;
        }
        Self::append_primary_container_node(blocks, node, context)
    }

    fn append_primary_text_node(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        context: SurfaceAppendContext<'_>,
    ) -> bool {
        match &node.kind {
            KmmNodeKind::Heading(heading) => Self::append_heading(blocks, heading, context.theme),
            KmmNodeKind::Paragraph => {
                Self::append_paragraph(blocks, node, context);
            }
            KmmNodeKind::CodeBlock(role) => Self::append_code(
                blocks,
                context.graph,
                node,
                role,
                context.quote_depth,
                context.list_depth,
                context.theme,
            ),
            _ => return false,
        }
        true
    }

    fn append_paragraph(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        context: SurfaceAppendContext<'_>,
    ) {
        if Self::append_standalone_image(blocks, context.graph, node) {
            return;
        }
        if SurfaceHtmlMarkup::starts_with_block_tag(&node.source.raw.text) {
            Self::append_html(
                blocks,
                context.graph,
                node,
                &HtmlBlockRole::Generic,
                context.quote_depth,
                context.list_depth,
                context.theme,
            );
            return;
        }
        Self::append_rich_line(
            blocks,
            node,
            context.quote_depth,
            context.list_depth,
            context.theme,
        );
    }

    fn append_primary_container_node(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        context: SurfaceAppendContext<'_>,
    ) -> bool {
        match &node.kind {
            KmmNodeKind::BlockQuote => Self::append_block_quote(
                blocks,
                context.graph,
                node,
                context.quote_depth,
                context.list_depth,
                context.theme,
            ),
            KmmNodeKind::Alert { label } => {
                Self::append_alert(blocks, node, label, context.quote_depth, context.list_depth)
            }
            _ => return false,
        }
        true
    }

    fn append_remaining_structured_node(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        kind: &KmmNodeKind,
        context: SurfaceAppendContext<'_>,
    ) -> bool {
        match kind {
            KmmNodeKind::Table(table) => Self::append_remaining_table(blocks, node, table, context),
            KmmNodeKind::HtmlBlock(role) => {
                Self::append_remaining_html(blocks, node, role, context)
            }
            KmmNodeKind::DollarMathBlock(math) => {
                Self::append_remaining_math(blocks, math, context)
            }
            _ => return false,
        }
        true
    }

    fn append_remaining_table(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        table: &katana_markdown_model::TableNode,
        context: SurfaceAppendContext<'_>,
    ) {
        Self::append_table(
            blocks,
            table,
            &node.source.raw.text,
            context.quote_depth,
            context.list_depth,
            context.theme,
        );
    }

    fn append_remaining_html(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        role: &HtmlBlockRole,
        context: SurfaceAppendContext<'_>,
    ) {
        Self::append_html(
            blocks,
            context.graph,
            node,
            role,
            context.quote_depth,
            context.list_depth,
            context.theme,
        );
    }

    fn append_remaining_math(
        blocks: &mut Vec<SurfaceBlock>,
        math: &katana_markdown_model::DollarMathBlockNode,
        context: SurfaceAppendContext<'_>,
    ) {
        Self::append_math_lines(blocks, &math.expression, context.theme);
    }
}

#[path = "dispatch_remaining.rs"]
mod dispatch_remaining;

#[cfg(test)]
#[path = "dispatch_tests.rs"]
mod tests;
