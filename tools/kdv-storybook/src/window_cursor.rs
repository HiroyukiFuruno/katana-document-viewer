use katana_ui_core::render_model::UiCursor;
use minifb::{CursorStyle, Window};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum StorybookCursorStyle {
    Arrow,
    Ibeam,
    ResizeAll,
    OpenHand,
    PointingHand,
}

pub(super) fn apply_cursor_style(window: &mut Window, cursor: UiCursor) {
    let style = StorybookCursorStyle::from_ui_cursor(cursor);
    window.set_cursor_style(style.fallback_minifb_cursor());
    if style == StorybookCursorStyle::PointingHand {
        platform_pointing_hand_cursor();
    }
}

impl StorybookCursorStyle {
    pub(super) fn from_ui_cursor(cursor: UiCursor) -> Self {
        match cursor {
            UiCursor::Pointer => Self::PointingHand,
            UiCursor::Text => Self::Ibeam,
            UiCursor::Move | UiCursor::Grab => Self::OpenHand,
            UiCursor::Resize => Self::ResizeAll,
            UiCursor::Default => Self::Arrow,
        }
    }

    fn fallback_minifb_cursor(self) -> CursorStyle {
        match self {
            Self::Arrow => CursorStyle::Arrow,
            Self::Ibeam => CursorStyle::Ibeam,
            Self::ResizeAll => CursorStyle::ResizeAll,
            Self::OpenHand | Self::PointingHand => CursorStyle::OpenHand,
        }
    }
}

#[cfg(target_os = "macos")]
fn platform_pointing_hand_cursor() {
    macos::set_pointing_hand_cursor();
}

#[cfg(not(target_os = "macos"))]
fn platform_pointing_hand_cursor() {}

#[cfg(target_os = "macos")]
mod macos {
    use crate::window_macos_objc;

    const NS_CURSOR_CLASS: &[u8] = b"NSCursor\0";
    const POINTING_HAND_CURSOR_SELECTOR: &[u8] = b"pointingHandCursor\0";
    const SET_SELECTOR: &[u8] = b"set\0";

    pub(super) fn set_pointing_hand_cursor() {
        let class = window_macos_objc::get_class(NS_CURSOR_CLASS);
        let selector = window_macos_objc::selector(POINTING_HAND_CURSOR_SELECTOR);
        let cursor = window_macos_objc::msg_send_id(class, selector);
        let set_selector = window_macos_objc::selector(SET_SELECTOR);
        let _ = window_macos_objc::msg_send_id(cursor, set_selector);
    }
}
