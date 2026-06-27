use crate::export_surface::SurfaceBlock;
use crate::{ViewerNode, ViewerNodePlan};

const HEIGHT_MARKERS: &[&str] = &[
    "KatanA Rendering Regression Test",
    "HTML Centering",
    "Badge Row",
    "Full README Header Reproduction",
    "Basic Markdown Elements",
    "Text Decoration",
    "Links",
    "Lists",
    "Unordered Lists",
    "Ordered Lists",
    "Task Lists",
    "Code Blocks",
    "Accordion",
    "Alert Blocks",
    "Edge Cases",
    "Very Long Line",
    "Special Characters",
    "Consecutive Diagrams",
    "Verification Complete",
];

pub(super) fn plan_height_failure_message(
    plan: &ViewerNodePlan,
    blocks: &[SurfaceBlock],
) -> String {
    format!(
        "viewer plan must use the same block stack height as export surface: plan_nodes={} export_blocks={} first_nodes=[{}] first_blocks=[{}] last_nodes=[{}] last_blocks=[{}] markers=[{}]",
        plan.nodes.len(),
        blocks.len(),
        first_nodes(plan),
        first_blocks(blocks),
        last_nodes(plan),
        last_blocks(blocks),
        height_marker_deltas(plan, blocks)
    )
}

pub(super) fn first_line(text: &str) -> &str {
    text.lines().next().unwrap_or("")
}

fn first_nodes(plan: &ViewerNodePlan) -> String {
    plan.nodes
        .iter()
        .take(12)
        .map(node_debug)
        .collect::<Vec<_>>()
        .join(" | ")
}

fn last_nodes(plan: &ViewerNodePlan) -> String {
    plan.nodes
        .iter()
        .rev()
        .take(12)
        .map(node_debug)
        .collect::<Vec<_>>()
        .join(" | ")
}

fn first_blocks(blocks: &[SurfaceBlock]) -> String {
    block_debug(blocks.iter().take(18))
}

fn last_blocks(blocks: &[SurfaceBlock]) -> String {
    block_debug(blocks.iter().rev().take(18))
}

fn height_marker_deltas(plan: &ViewerNodePlan, blocks: &[SurfaceBlock]) -> String {
    HEIGHT_MARKERS
        .iter()
        .map(|marker| height_marker_delta(plan, blocks, marker))
        .collect::<Vec<_>>()
        .join(" | ")
}

fn height_marker_delta(plan: &ViewerNodePlan, blocks: &[SurfaceBlock], marker: &str) -> String {
    format!(
        "{}={:?}/{:?}",
        marker,
        plan_marker_y(plan, marker),
        export_marker_y(blocks, marker)
    )
}

fn plan_marker_y(plan: &ViewerNodePlan, marker: &str) -> Option<i32> {
    plan.nodes
        .iter()
        .find(|node| node.text.contains(marker) || node.source.raw.text.contains(marker))
        .map(|node| node.rect.y.round() as i32)
}

fn export_marker_y(blocks: &[SurfaceBlock], marker: &str) -> Option<i32> {
    let mut y = 0_u32;
    for block in blocks {
        if block.text_for_tests().contains(marker) {
            return Some(y as i32);
        }
        y = y.saturating_add(block.height());
    }
    None
}

fn block_debug<'a>(blocks: impl Iterator<Item = &'a SurfaceBlock>) -> String {
    blocks
        .map(|block| format!("{}:{}", block.height(), block.debug_for_tests()))
        .collect::<Vec<_>>()
        .join(" | ")
}

fn node_debug(node: &ViewerNode) -> String {
    format!(
        "{:?}@{}+{} {:?}",
        node.kind,
        node.rect.y,
        node.rect.height,
        first_line(&node.source.raw.text)
    )
}
