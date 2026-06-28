#[derive(Debug, Clone, Copy)]
pub(super) enum StorybookForbiddenPattern {
    TextHeight,
    ButtonWidth,
    ButtonHeight,
    SetValueSynthesis,
    SettingsFieldStateParser,
    SettingsModeParser,
    SettingsInteractionFieldMapper,
    StateIdParser,
    ViewerImageActionPrefix,
    ViewerDiagramActionPrefix,
    ViewerCodeActionPrefix,
    KucMediaControlTarget,
    KucMediaControlAction,
    KucMediaControlParser,
    TaskStatePrefix,
    ManualBooleanInversion,
    FileTreeAnalyticHitTarget,
    SettingsFieldAnalyticHitTarget,
    SettingsSectionAnalyticHitTarget,
}

impl StorybookForbiddenPattern {
    pub(super) fn all() -> &'static [Self] {
        &[
            Self::TextHeight,
            Self::ButtonWidth,
            Self::ButtonHeight,
            Self::SetValueSynthesis,
            Self::SettingsFieldStateParser,
            Self::SettingsModeParser,
            Self::SettingsInteractionFieldMapper,
            Self::StateIdParser,
            Self::ViewerImageActionPrefix,
            Self::ViewerDiagramActionPrefix,
            Self::ViewerCodeActionPrefix,
            Self::KucMediaControlTarget,
            Self::KucMediaControlAction,
            Self::KucMediaControlParser,
            Self::TaskStatePrefix,
            Self::ManualBooleanInversion,
            Self::FileTreeAnalyticHitTarget,
            Self::SettingsFieldAnalyticHitTarget,
            Self::SettingsSectionAnalyticHitTarget,
        ]
    }

    pub(super) fn needle(self) -> &'static str {
        match self {
            Self::TextHeight => "TEXT_HEIGHT",
            Self::ButtonWidth => "BUTTON_WIDTH",
            Self::ButtonHeight => "BUTTON_HEIGHT",
            Self::SetValueSynthesis => "UiAction::SetValue",
            Self::SettingsFieldStateParser => "settings-field:",
            Self::SettingsModeParser => "mode_from_label",
            Self::SettingsInteractionFieldMapper => "apply_interaction_field",
            Self::StateIdParser => "parse_state_id",
            Self::ViewerImageActionPrefix => "viewer.image.",
            Self::ViewerDiagramActionPrefix => "viewer.diagram.",
            Self::ViewerCodeActionPrefix => "viewer.code.",
            Self::KucMediaControlTarget => "UiMediaControlTarget",
            Self::KucMediaControlAction => "UiMediaControlAction",
            Self::KucMediaControlParser => "media_control_action",
            Self::TaskStatePrefix => "kdv-task-state:",
            Self::ManualBooleanInversion => "= !",
            Self::FileTreeAnalyticHitTarget => "hit_target_for_item_with_state",
            Self::SettingsFieldAnalyticHitTarget => "hit_target_for_field(",
            Self::SettingsSectionAnalyticHitTarget => "hit_target_for_section(",
        }
    }

    pub(super) fn rule(self) -> &'static str {
        match self {
            Self::TextHeight => "no_manual_tree_hit_test",
            Self::ButtonWidth | Self::ButtonHeight => "no_manual_media_hit_test",
            Self::SetValueSynthesis => "no_storybook_action_synthesis",
            Self::SettingsFieldStateParser
            | Self::SettingsModeParser
            | Self::SettingsInteractionFieldMapper
            | Self::ManualBooleanInversion => "no_manual_settings_action",
            Self::FileTreeAnalyticHitTarget
            | Self::SettingsFieldAnalyticHitTarget
            | Self::SettingsSectionAnalyticHitTarget => "no_kuc_analytic_hit_target",
            Self::StateIdParser
            | Self::ViewerImageActionPrefix
            | Self::ViewerDiagramActionPrefix
            | Self::ViewerCodeActionPrefix
            | Self::KucMediaControlTarget
            | Self::KucMediaControlAction
            | Self::KucMediaControlParser
            | Self::TaskStatePrefix => "no_style_class_action_contract",
        }
    }

    pub(super) fn message(self) -> &'static str {
        if let Some(message) = self.hit_target_message() {
            return message;
        }
        if let Some(message) = self.style_class_message() {
            return message;
        }
        match self {
            Self::TextHeight => "Storybook must not recreate TreeView row geometry.",
            Self::ButtonWidth | Self::ButtonHeight => {
                "Storybook must not recreate media control button geometry."
            }
            Self::SetValueSynthesis => "Storybook must not synthesize KUC selection actions.",
            Self::SettingsFieldStateParser => {
                "Storybook must not parse settings state ids as action contracts."
            }
            Self::SettingsModeParser | Self::SettingsInteractionFieldMapper => {
                "Storybook settings must delegate viewer state semantics to KDV core."
            }
            Self::ManualBooleanInversion => {
                "Storybook settings must use KUC action values instead of local boolean inversion."
            }
            _ => unreachable!("message group handled before fallback match"),
        }
    }

    fn hit_target_message(self) -> Option<&'static str> {
        match self {
            Self::FileTreeAnalyticHitTarget
            | Self::SettingsFieldAnalyticHitTarget
            | Self::SettingsSectionAnalyticHitTarget => Some(
                "Storybook must use rendered KUC host action rects instead of analytic hit-target helpers.",
            ),
            _ => None,
        }
    }

    fn style_class_message(self) -> Option<&'static str> {
        match self {
            Self::StateIdParser => Some("Storybook must not parse state ids as action contracts."),
            Self::ViewerImageActionPrefix
            | Self::ViewerDiagramActionPrefix
            | Self::ViewerCodeActionPrefix => {
                Some("Storybook must not parse media action string prefixes.")
            }
            Self::KucMediaControlTarget
            | Self::KucMediaControlAction
            | Self::KucMediaControlParser => {
                Some("Storybook must not depend on KUC viewer media control semantics.")
            }
            Self::TaskStatePrefix => {
                Some("Storybook must not parse task state ids as action contracts.")
            }
            _ => None,
        }
    }
}
