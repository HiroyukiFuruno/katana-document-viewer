use super::{
    ViewerSettingsField, ViewerSettingsState, ViewerSettingsUpdate, ViewerSettingsUpdateError,
    ViewerSettingsValue, ViewerTypographyConfig,
};
use crate::viewer::types::{ViewerInteractionConfig, ViewerMode};

impl ViewerSettingsState {
    pub fn apply_update(
        &mut self,
        update: ViewerSettingsUpdate,
    ) -> Result<(), ViewerSettingsUpdateError> {
        match (update.field, update.value) {
            (field @ (ViewerSettingsField::Dark | ViewerSettingsField::Theme), value) => {
                self.apply_theme_value(field, value)?;
            }
            (ViewerSettingsField::Mode, ViewerSettingsValue::Text(value)) => {
                self.mode = mode_from_label(&value);
            }
            (ViewerSettingsField::PreviewFontSize, value) => {
                self.apply_font_size_value(value)?;
            }
            (field, ViewerSettingsValue::Bool(value)) if is_interaction_field(field) => {
                update_interaction(field, &mut self.interaction, value);
            }
            (field, value) => {
                return Err(ViewerSettingsUpdateError::InvalidValue { field, value });
            }
        }
        Ok(())
    }

    fn apply_theme_value(
        &mut self,
        field: ViewerSettingsField,
        value: ViewerSettingsValue,
    ) -> Result<(), ViewerSettingsUpdateError> {
        match value {
            ViewerSettingsValue::Bool(value) => self.dark = value,
            ViewerSettingsValue::Text(value) => self.dark = value == "dark",
            value => {
                return Err(ViewerSettingsUpdateError::InvalidValue { field, value });
            }
        }
        Ok(())
    }

    fn apply_font_size_value(
        &mut self,
        value: ViewerSettingsValue,
    ) -> Result<(), ViewerSettingsUpdateError> {
        match value {
            ViewerSettingsValue::Text(value) => {
                self.typography.preview_font_size = font_size_from_label(&value);
            }
            ViewerSettingsValue::Number(value) => {
                self.typography.preview_font_size = clamp_font_size(value);
            }
            value => {
                return Err(ViewerSettingsUpdateError::InvalidValue {
                    field: ViewerSettingsField::PreviewFontSize,
                    value,
                });
            }
        }
        Ok(())
    }
}

fn is_interaction_field(field: ViewerSettingsField) -> bool {
    matches!(
        field,
        ViewerSettingsField::Hover
            | ViewerSettingsField::Selection
            | ViewerSettingsField::ImageControls
            | ViewerSettingsField::DiagramControls
            | ViewerSettingsField::CodeControls
    )
}

fn update_interaction(
    field: ViewerSettingsField,
    interaction: &mut ViewerInteractionConfig,
    value: bool,
) {
    match field {
        ViewerSettingsField::Hover => interaction.hover_highlight_enabled = value,
        ViewerSettingsField::Selection => interaction.selection_enabled = value,
        ViewerSettingsField::ImageControls => interaction.image_controls_enabled = value,
        ViewerSettingsField::DiagramControls => interaction.diagram_controls_enabled = value,
        ViewerSettingsField::CodeControls => interaction.code_controls_enabled = value,
        ViewerSettingsField::Dark
        | ViewerSettingsField::Theme
        | ViewerSettingsField::Mode
        | ViewerSettingsField::PreviewFontSize => {}
    }
}

fn mode_from_label(value: &str) -> ViewerMode {
    match value {
        "slideshow" => ViewerMode::Slideshow,
        _ => ViewerMode::Document,
    }
}

fn font_size_from_label(value: &str) -> u16 {
    value.parse::<i64>().map_or_else(
        |_| ViewerTypographyConfig::default().preview_font_size,
        clamp_font_size,
    )
}

fn clamp_font_size(value: i64) -> u16 {
    value.clamp(12, 32) as u16
}

#[cfg(test)]
#[path = "settings_update_apply_tests.rs"]
mod tests;
