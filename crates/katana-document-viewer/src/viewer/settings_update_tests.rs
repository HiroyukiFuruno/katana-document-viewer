use super::{
    ViewerSettingsField, ViewerSettingsState, ViewerSettingsUpdate, ViewerSettingsUpdateError,
    ViewerSettingsValue,
};
use crate::{ViewerInteractionConfig, ViewerMode};

#[test]
fn viewer_settings_update_applies_theme_and_mode() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = ViewerSettingsState::new(
        true,
        ViewerMode::Document,
        ViewerInteractionConfig::default(),
    );

    state.apply_update(ViewerSettingsUpdate::from_field_id(
        "theme",
        ViewerSettingsValue::Text("light".to_string()),
    )?)?;
    state.apply_update(ViewerSettingsUpdate::from_field_id(
        "mode",
        ViewerSettingsValue::Text("slideshow".to_string()),
    )?)?;

    assert!(!state.dark);
    assert_eq!(ViewerMode::Slideshow, state.mode);
    Ok(())
}

#[test]
fn viewer_settings_update_applies_interaction_flags() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = ViewerSettingsState::new(
        false,
        ViewerMode::Document,
        ViewerInteractionConfig::default(),
    );

    state.apply_update(ViewerSettingsUpdate {
        field: ViewerSettingsField::DiagramControls,
        value: ViewerSettingsValue::Bool(false),
    })?;
    state.apply_update(ViewerSettingsUpdate::from_field_id(
        "code-controls",
        ViewerSettingsValue::Bool(false),
    )?)?;

    assert!(!state.interaction.diagram_controls_enabled);
    assert!(!state.interaction.code_controls_enabled);
    Ok(())
}

#[test]
fn viewer_settings_update_applies_preview_font_size() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = ViewerSettingsState::new(
        false,
        ViewerMode::Document,
        ViewerInteractionConfig::default(),
    );

    state.apply_update(ViewerSettingsUpdate::from_field_id(
        "preview-font-size",
        ViewerSettingsValue::Text("20".to_string()),
    )?)?;

    assert_eq!(20, state.typography.preview_font_size);
    Ok(())
}

#[test]
fn viewer_settings_update_rejects_unknown_field() -> Result<(), String> {
    let error =
        ViewerSettingsUpdate::from_field_id("storybook-only", ViewerSettingsValue::Bool(true))
            .err()
            .ok_or_else(|| "unknown field must fail fast".to_string())?;

    assert_eq!(
        ViewerSettingsUpdateError::UnknownField("storybook-only".to_string()),
        error
    );
    Ok(())
}

#[test]
fn viewer_settings_update_rejects_wrong_value_type() -> Result<(), String> {
    let mut state = ViewerSettingsState::new(
        false,
        ViewerMode::Document,
        ViewerInteractionConfig::default(),
    );

    let error = state
        .apply_update(ViewerSettingsUpdate {
            field: ViewerSettingsField::DiagramControls,
            value: ViewerSettingsValue::Text("off".to_string()),
        })
        .err()
        .ok_or_else(|| "wrong value type must fail fast".to_string())?;

    assert!(matches!(
        error,
        ViewerSettingsUpdateError::InvalidValue {
            field: ViewerSettingsField::DiagramControls,
            value: ViewerSettingsValue::Text(_)
        }
    ));
    Ok(())
}
