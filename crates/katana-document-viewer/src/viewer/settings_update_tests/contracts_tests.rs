use crate::{ViewerInteractionConfig, ViewerMode};
use crate::{
    ViewerSettingsField, ViewerSettingsState, ViewerSettingsUpdate, ViewerSettingsUpdateError,
    ViewerSettingsValue,
};

type SettingsResult = Result<(), ViewerSettingsUpdateError>;

#[test]
fn viewer_settings_update_displays_error_messages() {
    let invalid_field = ViewerSettingsUpdateError::UnknownField("missing".to_string());
    let invalid_value = ViewerSettingsUpdateError::InvalidValue {
        field: ViewerSettingsField::Theme,
        value: ViewerSettingsValue::Number(10),
    };

    assert_eq!(
        "unknown viewer settings field missing",
        invalid_field.to_string()
    );
    assert_eq!(
        "invalid viewer settings value Number(10) for Theme",
        invalid_value.to_string()
    );
}

#[test]
fn viewer_settings_apply_supports_all_known_field_ids_and_commands() -> SettingsResult {
    assert_field_ids();
    assert_field_ids_as_none();
    assert_field_ids_as_command();
    Ok(())
}

#[test]
fn viewer_settings_update_applies_clamped_font_size_and_unknown_type_errors() -> SettingsResult {
    let mut state = ViewerSettingsState::new(
        false,
        ViewerMode::Document,
        ViewerInteractionConfig::default(),
    );
    assert_font_size_clamps(&mut state)?;
    assert_mode_type_error(&mut state);
    Ok(())
}

fn assert_font_size_clamps(state: &mut ViewerSettingsState) -> SettingsResult {
    state.apply_update(ViewerSettingsUpdate::from_field_id(
        "preview-font-size",
        ViewerSettingsValue::Text("999".to_string()),
    )?)?;
    assert_eq!(32, state.typography.preview_font_size);
    state.apply_update(ViewerSettingsUpdate::from_field_id(
        "preview-font-size",
        ViewerSettingsValue::Text("6".to_string()),
    )?)?;
    assert_eq!(12, state.typography.preview_font_size);
    Ok(())
}

fn assert_mode_type_error(state: &mut ViewerSettingsState) {
    let number_error = state.apply_update(ViewerSettingsUpdate {
        field: ViewerSettingsField::Mode,
        value: ViewerSettingsValue::Number(3),
    });

    assert!(matches!(
        number_error,
        Err(ViewerSettingsUpdateError::InvalidValue {
            field: ViewerSettingsField::Mode,
            value: ViewerSettingsValue::Number(3),
        })
    ));
}

fn assert_field_ids() {
    for (actual, expected) in known_field_pairs() {
        assert_eq!(Some(expected), actual);
    }
}

fn known_field_pairs() -> Vec<(Option<ViewerSettingsField>, ViewerSettingsField)> {
    let mut fields = Vec::new();
    fields.extend(core_field_pairs());
    fields.extend(ui_field_pairs());
    fields
}

fn core_field_pairs() -> Vec<(Option<ViewerSettingsField>, ViewerSettingsField)> {
    vec![
        (
            ViewerSettingsField::from_id("dark"),
            ViewerSettingsField::Dark,
        ),
        (
            ViewerSettingsField::from_id("theme"),
            ViewerSettingsField::Theme,
        ),
        (
            ViewerSettingsField::from_id("mode"),
            ViewerSettingsField::Mode,
        ),
        (
            ViewerSettingsField::from_id("preview-font-size"),
            ViewerSettingsField::PreviewFontSize,
        ),
    ]
}

fn ui_field_pairs() -> Vec<(Option<ViewerSettingsField>, ViewerSettingsField)> {
    vec![
        (
            ViewerSettingsField::from_id("hover"),
            ViewerSettingsField::Hover,
        ),
        (
            ViewerSettingsField::from_id("selection"),
            ViewerSettingsField::Selection,
        ),
        (
            ViewerSettingsField::from_id("image-controls"),
            ViewerSettingsField::ImageControls,
        ),
        (
            ViewerSettingsField::from_id("diagram-controls"),
            ViewerSettingsField::DiagramControls,
        ),
        (
            ViewerSettingsField::from_id("code-controls"),
            ViewerSettingsField::CodeControls,
        ),
    ]
}

fn assert_field_ids_as_none() {
    assert_eq!(None, ViewerSettingsField::from_id("none"));
}

fn assert_field_ids_as_command() {
    assert_eq!("dark", ViewerSettingsField::Dark.id());
    assert_eq!("theme", ViewerSettingsField::Theme.id());
    assert_eq!("mode", ViewerSettingsField::Mode.id());
    assert_eq!(
        "preview-font-size",
        ViewerSettingsField::PreviewFontSize.id()
    );
    assert_eq!("hover", ViewerSettingsField::Hover.id());
    assert_eq!("selection", ViewerSettingsField::Selection.id());
    assert_eq!("image-controls", ViewerSettingsField::ImageControls.id());
    assert_eq!(
        "diagram-controls",
        ViewerSettingsField::DiagramControls.id()
    );
    assert_eq!("code-controls", ViewerSettingsField::CodeControls.id());
}
