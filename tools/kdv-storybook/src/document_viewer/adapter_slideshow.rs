use crate::document_viewer::config::KucViewerConfig;
use katana_document_viewer::{PreviewOutput, ViewerMode, ViewerSlideshowControlAction};
use katana_ui_core::atom::Button;
use katana_ui_core::layout::{Length, Row, Stack};
use katana_ui_core::render_model::{UiDimension, UiEdgeInsets, UiNode, UiPosition, UiZIndex};

const SLIDESHOW_CONTROL_SIZE_PX: u16 = 36;
const SLIDESHOW_CONTROL_GAP_PX: u16 = 8;
const SLIDESHOW_CONTROL_MARGIN_PX: u16 = 12;
const SLIDESHOW_CONTROL_Z_INDEX: i32 = 4;

pub(super) fn viewer_root_with_controls(
    scroll: UiNode,
    output: &PreviewOutput,
    config: &KucViewerConfig,
) -> UiNode {
    if output.input.mode != ViewerMode::Slideshow {
        return scroll;
    }
    UiNode::from(
        Stack::new()
            .child(scroll)
            .child(
                slideshow_close_control()
                    .position(UiPosition::Absolute)
                    .margin(slideshow_top_margin())
                    .z_index(UiZIndex::value(SLIDESHOW_CONTROL_Z_INDEX)),
            )
            .child(
                slideshow_page_controls()
                    .position(UiPosition::Absolute)
                    .margin(slideshow_bottom_margin())
                    .z_index(UiZIndex::value(SLIDESHOW_CONTROL_Z_INDEX)),
            ),
    )
    .width(logical_dimension(config.viewport.width))
    .height(logical_dimension(config.viewport.height))
}

fn slideshow_close_control() -> UiNode {
    slideshow_button(ViewerSlideshowControlAction::Close, "x")
}

fn slideshow_page_controls() -> UiNode {
    UiNode::from(
        Row::new()
            .gap(Length::px(f32::from(SLIDESHOW_CONTROL_GAP_PX)))
            .child(slideshow_button(
                ViewerSlideshowControlAction::PreviousPage,
                "<",
            ))
            .child(slideshow_button(
                ViewerSlideshowControlAction::NextPage,
                ">",
            )),
    )
}

fn slideshow_button(action: ViewerSlideshowControlAction, label: &'static str) -> UiNode {
    UiNode::from(
        Button::new(label)
            .accessibility_label(action.label())
            .value(action.command())
            .command_action(action.host_action_id(), action.label()),
    )
    .width(UiDimension::px(SLIDESHOW_CONTROL_SIZE_PX))
    .height(UiDimension::px(SLIDESHOW_CONTROL_SIZE_PX))
}

fn slideshow_top_margin() -> UiEdgeInsets {
    UiEdgeInsets {
        top: UiDimension::px(SLIDESHOW_CONTROL_MARGIN_PX),
        right: UiDimension::px(SLIDESHOW_CONTROL_MARGIN_PX),
        bottom: UiDimension::px(0),
        left: UiDimension::px(0),
    }
}

fn slideshow_bottom_margin() -> UiEdgeInsets {
    UiEdgeInsets {
        top: UiDimension::px(0),
        right: UiDimension::px(SLIDESHOW_CONTROL_MARGIN_PX),
        bottom: UiDimension::px(SLIDESHOW_CONTROL_MARGIN_PX),
        left: UiDimension::px(0),
    }
}

fn logical_extent(value: f32) -> u32 {
    if value <= 0.0 {
        return 0;
    }
    value.round() as u32
}

fn logical_dimension(value: f32) -> UiDimension {
    UiDimension::Px(logical_extent(value).min(u32::from(u16::MAX)) as u16)
}
