use crate::{ViewerInteractionConfig, ViewerMode};
use crate::{
    ViewerSettingsField, ViewerSettingsState, ViewerSettingsUpdate, ViewerSettingsUpdateError,
    ViewerSettingsValue,
};

#[test]
fn viewer_settings_update_rejects_invalid_theme_and_font_types() {
    let mut state = ViewerSettingsState::new(
        false,
        ViewerMode::Document,
        ViewerInteractionConfig::default(),
    );

    for update in [
        ViewerSettingsUpdate {
            field: ViewerSettingsField::Theme,
            value: ViewerSettingsValue::Number(1),
        },
        ViewerSettingsUpdate {
            field: ViewerSettingsField::PreviewFontSize,
            value: ViewerSettingsValue::Bool(true),
        },
    ] {
        assert!(matches!(
            state.apply_update(update),
            Err(ViewerSettingsUpdateError::InvalidValue { .. })
        ));
    }
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

#[test]
fn viewer_settings_update_rejects_unknown_mode_value_type() -> Result<(), String> {
    let mut state = ViewerSettingsState::new(
        false,
        ViewerMode::Document,
        ViewerInteractionConfig::default(),
    );

    let error = state
        .apply_update(ViewerSettingsUpdate {
            field: ViewerSettingsField::Mode,
            value: ViewerSettingsValue::Bool(true),
        })
        .err()
        .ok_or_else(|| "mode must require text input".to_string())?;

    assert!(matches!(
        error,
        ViewerSettingsUpdateError::InvalidValue {
            field: ViewerSettingsField::Mode,
            value: ViewerSettingsValue::Bool(_)
        }
    ));
    Ok(())
}
