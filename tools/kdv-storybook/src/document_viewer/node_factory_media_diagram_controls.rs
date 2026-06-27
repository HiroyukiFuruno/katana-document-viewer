use super::KucNodeFactory;
use katana_document_viewer::{
    ViewerDiagramControlSlot, ViewerMediaControlSet, ViewerMediaControlSpec, ViewerNode,
};
use katana_ui_core::layout::{Column, Length, Row};
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiVariant};

const DIAGRAM_CONTROL_STATE_PREFIX: &str = "viewer-media-control";
const INTERNAL_DIAGRAM_CONTROL_STATE_PREFIX: &str = "viewer-media-internal-control";
const FULLSCREEN_CLOSE_COMMAND: &str = "close-modal";
const FULLSCREEN_CLOSE_CONTROL_SIZE_PX: u16 = 32;

impl<'a> KucNodeFactory<'a> {
    pub(crate) fn diagram_top_controls(&self, node: &ViewerNode) -> UiNode {
        let mut row = Row::new();
        for slot in ViewerMediaControlSet::diagram_top_slots() {
            row = row.child(self.diagram_top_slot(node, *slot));
        }
        UiNode::from(row)
    }

    fn diagram_top_slot(&self, node: &ViewerNode, slot: ViewerDiagramControlSlot) -> UiNode {
        match slot {
            ViewerDiagramControlSlot::Control(spec) => self.diagram_fullscreen_button(node, spec),
            ViewerDiagramControlSlot::Gap {
                width_px,
                height_px,
            } => Self::diagram_gap(width_px, height_px),
            ViewerDiagramControlSlot::Spacer {
                width_px,
                height_px,
            } => Self::diagram_spacer(width_px, height_px),
        }
    }

    pub(crate) fn diagram_controls_grid(&self, node: &ViewerNode) -> UiNode {
        let mut column = Column::new().gap(Length::px(2.0));
        for row_slots in ViewerMediaControlSet::diagram_grid_rows() {
            column = column.child(self.diagram_row(node, row_slots));
        }
        UiNode::from(column)
    }

    fn diagram_row(&self, node: &ViewerNode, slots: &[ViewerDiagramControlSlot]) -> UiNode {
        let mut row = Row::new();
        for slot in slots {
            row = row.child(self.diagram_grid_slot(node, *slot));
        }
        row.into()
    }

    fn diagram_grid_slot(&self, node: &ViewerNode, slot: ViewerDiagramControlSlot) -> UiNode {
        match slot {
            ViewerDiagramControlSlot::Control(spec) => {
                self.diagram_internal_control_button(node, spec)
            }
            ViewerDiagramControlSlot::Gap {
                width_px,
                height_px,
            } => Self::diagram_gap(width_px, height_px),
            ViewerDiagramControlSlot::Spacer {
                width_px,
                height_px,
            } => Self::diagram_spacer(width_px, height_px),
        }
    }

    fn diagram_fullscreen_button(&self, node: &ViewerNode, spec: ViewerMediaControlSpec) -> UiNode {
        let mut button = self
            .media_control_button(node, spec, DIAGRAM_CONTROL_STATE_PREFIX)
            .variant(UiVariant::Icon);
        if self.diagram_fullscreen_open(node) {
            button = button
                .icon(
                    self.media_control_icons
                        .icon_for(FULLSCREEN_CLOSE_COMMAND, ""),
                )
                .width(UiDimension::px(FULLSCREEN_CLOSE_CONTROL_SIZE_PX))
                .height(UiDimension::px(FULLSCREEN_CLOSE_CONTROL_SIZE_PX));
        }
        button
    }

    fn diagram_internal_control_button(
        &self,
        node: &ViewerNode,
        spec: ViewerMediaControlSpec,
    ) -> UiNode {
        self.internal_media_control_button(node, spec, INTERNAL_DIAGRAM_CONTROL_STATE_PREFIX)
            .variant(UiVariant::Icon)
    }

    fn diagram_spacer(width_px: u16, height_px: u16) -> UiNode {
        UiNode::new(UiNodeKind::Stack, "")
            .width(UiDimension::px(width_px))
            .height(UiDimension::px(height_px))
    }

    fn diagram_gap(width_px: u16, height_px: u16) -> UiNode {
        UiNode::new(UiNodeKind::Stack, "")
            .width(UiDimension::px(width_px))
            .height(UiDimension::px(height_px))
    }
}
