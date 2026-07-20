#[path = "media_control_layout_diagram_consts.rs"]
mod diagram_consts;
#[path = "media_control_layout_image_consts.rs"]
mod image_consts;

use super::{
    ViewerDiagramControlSlot, ViewerMediaControlKind, ViewerMediaControlSpec,
    media_control_icons::surface_control_svg,
};
use diagram_consts::{
    DIAGRAM_GRID_BOTTOM, DIAGRAM_GRID_MIDDLE, DIAGRAM_GRID_TOP, DIAGRAM_TOP_SLOTS,
};
use image_consts::{
    IMAGE_CONTROLS, IMAGE_GRID_BOTTOM, IMAGE_GRID_MIDDLE, IMAGE_GRID_TOP, IMAGE_TOP_SLOTS,
};

const SQUARE_CONTROL_SIZE_PX: u16 = 28;
#[cfg(test)]
const IMAGE_CONTROL_SIZE_PX: u16 = SQUARE_CONTROL_SIZE_PX;
const CODE_CONTROL_SIZE_PX: u16 = SQUARE_CONTROL_SIZE_PX;
#[cfg(test)]
const DIAGRAM_CONTROL_GAP_PX: u16 = 2;
#[cfg(test)]
const IMAGE_CONTROL_GAP_PX: u16 = DIAGRAM_CONTROL_GAP_PX;

pub(crate) struct ViewerMediaControlLayout;

impl ViewerMediaControlLayout {
    #[must_use]
    pub(crate) fn image_controls() -> &'static [ViewerMediaControlSpec] {
        &IMAGE_CONTROLS
    }

    #[must_use]
    pub(crate) fn image_top_slots() -> &'static [ViewerDiagramControlSlot] {
        &IMAGE_TOP_SLOTS
    }

    #[must_use]
    pub(crate) fn image_grid_rows() -> [&'static [ViewerDiagramControlSlot]; 3] {
        [&IMAGE_GRID_TOP, &IMAGE_GRID_MIDDLE, &IMAGE_GRID_BOTTOM]
    }

    #[must_use]
    pub(crate) fn code_copy_control() -> ViewerMediaControlSpec {
        Self::code_control("copy-code", "⧉")
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn image_control(
        command: &'static str,
        label: &'static str,
    ) -> ViewerMediaControlSpec {
        Self::build_image_control(command, label)
    }

    #[must_use]
    pub(crate) fn code_control(
        command: &'static str,
        label: &'static str,
    ) -> ViewerMediaControlSpec {
        Self::build_code_control(command, label)
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn diagram_slot(
        command: &'static str,
        label: &'static str,
    ) -> ViewerDiagramControlSlot {
        ViewerDiagramControlSlot::Control(Self::diagram_control(command, label))
    }

    #[must_use]
    pub(crate) fn diagram_control(
        command: &'static str,
        label: &'static str,
    ) -> ViewerMediaControlSpec {
        Self::build_diagram_control(command, label)
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn image_slot(
        command: &'static str,
        label: &'static str,
    ) -> ViewerDiagramControlSlot {
        ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
            kind: ViewerMediaControlKind::Image,
            command,
            label,
            accessibility_label: command,
            icon_svg: surface_control_svg(command),
            width_px: IMAGE_CONTROL_SIZE_PX,
            height_px: IMAGE_CONTROL_SIZE_PX,
        })
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn grid_gap() -> ViewerDiagramControlSlot {
        ViewerDiagramControlSlot::Gap {
            width_px: DIAGRAM_CONTROL_GAP_PX,
            height_px: SQUARE_CONTROL_SIZE_PX,
        }
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn image_gap() -> ViewerDiagramControlSlot {
        ViewerDiagramControlSlot::Gap {
            width_px: IMAGE_CONTROL_GAP_PX,
            height_px: IMAGE_CONTROL_SIZE_PX,
        }
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn image_spacer() -> ViewerDiagramControlSlot {
        ViewerDiagramControlSlot::Spacer {
            width_px: IMAGE_CONTROL_SIZE_PX,
            height_px: IMAGE_CONTROL_SIZE_PX,
        }
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn spacer() -> ViewerDiagramControlSlot {
        ViewerDiagramControlSlot::Spacer {
            width_px: SQUARE_CONTROL_SIZE_PX,
            height_px: SQUARE_CONTROL_SIZE_PX,
        }
    }

    #[must_use]
    pub(crate) fn diagram_top_slots() -> &'static [ViewerDiagramControlSlot] {
        &DIAGRAM_TOP_SLOTS
    }

    #[must_use]
    pub(crate) fn diagram_fullscreen_control() -> ViewerMediaControlSpec {
        Self::diagram_control("fullscreen", "⛶")
    }

    #[must_use]
    pub(crate) fn diagram_grid_rows() -> [&'static [ViewerDiagramControlSlot]; 3] {
        [
            &DIAGRAM_GRID_TOP,
            &DIAGRAM_GRID_MIDDLE,
            &DIAGRAM_GRID_BOTTOM,
        ]
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn surface_control_svg(command: &str) -> &'static str {
        surface_control_svg(command)
    }

    #[cfg(test)]
    fn build_image_control(command: &'static str, label: &'static str) -> ViewerMediaControlSpec {
        ViewerMediaControlSpec {
            kind: ViewerMediaControlKind::Image,
            command,
            label,
            accessibility_label: command,
            icon_svg: surface_control_svg(command),
            width_px: IMAGE_CONTROL_SIZE_PX,
            height_px: IMAGE_CONTROL_SIZE_PX,
        }
    }

    fn build_code_control(command: &'static str, label: &'static str) -> ViewerMediaControlSpec {
        ViewerMediaControlSpec {
            kind: ViewerMediaControlKind::Code,
            command,
            label,
            accessibility_label: command,
            icon_svg: surface_control_svg(command),
            width_px: CODE_CONTROL_SIZE_PX,
            height_px: CODE_CONTROL_SIZE_PX,
        }
    }

    fn build_diagram_control(command: &'static str, label: &'static str) -> ViewerMediaControlSpec {
        ViewerMediaControlSpec {
            kind: ViewerMediaControlKind::Diagram,
            command,
            label,
            accessibility_label: command,
            icon_svg: surface_control_svg(command),
            width_px: SQUARE_CONTROL_SIZE_PX,
            height_px: SQUARE_CONTROL_SIZE_PX,
        }
    }
}
