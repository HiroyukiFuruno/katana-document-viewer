use crate::visual::UiTreeNodeHit;
use katana_document_viewer::{
    ViewerMediaControlAction, ViewerMediaControlKind, ViewerMediaControlSet,
};
use katana_ui_core::render_model::{UiNode, UiNodeId};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct KucDiagramControlResolver;

impl KucDiagramControlResolver {
    #[must_use]
    pub fn internal_action_for_node(
        root: &UiNode,
        node_id: &UiNodeId,
    ) -> Option<ViewerMediaControlAction> {
        let node = find_node(root, node_id)?;
        internal_action_for_control_node(node)
    }

    #[must_use]
    pub fn internal_action_at(
        root: &UiNode,
        hits: &[UiTreeNodeHit],
        x: f32,
        y: f32,
    ) -> Option<ViewerMediaControlAction> {
        let mut matching_hits = hits
            .iter()
            .filter(|hit| hit.contains_point(x, y))
            .collect::<Vec<_>>();
        matching_hits.sort_by_key(|hit| hit.rect.area());
        matching_hits
            .into_iter()
            .find_map(|hit| Self::internal_action_for_node(root, &hit.node_id))
    }

    #[must_use]
    pub fn internal_control_node_id_at(
        root: &UiNode,
        hits: &[UiTreeNodeHit],
        x: f32,
        y: f32,
    ) -> Option<UiNodeId> {
        let mut matching_hits = hits
            .iter()
            .filter(|hit| hit.contains_point(x, y))
            .collect::<Vec<_>>();
        matching_hits.sort_by_key(|hit| hit.rect.area());
        matching_hits.into_iter().find_map(|hit| {
            Self::internal_action_for_node(root, &hit.node_id)
                .map(|_| hit.node_id.clone())
                .or_else(|| {
                    let semantic_node_id = hit.semantic_node_id.as_ref()?;
                    Self::internal_action_for_node(root, semantic_node_id)?;
                    Some(semantic_node_id.clone())
                })
        })
    }
}

fn internal_action_for_control_node(node: &UiNode) -> Option<ViewerMediaControlAction> {
    if node.has_host_action() {
        return None;
    }
    let command = node.props().interaction.value.as_str();
    if !is_internal_diagram_command(command) {
        return None;
    }
    let node_id = target_node_id(node)?;
    Some(ViewerMediaControlAction::new(
        ViewerMediaControlKind::Diagram,
        node_id,
        command,
    ))
}

fn is_internal_diagram_command(command: &str) -> bool {
    ViewerMediaControlSet::diagram_grid_rows()
        .into_iter()
        .flat_map(|row| row.iter())
        .any(|slot| match slot {
            katana_document_viewer::ViewerDiagramControlSlot::Control(spec) => {
                spec.command == command
            }
            katana_document_viewer::ViewerDiagramControlSlot::Gap { .. }
            | katana_document_viewer::ViewerDiagramControlSlot::Spacer { .. } => false,
        })
}

fn target_node_id(node: &UiNode) -> Option<String> {
    let target = node.props().interaction.surface_control_target_id.as_str();
    if target.is_empty() {
        return None;
    }
    Some(target.to_string())
}

fn find_node<'a>(node: &'a UiNode, node_id: &UiNodeId) -> Option<&'a UiNode> {
    if node.id() == node_id {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| find_node(child, node_id))
}
