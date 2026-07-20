use crate::{ViewerInteractionConfig, ViewerMode};
use crate::{ViewerSettingsField, ViewerSettingsState, ViewerSettingsUpdate, ViewerSettingsValue};

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

    disable_interaction_flags(&mut state)?;

    assert!(!state.interaction.hover_highlight_enabled);
    assert!(!state.interaction.selection_enabled);
    assert!(!state.interaction.image_controls_enabled);
    assert!(!state.interaction.diagram_controls_enabled);
    assert!(!state.interaction.code_controls_enabled);
    Ok(())
}

fn disable_interaction_flags(
    state: &mut ViewerSettingsState,
) -> Result<(), Box<dyn std::error::Error>> {
    for update in [
        ViewerSettingsUpdate {
            field: ViewerSettingsField::Hover,
            value: ViewerSettingsValue::Bool(false),
        },
        ViewerSettingsUpdate {
            field: ViewerSettingsField::Selection,
            value: ViewerSettingsValue::Bool(false),
        },
        ViewerSettingsUpdate {
            field: ViewerSettingsField::ImageControls,
            value: ViewerSettingsValue::Bool(false),
        },
        ViewerSettingsUpdate {
            field: ViewerSettingsField::DiagramControls,
            value: ViewerSettingsValue::Bool(false),
        },
    ] {
        state.apply_update(update)?;
    }

    state.apply_update(ViewerSettingsUpdate::from_field_id(
        "code-controls",
        ViewerSettingsValue::Bool(false),
    )?)?;
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
fn viewer_settings_update_applies_dark_theme_as_bool() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = ViewerSettingsState::new(
        true,
        ViewerMode::Document,
        ViewerInteractionConfig::default(),
    );

    state.apply_update(ViewerSettingsUpdate {
        field: ViewerSettingsField::Dark,
        value: ViewerSettingsValue::Bool(false),
    })?;
    assert!(!state.dark);

    state.apply_update(ViewerSettingsUpdate {
        field: ViewerSettingsField::Theme,
        value: ViewerSettingsValue::Bool(true),
    })?;

    assert!(state.dark);
    Ok(())
}

#[test]
fn viewer_settings_update_accepts_preview_font_size_from_number()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = ViewerSettingsState::new(
        false,
        ViewerMode::Document,
        ViewerInteractionConfig::default(),
    );

    state.apply_update(ViewerSettingsUpdate {
        field: ViewerSettingsField::PreviewFontSize,
        value: ViewerSettingsValue::Number(18),
    })?;

    assert_eq!(18, state.typography.preview_font_size);
    Ok(())
}

#[test]
fn viewer_settings_update_rejects_font_size_non_numeric_text()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = ViewerSettingsState::new(
        false,
        ViewerMode::Document,
        ViewerInteractionConfig::default(),
    );

    state.apply_update(ViewerSettingsUpdate {
        field: ViewerSettingsField::PreviewFontSize,
        value: ViewerSettingsValue::Text("not-number".to_string()),
    })?;

    assert_eq!(14, state.typography.preview_font_size);
    Ok(())
}

#[test]
fn viewer_settings_update_handles_unknown_mode_label_as_document()
-> Result<(), Box<dyn std::error::Error>> {
    let mut state = ViewerSettingsState::new(
        false,
        ViewerMode::Slideshow,
        ViewerInteractionConfig::default(),
    );

    state.apply_update(ViewerSettingsUpdate::from_field_id(
        "mode",
        ViewerSettingsValue::Text("invalid-mode".to_string()),
    )?)?;

    assert_eq!(ViewerMode::Document, state.mode);
    Ok(())
}
