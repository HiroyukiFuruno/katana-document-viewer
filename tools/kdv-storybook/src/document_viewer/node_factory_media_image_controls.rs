use super::KucNodeFactory;
use katana_document_viewer::{
    ViewerDiagramControlSlot, ViewerMediaControlSet, ViewerMediaControlSpec, ViewerNode,
};
use katana_ui_core::layout::{Column, Length, Row, Stack};
use katana_ui_core::render_model::{
    UiDimension, UiEdgeInsets, UiNode, UiNodeKind, UiPosition, UiVariant, UiVisualRole, UiZIndex,
};

const IMAGE_CONTROL_STATE_PREFIX: &str = "viewer-media-control";
const IMAGE_CONTROL_MARGIN_PX: u16 = 8;

impl<'a> KucNodeFactory<'a> {
    pub(crate) fn image_media_with_controls(&self, node: &ViewerNode, media: UiNode) -> UiNode {
        UiNode::from(
            Stack::new()
                .child(media)
                .child(
                    self.image_controls_grid(node)
                        .position(UiPosition::Absolute)
                        .margin(Self::image_bottom_overlay_margin())
                        .z_index(UiZIndex::value(2)),
                )
                .child(
                    self.image_top_controls(node)
                        .position(UiPosition::Absolute)
                        .margin(Self::image_top_overlay_margin())
                        .z_index(UiZIndex::value(2)),
                ),
        )
        .visual_role(UiVisualRole::MediaFrame)
    }

    fn image_top_controls(&self, node: &ViewerNode) -> UiNode {
        let mut row = Row::new();
        for slot in ViewerMediaControlSet::image_top_slots() {
            row = row.child(self.image_slot(node, *slot));
        }
        row.into()
    }

    fn image_controls_grid(&self, node: &ViewerNode) -> UiNode {
        let mut column = Column::new().gap(Length::px(2.0));
        for row_slots in ViewerMediaControlSet::image_grid_rows() {
            column = column.child(self.image_row(node, row_slots));
        }
        UiNode::from(column)
    }

    fn image_row(&self, node: &ViewerNode, slots: &[ViewerDiagramControlSlot]) -> UiNode {
        let mut row = Row::new();
        for slot in slots {
            row = row.child(self.image_slot(node, *slot));
        }
        row.into()
    }

    fn image_slot(&self, node: &ViewerNode, slot: ViewerDiagramControlSlot) -> UiNode {
        match slot {
            ViewerDiagramControlSlot::Control(spec) => self.image_control_button(node, spec),
            ViewerDiagramControlSlot::Gap {
                width_px,
                height_px,
            } => Self::image_gap(width_px, height_px),
            ViewerDiagramControlSlot::Spacer {
                width_px,
                height_px,
            } => Self::image_spacer(width_px, height_px),
        }
    }

    fn image_control_button(&self, node: &ViewerNode, spec: ViewerMediaControlSpec) -> UiNode {
        self.media_control_button(node, spec, IMAGE_CONTROL_STATE_PREFIX)
            .variant(UiVariant::Icon)
    }

    fn image_spacer(width_px: u16, height_px: u16) -> UiNode {
        UiNode::new(UiNodeKind::Stack, "")
            .width(UiDimension::px(width_px))
            .height(UiDimension::px(height_px))
    }

    fn image_gap(width_px: u16, height_px: u16) -> UiNode {
        UiNode::new(UiNodeKind::Stack, "")
            .width(UiDimension::px(width_px))
            .height(UiDimension::px(height_px))
    }

    fn image_top_overlay_margin() -> UiEdgeInsets {
        UiEdgeInsets {
            top: UiDimension::px(IMAGE_CONTROL_MARGIN_PX),
            right: UiDimension::px(IMAGE_CONTROL_MARGIN_PX),
            bottom: UiDimension::px(0),
            left: UiDimension::px(0),
        }
    }

    fn image_bottom_overlay_margin() -> UiEdgeInsets {
        UiEdgeInsets {
            top: UiDimension::px(0),
            right: UiDimension::px(IMAGE_CONTROL_MARGIN_PX),
            bottom: UiDimension::px(IMAGE_CONTROL_MARGIN_PX),
            left: UiDimension::px(0),
        }
    }
}
