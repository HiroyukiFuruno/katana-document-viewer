use super::KucNodeFactory;
use katana_document_viewer::{ViewerMediaControlAction, ViewerMediaControlSet, ViewerNode};
use katana_ui_core::atom::Button;
use katana_ui_core::layout::{Column, Row, Stack};
use katana_ui_core::render_model::{
    UiBorder, UiCommonProps, UiDimension, UiEdgeInsets, UiNode, UiPosition, UiVariant, UiZIndex,
};

const CODE_CONTROL_STATE_PREFIX: &str = "viewer-media-control";
const CODE_CONTROL_MARGIN_PX: u16 = 12;
const CODE_CONTROL_SIZE_PX: u16 = 28;
const CODE_COPIED_LABEL: &str = "✓";
pub(super) const CODE_BLOCK_PADDING_LEFT_PX: u16 = 24;
pub(super) const CODE_BLOCK_PADDING_RIGHT_PX: u16 =
    CODE_CONTROL_SIZE_PX + CODE_CONTROL_MARGIN_PX.saturating_mul(2);
pub(super) const CODE_BLOCK_PADDING_TOP_PX: u16 = 20;
pub(super) const QUOTED_CODE_PADDING_TOP_PX: u16 = 8;

impl<'a> KucNodeFactory<'a> {
    pub(super) fn code_node(&self, node: &ViewerNode) -> UiNode {
        let block_height = Self::viewer_height(node);
        let body = self
            .text_node(node)
            .common(Self::code_body_common(CODE_BLOCK_PADDING_TOP_PX))
            .height(block_height.clone());
        if !self.interaction.code_controls_enabled {
            return Self::code_with_viewer_rect(body, node, block_height);
        }
        let stack = UiNode::from(Stack::new().child(body).child(self.copy_controls(node)))
            .height(block_height.clone());
        Self::code_with_viewer_rect(stack, node, block_height)
    }

    pub(super) fn code_body_common(top_padding_px: u16) -> UiCommonProps {
        UiCommonProps::default()
            .padding(UiEdgeInsets {
                top: UiDimension::Px(top_padding_px),
                left: UiDimension::Px(CODE_BLOCK_PADDING_LEFT_PX),
                right: UiDimension::Px(CODE_BLOCK_PADDING_RIGHT_PX),
                ..UiEdgeInsets::default()
            })
            .border(UiBorder::solid(1, 0, "document.code.border"))
    }

    fn copy_controls(&self, node: &ViewerNode) -> UiNode {
        UiNode::from(Row::new().child(self.copy_button(node)))
            .position(UiPosition::Absolute)
            .margin(Self::copy_controls_margin())
            .z_index(UiZIndex::value(2))
    }

    fn copy_controls_margin() -> UiEdgeInsets {
        UiEdgeInsets {
            top: UiDimension::Px(CODE_CONTROL_MARGIN_PX),
            right: UiDimension::Px(CODE_CONTROL_MARGIN_PX),
            ..UiEdgeInsets::default()
        }
    }

    fn copy_button(&self, node: &ViewerNode) -> UiNode {
        let spec = ViewerMediaControlSet::code_copy_control();
        let label = if self.is_code_copied(node) {
            CODE_COPIED_LABEL
        } else {
            spec.label
        };
        let state_id = Self::media_control_state_id(CODE_CONTROL_STATE_PREFIX, node, spec);
        UiNode::from(
            Button::new(label)
                .accessibility_label(spec.accessibility_label)
                .value(spec.command)
                .surface_control_action(
                    ViewerMediaControlAction::host_action_id_for(spec.kind, spec.command),
                    spec.command,
                    &node.node_id.0,
                ),
        )
        .stable_node_id(state_id.as_str().to_string())
        .state_id(state_id)
        .width(UiDimension::px(spec.width_px))
        .height(UiDimension::px(spec.height_px))
        .variant(UiVariant::Icon)
    }

    fn is_code_copied(&self, node: &ViewerNode) -> bool {
        self.copied_code_node_ids
            .is_some_and(|node_ids| node_ids.contains(&node.node_id.0))
    }

    fn code_with_viewer_rect(
        ui_node: UiNode,
        node: &ViewerNode,
        block_height: UiDimension,
    ) -> UiNode {
        let left = Self::viewer_rect_x_px(node);
        if left == 0 {
            return ui_node.height(block_height);
        }
        let wrapper: UiNode = Column::new().child(ui_node).into();
        let common = wrapper.props().common.clone().padding(UiEdgeInsets {
            left: UiDimension::Px(left),
            ..UiEdgeInsets::default()
        });
        wrapper.common(common).height(block_height)
    }

    fn viewer_rect_x_px(node: &ViewerNode) -> u16 {
        if node.rect.x <= 0.0 {
            return 0;
        }
        node.rect.x.round().clamp(0.0, f32::from(u16::MAX)) as u16
    }
}
