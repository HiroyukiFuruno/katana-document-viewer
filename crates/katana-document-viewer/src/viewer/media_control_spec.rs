#[path = "media_control_icons.rs"]
mod media_control_icons;
#[path = "media_control_layout.rs"]
mod media_control_layout;

use super::ViewerMediaControlKind;

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
    pub fn image_controls() -> &'static [ViewerMediaControlSpec] {
        media_control_layout::ViewerMediaControlLayout::image_controls()
    }

    #[must_use]
    pub fn image_top_slots() -> &'static [ViewerDiagramControlSlot] {
        media_control_layout::ViewerMediaControlLayout::image_top_slots()
    }

    #[must_use]
    pub fn image_grid_rows() -> [&'static [ViewerDiagramControlSlot]; 3] {
        media_control_layout::ViewerMediaControlLayout::image_grid_rows()
    }

    #[must_use]
    pub fn code_copy_control() -> ViewerMediaControlSpec {
        media_control_layout::ViewerMediaControlLayout::code_copy_control()
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn image_control(
        command: &'static str,
        label: &'static str,
    ) -> ViewerMediaControlSpec {
        media_control_layout::ViewerMediaControlLayout::image_control(command, label)
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn code_control(
        command: &'static str,
        label: &'static str,
    ) -> ViewerMediaControlSpec {
        media_control_layout::ViewerMediaControlLayout::code_control(command, label)
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn diagram_slot(
        command: &'static str,
        label: &'static str,
    ) -> ViewerDiagramControlSlot {
        media_control_layout::ViewerMediaControlLayout::diagram_slot(command, label)
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn diagram_control(
        command: &'static str,
        label: &'static str,
    ) -> ViewerMediaControlSpec {
        media_control_layout::ViewerMediaControlLayout::diagram_control(command, label)
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn image_slot(
        command: &'static str,
        label: &'static str,
    ) -> ViewerDiagramControlSlot {
        media_control_layout::ViewerMediaControlLayout::image_slot(command, label)
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn grid_gap() -> ViewerDiagramControlSlot {
        media_control_layout::ViewerMediaControlLayout::grid_gap()
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn image_gap() -> ViewerDiagramControlSlot {
        media_control_layout::ViewerMediaControlLayout::image_gap()
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn image_spacer() -> ViewerDiagramControlSlot {
        media_control_layout::ViewerMediaControlLayout::image_spacer()
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn spacer() -> ViewerDiagramControlSlot {
        media_control_layout::ViewerMediaControlLayout::spacer()
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn surface_control_svg(command: &str) -> &'static str {
        media_control_layout::ViewerMediaControlLayout::surface_control_svg(command)
    }

    #[must_use]
    pub fn diagram_top_slots() -> &'static [ViewerDiagramControlSlot] {
        media_control_layout::ViewerMediaControlLayout::diagram_top_slots()
    }

    #[must_use]
    pub fn diagram_fullscreen_control() -> ViewerMediaControlSpec {
        media_control_layout::ViewerMediaControlLayout::diagram_fullscreen_control()
    }

    #[must_use]
    pub fn diagram_grid_rows() -> [&'static [ViewerDiagramControlSlot]; 3] {
        media_control_layout::ViewerMediaControlLayout::diagram_grid_rows()
    }
}
