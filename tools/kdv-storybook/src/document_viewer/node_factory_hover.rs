use super::KucNodeFactory;
use katana_document_viewer::ViewerNode;
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiVisualRole};

impl<'a> KucNodeFactory<'a> {
    pub(crate) fn hovered_node_id(mut self, value: Option<&'a str>) -> Self {
        self.hovered_node_id = value;
        self
    }

    pub(super) fn hover_node_if_needed(&self, ui_node: UiNode, node: &ViewerNode) -> UiNode {
        if !self.interaction.hover_highlight_enabled {
            return ui_node;
        }
        if self.hovered_node_id != Some(node.node_id.0.as_str()) {
            return ui_node;
        }
        let common = ui_node.props().common.clone();
        UiNode::new(UiNodeKind::Stack, "")
            .common(common)
            .visual_role(UiVisualRole::HoverSurface)
            .style_class("kdv-hover-highlight")
            .child(ui_node)
    }

    pub(super) fn viewer_height(node: &ViewerNode) -> UiDimension {
        UiDimension::Px(node.rect.height.round().max(1.0) as u16)
    }
}
