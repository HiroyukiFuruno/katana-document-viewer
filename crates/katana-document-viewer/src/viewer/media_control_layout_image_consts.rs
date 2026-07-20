use super::{ViewerDiagramControlSlot, ViewerMediaControlKind, ViewerMediaControlSpec};
use crate::viewer::media_control_spec::media_control_icons;

const IMAGE_CONTROL_SIZE_PX: u16 = 28;

pub(super) const IMAGE_CONTROLS: [ViewerMediaControlSpec; 6] = [
    ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Image,
        command: "fit",
        label: "⤢",
        accessibility_label: "fit",
        icon_svg: media_control_icons::FIT_ICON,
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    },
    ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Image,
        command: "open",
        label: "↗",
        accessibility_label: "open",
        icon_svg: media_control_icons::OPEN_ICON,
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    },
    ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Image,
        command: "copy",
        label: "⧉",
        accessibility_label: "copy",
        icon_svg: media_control_icons::COPY_ICON,
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    },
    ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Image,
        command: "reveal-in-os",
        label: "…",
        accessibility_label: "reveal-in-os",
        icon_svg: media_control_icons::TRACKPAD_HELP_ICON,
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    },
    ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Image,
        command: "zoom-in",
        label: "+",
        accessibility_label: "zoom-in",
        icon_svg: media_control_icons::ZOOM_IN_ICON,
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    },
    ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Image,
        command: "zoom-out",
        label: "-",
        accessibility_label: "zoom-out",
        icon_svg: media_control_icons::ZOOM_OUT_ICON,
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    },
];

const IMAGE_SLOT_OPEN: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Image,
        command: "open",
        label: "↗",
        accessibility_label: "open",
        icon_svg: media_control_icons::OPEN_ICON,
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    });

const IMAGE_SLOT_COPY: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Image,
        command: "copy",
        label: "⧉",
        accessibility_label: "copy",
        icon_svg: media_control_icons::COPY_ICON,
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    });

const IMAGE_SLOT_REVEAL: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Image,
        command: "reveal-in-os",
        label: "…",
        accessibility_label: "reveal-in-os",
        icon_svg: media_control_icons::TRACKPAD_HELP_ICON,
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    });

const IMAGE_SLOT_ZOOM_IN: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Image,
        command: "zoom-in",
        label: "+",
        accessibility_label: "zoom-in",
        icon_svg: media_control_icons::ZOOM_IN_ICON,
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    });

const IMAGE_SLOT_ZOOM_OUT: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Image,
        command: "zoom-out",
        label: "-",
        accessibility_label: "zoom-out",
        icon_svg: media_control_icons::ZOOM_OUT_ICON,
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    });

const IMAGE_SLOT_FIT: ViewerDiagramControlSlot =
    ViewerDiagramControlSlot::Control(ViewerMediaControlSpec {
        kind: ViewerMediaControlKind::Image,
        command: "fit",
        label: "⤢",
        accessibility_label: "fit",
        icon_svg: media_control_icons::FIT_ICON,
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    });

const IMAGE_GAP: ViewerDiagramControlSlot = ViewerDiagramControlSlot::Gap {
    width_px: 2,
    height_px: IMAGE_CONTROL_SIZE_PX,
};

pub(super) const IMAGE_TOP_SLOTS: [ViewerDiagramControlSlot; 5] = [
    IMAGE_SLOT_OPEN,
    IMAGE_GAP,
    IMAGE_SLOT_COPY,
    IMAGE_GAP,
    IMAGE_SLOT_REVEAL,
];

pub(super) const IMAGE_GRID_TOP: [ViewerDiagramControlSlot; 3] = [
    ViewerDiagramControlSlot::Spacer {
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    },
    IMAGE_GAP,
    IMAGE_SLOT_ZOOM_IN,
];

pub(super) const IMAGE_GRID_MIDDLE: [ViewerDiagramControlSlot; 3] = [
    ViewerDiagramControlSlot::Spacer {
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    },
    IMAGE_GAP,
    IMAGE_SLOT_FIT,
];

pub(super) const IMAGE_GRID_BOTTOM: [ViewerDiagramControlSlot; 3] = [
    ViewerDiagramControlSlot::Spacer {
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    },
    IMAGE_GAP,
    IMAGE_SLOT_ZOOM_OUT,
];
