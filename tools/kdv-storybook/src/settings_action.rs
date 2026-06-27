use crate::preview::PreviewScene;
use crate::sidebar::sidebar_settings;
use crate::sidebar_settings_state::StorybookSettingsState;
use katana_document_viewer::{
    ViewerInteractionConfig, ViewerMode, ViewerSettingsState, ViewerSettingsUpdate,
    ViewerSettingsValue, ViewerTypographyConfig,
};
use katana_ui_core::molecule::{SettingsListAction, SettingsListEvent, SettingsValue};

pub(crate) struct StorybookSettingsAction;
pub(crate) use katana_document_viewer::ViewerSettingsField as StorybookSettingsField;

pub(crate) struct StorybookSettingsApplyResult {
    pub(crate) changed: bool,
    pub(crate) field: Option<StorybookSettingsField>,
}

pub(crate) struct StorybookSettingsRequest<'a> {
    pub(crate) scene: Option<&'a PreviewScene>,
    pub(crate) dark: &'a mut bool,
    pub(crate) mode: &'a mut ViewerMode,
    pub(crate) interaction: &'a mut ViewerInteractionConfig,
    pub(crate) typography: &'a mut ViewerTypographyConfig,
    pub(crate) settings_state: &'a mut StorybookSettingsState,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl StorybookSettingsAction {
    pub(crate) fn apply_action(
        action: SettingsListAction,
        request: StorybookSettingsRequest<'_>,
    ) -> Result<StorybookSettingsApplyResult, Box<dyn std::error::Error>> {
        match action.clone() {
            SettingsListAction::UpdateField { field_id, value } => {
                let update =
                    ViewerSettingsUpdate::from_field_id(&field_id, viewer_settings_value(value)?)?;
                Self::apply_update_field(field_id, update, action, request)
            }
            SettingsListAction::ToggleSection { section_id } => {
                Self::apply_toggle_section(section_id, action, request)
            }
            _ => Err("unsupported KUC settings action".into()),
        }
    }

    pub(crate) fn apply_field(
        field: StorybookSettingsField,
        request: StorybookSettingsRequest<'_>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let list = sidebar_settings::settings_list(
            request.scene,
            *request.dark,
            request.interaction,
            *request.typography,
            request.settings_state,
            request.width,
            request.height,
        );
        let action = list
            .activation_action_for_field(field.id())
            .ok_or_else(|| format!("KUC settings has no activation action for {}", field.id()))?;
        Ok(Self::apply_action(action, request)?.changed)
    }

    fn apply_update_field(
        field_id: String,
        update: ViewerSettingsUpdate,
        action: SettingsListAction,
        request: StorybookSettingsRequest<'_>,
    ) -> Result<StorybookSettingsApplyResult, Box<dyn std::error::Error>> {
        let field = update.field;
        let mut list = sidebar_settings::settings_list(
            request.scene,
            *request.dark,
            request.interaction,
            *request.typography,
            request.settings_state,
            request.width,
            request.height,
        );
        let events = list.apply_settings_action(action);
        Self::assert_field_changed(&field_id, &events)?;
        let mut state = ViewerSettingsState::new(
            *request.dark,
            request.mode.clone(),
            request.interaction.clone(),
        );
        state.typography = *request.typography;
        state.apply_update(update)?;
        *request.dark = state.dark;
        *request.mode = state.mode;
        *request.interaction = state.interaction;
        *request.typography = state.typography;
        Ok(StorybookSettingsApplyResult {
            changed: true,
            field: Some(field),
        })
    }

    fn apply_toggle_section(
        section_id: String,
        action: SettingsListAction,
        request: StorybookSettingsRequest<'_>,
    ) -> Result<StorybookSettingsApplyResult, Box<dyn std::error::Error>> {
        let mut list = sidebar_settings::settings_list(
            request.scene,
            *request.dark,
            request.interaction,
            *request.typography,
            request.settings_state,
            request.width,
            request.height,
        );
        let events = list.apply_settings_action(action);
        Self::assert_section_toggled(&section_id, &events)?;
        request.settings_state.apply_events(&events);
        Ok(StorybookSettingsApplyResult {
            changed: true,
            field: None,
        })
    }

    fn assert_field_changed(
        expected_field_id: &str,
        events: &[SettingsListEvent],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let changed = events.iter().any(|event| {
            matches!(event, SettingsListEvent::FieldChanged { field_id } if field_id == expected_field_id)
        });
        if changed {
            return Ok(());
        }
        Err(format!("KUC settings action did not change {expected_field_id}").into())
    }

    fn assert_section_toggled(
        expected_section_id: &str,
        events: &[SettingsListEvent],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let changed = events.iter().any(|event| {
            matches!(
                event,
                SettingsListEvent::SectionCollapsed { section_id, .. }
                    if section_id == expected_section_id
            )
        });
        if changed {
            return Ok(());
        }
        Err(format!("KUC settings action did not toggle {expected_section_id}").into())
    }
}

fn viewer_settings_value(
    value: SettingsValue,
) -> Result<ViewerSettingsValue, Box<dyn std::error::Error>> {
    match value {
        SettingsValue::Bool(value) => Ok(ViewerSettingsValue::Bool(value)),
        SettingsValue::Text(value) => Ok(ViewerSettingsValue::Text(value)),
        SettingsValue::Number(value) => Ok(ViewerSettingsValue::Number(value)),
        SettingsValue::Color { .. } | SettingsValue::List(_) | SettingsValue::None => {
            Err("unsupported KUC settings value".into())
        }
    }
}

#[cfg(test)]
#[path = "settings_action_tests.rs"]
mod tests;
