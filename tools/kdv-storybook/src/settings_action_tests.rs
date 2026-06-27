use super::{StorybookSettingsAction, StorybookSettingsField, StorybookSettingsRequest};
use crate::sidebar_settings_state::StorybookSettingsState;
use katana_document_viewer::{ViewerInteractionConfig, ViewerMode, ViewerTypographyConfig};
use katana_ui_core::molecule::SettingsListAction;

#[test]
fn toggle_dark_uses_kuc_settings_action_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut dark = true;
    let mut mode = ViewerMode::Document;
    let mut interaction = ViewerInteractionConfig::default();
    let mut typography = ViewerTypographyConfig::default();
    let mut settings_state = StorybookSettingsState::default();

    StorybookSettingsAction::apply_field(
        StorybookSettingsField::Dark,
        request(
            &mut dark,
            &mut mode,
            &mut interaction,
            &mut typography,
            &mut settings_state,
        ),
    )?;

    assert!(!dark);
    Ok(())
}

#[test]
fn select_mode_uses_kuc_settings_action_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut dark = true;
    let mut mode = ViewerMode::Document;
    let mut interaction = ViewerInteractionConfig::default();
    let mut typography = ViewerTypographyConfig::default();
    let mut settings_state = StorybookSettingsState::default();

    StorybookSettingsAction::apply_field(
        StorybookSettingsField::Mode,
        request(
            &mut dark,
            &mut mode,
            &mut interaction,
            &mut typography,
            &mut settings_state,
        ),
    )?;

    assert_eq!(ViewerMode::Slideshow, mode);
    Ok(())
}

#[test]
fn interaction_toggles_use_kuc_settings_action_result() -> Result<(), Box<dyn std::error::Error>> {
    let cases = [
        (
            StorybookSettingsField::Hover,
            is_hover_highlight_enabled as fn(&ViewerInteractionConfig) -> bool,
        ),
        (
            StorybookSettingsField::Selection,
            is_selection_enabled as fn(&ViewerInteractionConfig) -> bool,
        ),
        (
            StorybookSettingsField::ImageControls,
            is_image_controls_enabled as fn(&ViewerInteractionConfig) -> bool,
        ),
        (
            StorybookSettingsField::DiagramControls,
            is_diagram_controls_enabled as fn(&ViewerInteractionConfig) -> bool,
        ),
        (
            StorybookSettingsField::CodeControls,
            is_code_controls_enabled as fn(&ViewerInteractionConfig) -> bool,
        ),
    ];

    for (field, is_enabled) in cases {
        let mut dark = true;
        let mut mode = ViewerMode::Document;
        let mut interaction = ViewerInteractionConfig::default();
        let mut typography = ViewerTypographyConfig::default();
        let mut settings_state = StorybookSettingsState::default();

        StorybookSettingsAction::apply_field(
            field,
            request(
                &mut dark,
                &mut mode,
                &mut interaction,
                &mut typography,
                &mut settings_state,
            ),
        )?;

        assert!(!is_enabled(&interaction), "{field:?} did not toggle off");
    }
    Ok(())
}

#[test]
fn toggle_section_uses_kuc_settings_action_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut dark = true;
    let mut mode = ViewerMode::Document;
    let mut interaction = ViewerInteractionConfig::default();
    let mut typography = ViewerTypographyConfig::default();
    let mut settings_state = StorybookSettingsState::default();

    StorybookSettingsAction::apply_action(
        SettingsListAction::ToggleSection {
            section_id: "display".to_string(),
        },
        request(
            &mut dark,
            &mut mode,
            &mut interaction,
            &mut typography,
            &mut settings_state,
        ),
    )?;

    assert!(settings_state.is_collapsed("display"));
    Ok(())
}

#[test]
fn preview_font_size_uses_kuc_settings_action_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut dark = true;
    let mut mode = ViewerMode::Document;
    let mut interaction = ViewerInteractionConfig::default();
    let mut typography = ViewerTypographyConfig::default();
    let mut settings_state = StorybookSettingsState::default();

    StorybookSettingsAction::apply_field(
        StorybookSettingsField::PreviewFontSize,
        request(
            &mut dark,
            &mut mode,
            &mut interaction,
            &mut typography,
            &mut settings_state,
        ),
    )?;

    assert_eq!(16, typography.preview_font_size);
    Ok(())
}

fn request<'a>(
    dark: &'a mut bool,
    mode: &'a mut ViewerMode,
    interaction: &'a mut ViewerInteractionConfig,
    typography: &'a mut ViewerTypographyConfig,
    settings_state: &'a mut StorybookSettingsState,
) -> StorybookSettingsRequest<'a> {
    StorybookSettingsRequest {
        scene: None,
        dark,
        mode,
        interaction,
        typography,
        settings_state,
        width: 320,
        height: 240,
    }
}

fn is_hover_highlight_enabled(interaction: &ViewerInteractionConfig) -> bool {
    interaction.hover_highlight_enabled
}

fn is_selection_enabled(interaction: &ViewerInteractionConfig) -> bool {
    interaction.selection_enabled
}

fn is_image_controls_enabled(interaction: &ViewerInteractionConfig) -> bool {
    interaction.image_controls_enabled
}

fn is_diagram_controls_enabled(interaction: &ViewerInteractionConfig) -> bool {
    interaction.diagram_controls_enabled
}

fn is_code_controls_enabled(interaction: &ViewerInteractionConfig) -> bool {
    interaction.code_controls_enabled
}
