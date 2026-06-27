use super::KucNodeFactory;
use katana_document_viewer::{
    ViewerMediaControlAction, ViewerMediaControlKind, ViewerMediaControlSet,
    ViewerMediaControlSpec, ViewerNode, ViewerNodeKind,
};
use katana_ui_core::atom::Button;
use katana_ui_core::layout::Stack;
use katana_ui_core::render_model::{
    UiDimension, UiEdgeInsets, UiIconProps, UiNode, UiNodeKind, UiPosition, UiStateId,
    UiVisualRole, UiZIndex,
};

const DIAGRAM_CONTROL_MARGIN_PX: u16 = 8;
const FULLSCREEN_CLOSE_CONTROL_MARGIN_PX: u16 = 20;
const KATANA_MIN_CONTROL_CONTAINER_HEIGHT_PX: u16 = 145;
const EXPORT_MEDIA_VERTICAL_MARGIN_PX: u16 = 18;
const DISPLAY_SIZE_MILLI: f32 = 1000.0;
const CONTENT_SCALE_PERCENT: u64 = 100;

impl<'a> KucNodeFactory<'a> {
    pub(super) fn media_with_controls(&self, node: &ViewerNode, media: UiNode) -> UiNode {
        if matches!(node.kind, ViewerNodeKind::Diagram { .. }) && !self.media_controls_enabled(node)
        {
            return Self::diagram_media_frame(media);
        }
        if !self.media_controls_enabled(node) {
            return media;
        }
        if matches!(node.kind, ViewerNodeKind::Image) {
            return self.image_media_with_controls(node, media);
        }
        if matches!(node.kind, ViewerNodeKind::Diagram { .. }) {
            return self.diagram_media_with_controls(node, media);
        }
        media
    }

    fn diagram_media_with_controls(&self, node: &ViewerNode, media: UiNode) -> UiNode {
        let container_height = self.diagram_control_container_height(node, &media);
        let media = media.height(UiDimension::px(container_height));
        let mut stack = Stack::new();
        if self.diagram_fullscreen_open(node) {
            stack = stack.child(self.diagram_fullscreen_backdrop(node, container_height));
        }
        stack = stack
            .child(media)
            .child(
                self.diagram_controls_grid(node)
                    .position(UiPosition::Absolute)
                    .margin(Self::diagram_bottom_overlay_margin())
                    .z_index(UiZIndex::value(2)),
            )
            .child(
                self.diagram_top_controls(node)
                    .position(UiPosition::Absolute)
                    .margin(self.diagram_top_overlay_margin(node))
                    .z_index(UiZIndex::value(2)),
            );
        let mut frame = UiNode::from(stack)
            .height(UiDimension::px(container_height))
            .visual_role(UiVisualRole::MediaFrame);
        if self.diagram_fullscreen_open(node) {
            frame = frame.width(UiDimension::px(Self::clamped_viewport_extent(
                self.fullscreen_control_container_width(),
            )));
        }
        frame
    }

    fn diagram_fullscreen_backdrop(&self, node: &ViewerNode, container_height: u16) -> UiNode {
        let spec = ViewerMediaControlSet::diagram_fullscreen_control();
        UiNode::new(UiNodeKind::Stack, "")
            .stable_node_id(format!("viewer-diagram-backdrop:{}", node.node_id.0))
            .width(UiDimension::px(Self::clamped_viewport_extent(
                self.fullscreen_control_container_width(),
            )))
            .height(UiDimension::px(container_height))
            .position(UiPosition::Absolute)
            .z_index(UiZIndex::value(0))
            .surface_control_action(
                Self::media_host_action_id(spec.kind, spec.command),
                spec.command,
                &node.node_id.0,
            )
    }

    fn diagram_media_frame(media: UiNode) -> UiNode {
        let visual_role = if media.props().visual_role == UiVisualRole::ExportMediaFrame {
            UiVisualRole::ExportMediaFrame
        } else {
            UiVisualRole::MediaFrame
        };
        let container_height = Self::media_control_container_height(&media);
        media
            .height(UiDimension::px(container_height))
            .visual_role(visual_role)
    }

    fn media_control_container_height(media: &UiNode) -> u16 {
        let content_height = match media.props().common.height {
            UiDimension::Px(height) => height,
            _ => Self::logical_media_height(media),
        };
        let height = if media.props().visual_role == UiVisualRole::ExportMediaFrame {
            content_height.saturating_add(EXPORT_MEDIA_VERTICAL_MARGIN_PX.saturating_mul(2))
        } else {
            content_height
        };
        if media.props().visual_role == UiVisualRole::ExportMediaFrame {
            return height;
        }
        height.max(KATANA_MIN_CONTROL_CONTAINER_HEIGHT_PX)
    }

    fn diagram_control_container_height(&self, node: &ViewerNode, media: &UiNode) -> u16 {
        if self.diagram_fullscreen_open(node)
            && let Some(height) = self.fullscreen_viewport_height
        {
            return height.min(u32::from(u16::MAX)) as u16;
        }
        Self::media_control_container_height(media)
    }

    fn fullscreen_control_container_width(&self) -> u32 {
        self.fullscreen_viewport_width
            .unwrap_or(self.max_media_width)
    }

    pub(super) fn clamped_viewport_extent(value: u32) -> u16 {
        value.min(u32::from(u16::MAX)) as u16
    }

    fn logical_media_height(media: &UiNode) -> u16 {
        let surface = &media.props().image_surface;
        if surface.display_height_milli > 0 {
            return (surface.display_height_milli as f32 / DISPLAY_SIZE_MILLI)
                .ceil()
                .min(f32::from(u16::MAX)) as u16;
        }
        if surface.display_height > 0 {
            return surface.display_height.min(u32::from(u16::MAX)) as u16;
        }
        let scale = u64::from(surface.content_scale.max(1));
        let logical = (u64::from(surface.height) * CONTENT_SCALE_PERCENT)
            .div_ceil(scale)
            .max(1);
        logical.min(u64::from(u16::MAX)) as u16
    }

    fn diagram_top_overlay_margin(&self, node: &ViewerNode) -> UiEdgeInsets {
        let margin = if self.diagram_fullscreen_open(node) {
            FULLSCREEN_CLOSE_CONTROL_MARGIN_PX
        } else {
            DIAGRAM_CONTROL_MARGIN_PX
        };
        UiEdgeInsets {
            top: UiDimension::px(margin),
            right: UiDimension::px(margin),
            bottom: UiDimension::px(0),
            left: UiDimension::px(0),
        }
    }

    fn diagram_bottom_overlay_margin() -> UiEdgeInsets {
        UiEdgeInsets {
            top: UiDimension::px(0),
            right: UiDimension::px(DIAGRAM_CONTROL_MARGIN_PX),
            bottom: UiDimension::px(DIAGRAM_CONTROL_MARGIN_PX),
            left: UiDimension::px(0),
        }
    }

    pub(super) fn media_controls_enabled(&self, node: &ViewerNode) -> bool {
        match node.kind {
            ViewerNodeKind::Image => self.interaction.image_controls_enabled,
            ViewerNodeKind::Diagram { .. } => self.interaction.diagram_controls_enabled,
            _ => false,
        }
    }

    pub(super) fn media_host_action_id(kind: ViewerMediaControlKind, action: &str) -> String {
        ViewerMediaControlAction::host_action_id_for(kind, action)
    }

    pub(super) fn media_control_state_id(
        prefix: &str,
        node: &ViewerNode,
        spec: ViewerMediaControlSpec,
    ) -> UiStateId {
        UiStateId::new(format!("{prefix}:{}:{}", node.node_id.0, spec.command))
    }

    pub(super) fn media_control_button(
        &self,
        node: &ViewerNode,
        spec: ViewerMediaControlSpec,
        state_prefix: &str,
    ) -> UiNode {
        self.media_control_button_base(node, spec, state_prefix)
            .surface_control_action(
                Self::media_host_action_id(spec.kind, spec.command),
                spec.command,
                &node.node_id.0,
            )
    }

    pub(super) fn internal_media_control_button(
        &self,
        node: &ViewerNode,
        spec: ViewerMediaControlSpec,
        state_prefix: &str,
    ) -> UiNode {
        self.media_control_button_base(node, spec, state_prefix)
            .surface_control_target_id(node.node_id.0.clone())
    }

    fn media_control_button_base(
        &self,
        node: &ViewerNode,
        spec: ViewerMediaControlSpec,
        state_prefix: &str,
    ) -> UiNode {
        let state_id = Self::media_control_state_id(state_prefix, node, spec);
        UiNode::from(
            Button::new(spec.label)
                .accessibility_label(spec.accessibility_label)
                .value(spec.command),
        )
        .icon(self.surface_control_icon(spec))
        .stable_node_id(state_id.as_str().to_string())
        .state_id(state_id)
        .width(UiDimension::px(spec.width_px))
        .height(UiDimension::px(spec.height_px))
    }

    fn surface_control_icon(&self, spec: ViewerMediaControlSpec) -> UiIconProps {
        self.media_control_icons
            .icon_for(spec.command, spec.icon_svg)
    }
}
