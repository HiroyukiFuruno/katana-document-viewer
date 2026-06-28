use crate::preview::PreviewScene;
use crate::sidebar_settings_state::StorybookSettingsState;
use crate::sidebar_settings_stats::{mode_label, scene_stats, slide_label};
use katana_document_viewer::{ViewerInteractionConfig, ViewerTypographyConfig};
use katana_ui_core::molecule::{
    SettingsControl, SettingsControlOption, SettingsField, SettingsList, SettingsListDensity,
    SettingsSection,
};

pub(crate) fn settings_list(
    scene: Option<&PreviewScene>,
    dark: bool,
    interaction: &ViewerInteractionConfig,
    typography: ViewerTypographyConfig,
    settings_state: &StorybookSettingsState,
    width: usize,
    height: usize,
) -> SettingsList {
    let stats = scene_stats(scene);
    let mut list = SettingsList::new("KDV settings")
        .density(SettingsListDensity::Compact)
        .section(
            SettingsSection::new("display", "Display")
                .collapsible(true)
                .field(toggle_field("dark", "Dark", dark))
                .field(SettingsField::new(
                    "theme",
                    "Theme",
                    SettingsControl::Select {
                        options: theme_options(),
                        selected: theme_label(dark).to_string(),
                    },
                ))
                .field(SettingsField::new(
                    "mode",
                    "Mode",
                    SettingsControl::Select {
                        options: mode_options(),
                        selected: mode_label(&stats.mode).to_string(),
                    },
                ))
                .field(SettingsField::new(
                    "preview-font-size",
                    "Preview font",
                    SettingsControl::Select {
                        options: preview_font_options(),
                        selected: typography.preview_font_size.to_string(),
                    },
                ))
                .field(readonly_text(
                    "viewport",
                    "Viewport",
                    format!("{width}x{height}"),
                )),
        )
        .section(
            SettingsSection::new("interaction", "Interaction")
                .collapsible(true)
                .field(toggle_field(
                    "hover",
                    "Hover highlight",
                    interaction.hover_highlight_enabled,
                ))
                .field(toggle_field(
                    "selection",
                    "Selection",
                    interaction.selection_enabled,
                ))
                .field(toggle_field(
                    "image-controls",
                    "Image controls",
                    interaction.image_controls_enabled,
                ))
                .field(toggle_field(
                    "diagram-controls",
                    "Diagram controls",
                    interaction.diagram_controls_enabled,
                ))
                .field(toggle_field(
                    "code-controls",
                    "Code controls",
                    interaction.code_controls_enabled,
                )),
        )
        .section(
            SettingsSection::new("state", "State")
                .collapsible(true)
                .field(readonly_text("slide", "Slide", slide_label(&stats)))
                .field(readonly_number(
                    "scene-font",
                    "Scene font",
                    stats.scene_font,
                ))
                .field(readonly_number("nodes", "Nodes", stats.nodes))
                .field(readonly_number("loaded", "Loaded assets", stats.loaded))
                .field(readonly_number("failed", "Failed assets", stats.failed))
                .field(readonly_number("images", "Image surfaces", stats.images))
                .field(readonly_number(
                    "task-changes",
                    "Task changes",
                    settings_state.task_change_count() as i64,
                ))
                .field(readonly_text(
                    "last-task-change",
                    "Last task",
                    settings_state.last_task_change_label(),
                ))
                .field(readonly_text(
                    "last-task-target",
                    "Changed target",
                    settings_state.last_task_target_label(),
                ))
                .field(readonly_text(
                    "last-task-span",
                    "Changed span",
                    settings_state.last_task_span_label(),
                ))
                .field(readonly_text(
                    "last-task-location",
                    "Changed location",
                    settings_state.last_task_location_label(),
                ))
                .field(readonly_text(
                    "task-location-history",
                    "Changed history",
                    settings_state.recent_task_location_history_label(),
                ))
                .field(readonly_text(
                    "last-task-source",
                    "Task source",
                    settings_state.last_task_source_label(),
                ))
                .field(readonly_text("surface", "Surface", stats.surface_label)),
        );
    settings_state.apply_to_list(&mut list);
    list
}

fn toggle_field(id: &str, label: &str, checked: bool) -> SettingsField {
    SettingsField::new(id, label, SettingsControl::Toggle { checked })
}

fn readonly_number(id: &str, label: &str, value: i64) -> SettingsField {
    SettingsField::new(
        id,
        label,
        SettingsControl::Number {
            value,
            min: 0,
            max: i64::MAX,
        },
    )
}

fn readonly_text(id: &str, label: &str, value: impl Into<String>) -> SettingsField {
    SettingsField::new(
        id,
        label,
        SettingsControl::Input {
            value: value.into(),
        },
    )
}

fn theme_options() -> Vec<SettingsControlOption> {
    vec![
        SettingsControlOption::new("dark", "Dark"),
        SettingsControlOption::new("light", "Light"),
    ]
}

fn mode_options() -> Vec<SettingsControlOption> {
    vec![
        SettingsControlOption::new("document", "Document"),
        SettingsControlOption::new("slideshow", "Slideshow"),
    ]
}

fn preview_font_options() -> Vec<SettingsControlOption> {
    vec![
        SettingsControlOption::new("14", "14"),
        SettingsControlOption::new("16", "16"),
        SettingsControlOption::new("18", "18"),
        SettingsControlOption::new("20", "20"),
        SettingsControlOption::new("22", "22"),
        SettingsControlOption::new("24", "24"),
    ]
}

fn theme_label(dark: bool) -> &'static str {
    if dark { "dark" } else { "light" }
}
