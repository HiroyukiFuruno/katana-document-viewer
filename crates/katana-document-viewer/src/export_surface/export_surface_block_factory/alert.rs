use katana_markdown_model::KmmNode;

use super::super::markup::{alert_body_lines, legacy_note_children};
use super::super::{SurfaceAlertBlock, SurfaceBlock};
use super::SurfaceBlockFactory;

impl SurfaceBlockFactory {
    pub(super) fn append_alert(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        label: &str,
        context: super::dispatch::SurfaceAppendContext<'_>,
    ) {
        if let Some((title, body)) = legacy_note_children(&node.children) {
            Self::append_wrapped(
                blocks,
                format!("{title} {body}"),
                context.quote_depth + 1,
                context.list_depth,
            );
            return;
        }
        blocks.push(SurfaceBlock::Alert(SurfaceAlertBlock::new(
            label,
            alert_body_lines(node),
            context.quote_depth,
            context.theme,
        )));
    }
}
