use katana_ui_core::render_model::{UiIconProps, UiSvgPaintPolicy};
use std::collections::BTreeMap;

const KATANA_STROKE_ICON_VIEW_BOX: &str = "0 0 16 16";
const KATANA_MATERIAL_ICON_VIEW_BOX: &str = "0 -960 960 960";
const COLOR_TOKEN: &str = "text";
const THEME_TOKEN: &str = "text";

const KATANA_COPY: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="8" height="10" rx="1"/><rect x="5" y="1" width="8" height="10" rx="1"/></svg>"##;
const KATANA_CLOSE_MODAL: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><line x1="3" y1="3" x2="13" y2="13"/><line x1="13" y1="3" x2="3" y2="13"/></svg>"##;
const KATANA_EXTERNAL_LINK: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M10 2 h4 v4"/><line x1="14" y1="2" x2="7" y2="9"/><path d="M14 9 v4 a1 1 0 0 1-1 1 H3 a1 1 0 0 1-1-1 V3 a1 1 0 0 1 1-1 h4"/></svg>"##;
const KATANA_FULLSCREEN: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="2 6 2 2 6 2"/><polyline points="10 2 14 2 14 6"/><polyline points="14 10 14 14 10 14"/><polyline points="6 14 2 14 2 10"/></svg>"##;
const KATANA_INFO: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="8" cy="8" r="6"/><line x1="8" y1="7" x2="8" y2="12"/><circle cx="8" cy="4.5" r="0.5" fill="#FFFFFF" stroke="none"/></svg>"##;
const KATANA_PAN_DOWN: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 6 8 12 12 6"/></svg>"##;
const KATANA_PAN_LEFT: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="10 4 4 8 10 12"/></svg>"##;
const KATANA_PAN_RIGHT: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 4 12 8 6 12"/></svg>"##;
const KATANA_PAN_UP: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 10 8 4 12 10"/></svg>"##;
const KATANA_RESET_VIEW: &str = r##"<svg fill="#FFFFFF" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"><path d="M480-80q-75 0-140.5-28.5t-114-77q-48.5-48.5-77-114T120-440h80q0 117 81.5 198.5T480-160q117 0 198.5-81.5T760-440q0-117-81.5-198.5T480-720h-6l62 62-56 58-160-160 160-160 56 58-62 62h6q75 0 140.5 28.5t114 77q48.5 48.5 77 114T840-440q0 75-28.5 140.5t-77 114q-48.5 48.5-114 77T480-80Z"/></svg>"##;
const KATANA_ZOOM_IN: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="7" cy="7" r="4"/><line x1="10" y1="10" x2="14" y2="14"/><line x1="7" y1="5" x2="7" y2="9"/><line x1="5" y1="7" x2="9" y2="7"/></svg>"##;
const KATANA_ZOOM_OUT: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="7" cy="7" r="4"/><line x1="10" y1="10" x2="14" y2="14"/><line x1="5" y1="7" x2="9" y2="7"/></svg>"##;

const KATANA_DEFAULT_ICON_ASSETS: &[(&str, &str, &str, &str)] = &[
    (
        "close-modal",
        KATANA_CLOSE_MODAL,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.ui.close_modal",
    ),
    (
        "copy",
        KATANA_COPY,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.ui.copy",
    ),
    (
        "copy-code",
        KATANA_COPY,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.ui.copy",
    ),
    (
        "copy-source",
        KATANA_COPY,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.ui.copy",
    ),
    (
        "fit",
        KATANA_FULLSCREEN,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.view.fullscreen",
    ),
    (
        "fullscreen",
        KATANA_FULLSCREEN,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.view.fullscreen",
    ),
    (
        "open",
        KATANA_EXTERNAL_LINK,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.system.external_link",
    ),
    (
        "pan-down",
        KATANA_PAN_DOWN,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.view.pan_down",
    ),
    (
        "pan-left",
        KATANA_PAN_LEFT,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.view.pan_left",
    ),
    (
        "pan-right",
        KATANA_PAN_RIGHT,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.view.pan_right",
    ),
    (
        "pan-up",
        KATANA_PAN_UP,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.view.pan_up",
    ),
    (
        "reset-view",
        KATANA_RESET_VIEW,
        KATANA_MATERIAL_ICON_VIEW_BOX,
        "katana.view.reset_view",
    ),
    (
        "reveal-in-os",
        KATANA_EXTERNAL_LINK,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.system.external_link",
    ),
    (
        "trackpad-help",
        KATANA_INFO,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.status.info",
    ),
    (
        "zoom-in",
        KATANA_ZOOM_IN,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.view.zoom_in",
    ),
    (
        "zoom-out",
        KATANA_ZOOM_OUT,
        KATANA_STROKE_ICON_VIEW_BOX,
        "katana.view.zoom_out",
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucMediaControlIconSet {
    icons: BTreeMap<String, UiIconProps>,
}

impl Default for KucMediaControlIconSet {
    fn default() -> Self {
        Self::katana_default()
    }
}

impl KucMediaControlIconSet {
    #[must_use]
    pub fn katana_default() -> Self {
        let mut icons = BTreeMap::new();
        for (command, svg, view_box, summary) in KATANA_DEFAULT_ICON_ASSETS.iter().copied() {
            icons.insert(
                command.to_string(),
                Self::katana_icon(command, svg, view_box, summary),
            );
        }
        Self { icons }
    }

    #[cfg(test)]
    pub(crate) fn katana_default_asset_sources()
    -> impl Iterator<Item = (&'static str, &'static str, &'static str)> {
        KATANA_DEFAULT_ICON_ASSETS
            .iter()
            .map(|(command, svg, _, summary)| (*command, *svg, *summary))
    }

    #[must_use]
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "KDV keeps a per-command icon override hook while the default Storybook binary uses the Katana preset."
        )
    )]
    pub fn with_icon(mut self, command: impl Into<String>, icon: UiIconProps) -> Self {
        self.icons.insert(command.into(), icon);
        self
    }

    #[must_use]
    pub fn icon_for(&self, command: &str, fallback_svg: &str) -> UiIconProps {
        self.icons
            .get(command)
            .cloned()
            .unwrap_or_else(|| Self::fallback_icon(command, fallback_svg))
    }

    fn katana_icon(command: &str, svg: &str, view_box: &str, summary: &str) -> UiIconProps {
        UiIconProps::new(svg)
            .role(format!("surface.{command}"))
            .view_box(view_box)
            .path_summary(summary)
            .color_token(COLOR_TOKEN)
            .theme_token(THEME_TOKEN)
            .paint_policy(UiSvgPaintPolicy::CurrentColor)
    }

    fn fallback_icon(command: &str, fallback_svg: &str) -> UiIconProps {
        UiIconProps::new(fallback_svg)
            .role(format!("surface.{command}"))
            .view_box("0 0 16 16")
            .path_summary(command)
            .color_token(COLOR_TOKEN)
            .theme_token(THEME_TOKEN)
            .paint_policy(UiSvgPaintPolicy::StrokeOnly)
    }
}
