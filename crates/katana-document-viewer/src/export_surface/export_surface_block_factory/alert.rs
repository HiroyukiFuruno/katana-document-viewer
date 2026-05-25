use katana_markdown_model::KmmNode;

use super::super::markup::{alert_body_lines, legacy_note_children};
use super::super::{SurfaceAlertBlock, SurfaceBlock};
use super::SurfaceBlockFactory;

impl SurfaceBlockFactory {
    pub(super) fn append_alert(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        label: &str,
        quote_depth: u32,
        list_depth: u32,
    ) {
        if let Some((title, body)) = legacy_note_children(&node.children) {
            Self::append_wrapped(
                blocks,
                format!("{title} {body}"),
                quote_depth + 1,
                list_depth,
            );
            return;
        }
        blocks.push(SurfaceBlock::Alert(SurfaceAlertBlock::new(
            label,
            alert_body_lines(node),
            quote_depth,
        )));
    }
}
