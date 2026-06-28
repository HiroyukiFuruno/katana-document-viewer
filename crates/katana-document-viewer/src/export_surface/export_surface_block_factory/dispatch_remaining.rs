use super::{KmmNode, KmmNodeKind, SurfaceAppendContext, SurfaceBlock, SurfaceBlockFactory};

impl SurfaceBlockFactory {
    pub(super) fn append_remaining_fallback_node(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        kind: &KmmNodeKind,
        context: SurfaceAppendContext<'_>,
    ) {
        if Self::append_remaining_definition_node(blocks, node, kind, context) {
            return;
        }
        Self::append_remaining_container_node(blocks, node, kind, context);
    }

    fn append_remaining_definition_node(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        kind: &KmmNodeKind,
        context: SurfaceAppendContext<'_>,
    ) -> bool {
        match kind {
            KmmNodeKind::FootnoteDefinition(_) => {
                Self::append_footnote_definition(blocks, node, context.quote_depth, context.theme)
            }
            KmmNodeKind::RawBlock { .. } => Self::append_raw(
                blocks,
                &node.source.raw.text,
                context.quote_depth,
                context.list_depth,
            ),
            _ => return false,
        }
        true
    }

    fn append_remaining_container_node(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        kind: &KmmNodeKind,
        context: SurfaceAppendContext<'_>,
    ) {
        match kind {
            KmmNodeKind::List(list) => Self::append_list(
                blocks,
                context.graph,
                list,
                context.quote_depth,
                context.list_depth,
                context.theme,
            ),
            KmmNodeKind::ThematicBreak => blocks.push(SurfaceBlock::Rule),
            _ => Self::append_wrapped(
                blocks,
                crate::export_surface_text::SurfaceTextParser::inline_markdown_text(
                    &node.source.raw.text,
                ),
                context.quote_depth,
                context.list_depth,
            ),
        }
    }
}
