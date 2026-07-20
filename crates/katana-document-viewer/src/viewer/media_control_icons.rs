pub(super) fn surface_control_svg(command: &str) -> &'static str {
    match command {
        "copy" | "copy-code" | "copy-source" => COPY_ICON,
        "fit" | "fullscreen" => FIT_ICON,
        "open" => OPEN_ICON,
        "pan-up" => PAN_UP_ICON,
        "pan-down" => PAN_DOWN_ICON,
        "pan-left" => PAN_LEFT_ICON,
        "pan-right" => PAN_RIGHT_ICON,
        "zoom-in" => ZOOM_IN_ICON,
        "zoom-out" => ZOOM_OUT_ICON,
        "reset-view" => RESET_VIEW_ICON,
        "trackpad-help" | "reveal-in-os" => TRACKPAD_HELP_ICON,
        _ => DEFAULT_ICON,
    }
}

pub(crate) const COPY_ICON: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="6" y="5" width="6" height="7"/><path d="M4 10V3h7"/></svg>"#;
pub(crate) const FIT_ICON: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M3 6V3h3M10 3h3v3M13 10v3h-3M6 13H3v-3"/></svg>"#;
pub(crate) const OPEN_ICON: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M6 4H4v8h8v-2M9 4h3v3M12 4 7 9"/></svg>"#;
pub(crate) const PAN_UP_ICON: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M4 10 8 5l4 5"/></svg>"#;
pub(crate) const PAN_DOWN_ICON: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="m4 6 4 5 4-5"/></svg>"#;
pub(crate) const PAN_LEFT_ICON: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M10 4 5 8l5 4"/></svg>"#;
pub(crate) const PAN_RIGHT_ICON: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="m6 4 5 4-5 4"/></svg>"#;
pub(crate) const ZOOM_IN_ICON: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="7" cy="7" r="4"/><path d="M7 5v4M5 7h4M10 10l3 3"/></svg>"#;
pub(crate) const ZOOM_OUT_ICON: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="7" cy="7" r="4"/><path d="M5 7h4M10 10l3 3"/></svg>"#;
pub(crate) const RESET_VIEW_ICON: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 6a4 4 0 1 0 1 3M12 3v3H9"/></svg>"#;
pub(crate) const TRACKPAD_HELP_ICON: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="8" cy="8" r="5"/><path d="M8 7v4M8 5h.01"/></svg>"#;
pub(crate) const DEFAULT_ICON: &str = r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="8" cy="8" r="4"/></svg>"#;
