use super::ViewerMediaControlKind;

const SQUARE_CONTROL_SIZE_PX: u16 = 28;
const IMAGE_CONTROL_SIZE_PX: u16 = SQUARE_CONTROL_SIZE_PX;
const CODE_CONTROL_SIZE_PX: u16 = SQUARE_CONTROL_SIZE_PX;
const DIAGRAM_CONTROL_GAP_PX: u16 = 2;
const IMAGE_CONTROL_GAP_PX: u16 = DIAGRAM_CONTROL_GAP_PX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewerMediaControlSpec {
    pub kind: ViewerMediaControlKind,
    pub command: &'static str,
    pub label: &'static str,
    pub accessibility_label: &'static str,
    pub icon_svg: &'static str,
    pub width_px: u16,
    pub height_px: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewerDiagramControlSlot {
    Control(ViewerMediaControlSpec),
    Gap { width_px: u16, height_px: u16 },
    Spacer { width_px: u16, height_px: u16 },
}

pub struct ViewerMediaControlSet;

impl ViewerMediaControlSet {
    #[must_use]
    pub const fn image_controls() -> &'static [ViewerMediaControlSpec] {
        &IMAGE_CONTROLS
    }

    #[must_use]
    pub const fn image_top_slots() -> &'static [ViewerDiagramControlSlot] {
        &IMAGE_TOP_SLOTS
    }

    #[must_use]
    pub const fn image_grid_rows() -> [&'static [ViewerDiagramControlSlot]; 3] {
        [&IMAGE_GRID_TOP, &IMAGE_GRID_MIDDLE, &IMAGE_GRID_BOTTOM]
    }

    #[must_use]
    pub const fn code_copy_control() -> ViewerMediaControlSpec {
        code_control("copy-code", "⧉")
    }

    #[must_use]
    pub const fn diagram_top_slots() -> &'static [ViewerDiagramControlSlot] {
        &DIAGRAM_TOP_SLOTS
    }

    #[must_use]
    pub const fn diagram_fullscreen_control() -> ViewerMediaControlSpec {
        diagram_control("fullscreen", "⛶")
    }

    #[must_use]
    pub const fn diagram_grid_rows() -> [&'static [ViewerDiagramControlSlot]; 3] {
        [
            &DIAGRAM_GRID_TOP,
            &DIAGRAM_GRID_MIDDLE,
            &DIAGRAM_GRID_BOTTOM,
        ]
    }
}

const IMAGE_CONTROLS: [ViewerMediaControlSpec; 6] = [
    image_control("fit", "⤢"),
    image_control("open", "↗"),
    image_control("copy", "⧉"),
    image_control("reveal-in-os", "…"),
    image_control("zoom-in", "+"),
    image_control("zoom-out", "-"),
];

const IMAGE_TOP_SLOTS: [ViewerDiagramControlSlot; 5] = [
    image_slot("open", "↗"),
    image_gap(),
    image_slot("copy", "⧉"),
    image_gap(),
    image_slot("reveal-in-os", "…"),
];

const IMAGE_GRID_TOP: [ViewerDiagramControlSlot; 3] =
    [image_spacer(), image_gap(), image_slot("zoom-in", "+")];

const IMAGE_GRID_MIDDLE: [ViewerDiagramControlSlot; 3] =
    [image_spacer(), image_gap(), image_slot("fit", "⤢")];

const IMAGE_GRID_BOTTOM: [ViewerDiagramControlSlot; 3] =
    [image_spacer(), image_gap(), image_slot("zoom-out", "-")];

const DIAGRAM_TOP_SLOTS: [ViewerDiagramControlSlot; 1] = [diagram_slot("fullscreen", "⛶")];

const DIAGRAM_GRID_TOP: [ViewerDiagramControlSlot; 5] = [
    spacer(),
    grid_gap(),
    diagram_slot("pan-up", "↑"),
    grid_gap(),
    diagram_slot("zoom-in", "+"),
];

const DIAGRAM_GRID_MIDDLE: [ViewerDiagramControlSlot; 5] = [
    diagram_slot("pan-left", "←"),
    grid_gap(),
    diagram_slot("reset-view", "↻"),
    grid_gap(),
    diagram_slot("pan-right", "→"),
];

const DIAGRAM_GRID_BOTTOM: [ViewerDiagramControlSlot; 5] = [
    diagram_slot("trackpad-help", "i"),
    grid_gap(),
    diagram_slot("pan-down", "↓"),
    grid_gap(),
    diagram_slot("zoom-out", "-"),
];

const fn image_control(command: &'static str, label: &'static str) -> ViewerMediaControlSpec {
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

const fn code_control(command: &'static str, label: &'static str) -> ViewerMediaControlSpec {
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

const fn diagram_slot(command: &'static str, label: &'static str) -> ViewerDiagramControlSlot {
    ViewerDiagramControlSlot::Control(diagram_control(command, label))
}

const fn diagram_control(command: &'static str, label: &'static str) -> ViewerMediaControlSpec {
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

const fn image_slot(command: &'static str, label: &'static str) -> ViewerDiagramControlSlot {
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

const fn grid_gap() -> ViewerDiagramControlSlot {
    ViewerDiagramControlSlot::Gap {
        width_px: DIAGRAM_CONTROL_GAP_PX,
        height_px: SQUARE_CONTROL_SIZE_PX,
    }
}

const fn image_gap() -> ViewerDiagramControlSlot {
    ViewerDiagramControlSlot::Gap {
        width_px: IMAGE_CONTROL_GAP_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    }
}

const fn surface_control_svg(command: &str) -> &'static str {
    match command.as_bytes() {
        b"copy" | b"copy-code" | b"copy-source" => {
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="6" y="5" width="6" height="7"/><path d="M4 10V3h7"/></svg>"#
        }
        b"fit" | b"fullscreen" => {
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M3 6V3h3M10 3h3v3M13 10v3h-3M6 13H3v-3"/></svg>"#
        }
        b"open" => {
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M6 4H4v8h8v-2M9 4h3v3M12 4 7 9"/></svg>"#
        }
        b"pan-up" => {
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M4 10 8 5l4 5"/></svg>"#
        }
        b"pan-down" => {
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="m4 6 4 5 4-5"/></svg>"#
        }
        b"pan-left" => {
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M10 4 5 8l5 4"/></svg>"#
        }
        b"pan-right" => {
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="m6 4 5 4-5 4"/></svg>"#
        }
        b"zoom-in" => {
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="7" cy="7" r="4"/><path d="M7 5v4M5 7h4M10 10l3 3"/></svg>"#
        }
        b"zoom-out" => {
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="7" cy="7" r="4"/><path d="M5 7h4M10 10l3 3"/></svg>"#
        }
        b"reset-view" => {
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 6a4 4 0 1 0 1 3M12 3v3H9"/></svg>"#
        }
        b"trackpad-help" | b"reveal-in-os" => {
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="8" cy="8" r="5"/><path d="M8 7v4M8 5h.01"/></svg>"#
        }
        _ => {
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="8" cy="8" r="4"/></svg>"#
        }
    }
}

const fn image_spacer() -> ViewerDiagramControlSlot {
    ViewerDiagramControlSlot::Spacer {
        width_px: IMAGE_CONTROL_SIZE_PX,
        height_px: IMAGE_CONTROL_SIZE_PX,
    }
}

const fn spacer() -> ViewerDiagramControlSlot {
    ViewerDiagramControlSlot::Spacer {
        width_px: SQUARE_CONTROL_SIZE_PX,
        height_px: SQUARE_CONTROL_SIZE_PX,
    }
}
