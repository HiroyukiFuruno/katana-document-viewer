use super::{ViewerDiagramControlSlot, ViewerMediaControlKind, ViewerMediaControlSpec};
use crate::viewer::media_control_spec::media_control_icons;

const DIAGRAM_CONTROL_SIZE_PX: u16 = 28;

const DIAGRAM_SLOT_FULLSCREEN: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Diagram,
        command: "fullscreen",
        label: "⛶",
        accessibility_label: "fullscreen",
        icon_svg: media_control_icons::FIT_ICON,
        width_px: DIAGRAM_CONTROL_SIZE_PX,
        height_px: DIAGRAM_CONTROL_SIZE_PX,
    });

const DIAGRAM_SLOT_PAN_UP: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Diagram,
        command: "pan-up",
        label: "↑",
        accessibility_label: "pan-up",
        icon_svg: media_control_icons::PAN_UP_ICON,
        width_px: DIAGRAM_CONTROL_SIZE_PX,
        height_px: DIAGRAM_CONTROL_SIZE_PX,
    });

const DIAGRAM_SLOT_ZOOM_IN: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Diagram,
        command: "zoom-in",
        label: "+",
        accessibility_label: "zoom-in",
        icon_svg: media_control_icons::ZOOM_IN_ICON,
        width_px: DIAGRAM_CONTROL_SIZE_PX,
        height_px: DIAGRAM_CONTROL_SIZE_PX,
    });

const DIAGRAM_SLOT_PAN_LEFT: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Diagram,
        command: "pan-left",
        label: "←",
        accessibility_label: "pan-left",
        icon_svg: media_control_icons::PAN_LEFT_ICON,
        width_px: DIAGRAM_CONTROL_SIZE_PX,
        height_px: DIAGRAM_CONTROL_SIZE_PX,
    });

const DIAGRAM_SLOT_RESET: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Diagram,
        command: "reset-view",
        label: "↻",
        accessibility_label: "reset-view",
        icon_svg: media_control_icons::RESET_VIEW_ICON,
        width_px: DIAGRAM_CONTROL_SIZE_PX,
        height_px: DIAGRAM_CONTROL_SIZE_PX,
    });

const DIAGRAM_SLOT_PAN_RIGHT: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Diagram,
        command: "pan-right",
        label: "→",
        accessibility_label: "pan-right",
        icon_svg: media_control_icons::PAN_RIGHT_ICON,
        width_px: DIAGRAM_CONTROL_SIZE_PX,
        height_px: DIAGRAM_CONTROL_SIZE_PX,
    });

const DIAGRAM_SLOT_TRACKPAD_HELP: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Diagram,
        command: "trackpad-help",
        label: "i",
        accessibility_label: "trackpad-help",
        icon_svg: media_control_icons::TRACKPAD_HELP_ICON,
        width_px: DIAGRAM_CONTROL_SIZE_PX,
        height_px: DIAGRAM_CONTROL_SIZE_PX,
    });

const DIAGRAM_SLOT_PAN_DOWN: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Diagram,
        command: "pan-down",
        label: "↓",
        accessibility_label: "pan-down",
        icon_svg: media_control_icons::PAN_DOWN_ICON,
        width_px: DIAGRAM_CONTROL_SIZE_PX,
        height_px: DIAGRAM_CONTROL_SIZE_PX,
    });

const DIAGRAM_SLOT_ZOOM_OUT: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Diagram,
        command: "zoom-out",
        label: "-",
        accessibility_label: "zoom-out",
        icon_svg: media_control_icons::ZOOM_OUT_ICON,
        width_px: DIAGRAM_CONTROL_SIZE_PX,
        height_px: DIAGRAM_CONTROL_SIZE_PX,
    });

const DIAGRAM_GAP: ViewerDiagramControlSlot = ViewerDiagramControlSlot::Gap {
    width_px: 2,
    height_px: DIAGRAM_CONTROL_SIZE_PX,
};

pub(super) const DIAGRAM_TOP_SLOTS: [ViewerDiagramControlSlot; 1] = [DIAGRAM_SLOT_FULLSCREEN];

pub(super) const DIAGRAM_GRID_TOP: [ViewerDiagramControlSlot; 5] = [
    ViewerDiagramControlSlot::Spacer {
        width_px: DIAGRAM_CONTROL_SIZE_PX,
        height_px: DIAGRAM_CONTROL_SIZE_PX,
    },
    DIAGRAM_GAP,
    DIAGRAM_SLOT_PAN_UP,
    DIAGRAM_GAP,
    DIAGRAM_SLOT_ZOOM_IN,
];

pub(super) const DIAGRAM_GRID_MIDDLE: [ViewerDiagramControlSlot; 5] = [
    DIAGRAM_SLOT_PAN_LEFT,
    DIAGRAM_GAP,
    DIAGRAM_SLOT_RESET,
    DIAGRAM_GAP,
    DIAGRAM_SLOT_PAN_RIGHT,
];

pub(super) const DIAGRAM_GRID_BOTTOM: [ViewerDiagramControlSlot; 5] = [
    DIAGRAM_SLOT_TRACKPAD_HELP,
    DIAGRAM_GAP,
    DIAGRAM_SLOT_PAN_DOWN,
    DIAGRAM_GAP,
    DIAGRAM_SLOT_ZOOM_OUT,
];
