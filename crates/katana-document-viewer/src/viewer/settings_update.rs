use super::types::{ViewerInteractionConfig, ViewerMode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerSettingsField {
    Dark,
    Theme,
    Mode,
    PreviewFontSize,
    Hover,
    Selection,
    ImageControls,
    DiagramControls,
    CodeControls,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerSettingsValue {
    Bool(bool),
    Text(String),
    Number(i64),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerSettingsState {
    pub dark: bool,
    pub mode: ViewerMode,
    pub interaction: ViewerInteractionConfig,
    pub typography: ViewerTypographyConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerSettingsUpdate {
    pub field: ViewerSettingsField,
    pub value: ViewerSettingsValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerTypographyConfig {
    pub preview_font_size: u16,
}

impl Default for ViewerTypographyConfig {
    fn default() -> Self {
        Self {
            preview_font_size: 14,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewerSettingsUpdateError {
    UnknownField(String),
    InvalidValue {
        field: ViewerSettingsField,
        value: ViewerSettingsValue,
    },
}

impl ViewerSettingsField {
    #[must_use]
    pub fn from_id(value: &str) -> Option<Self> {
        match value {
            "dark" => Some(Self::Dark),
            "theme" => Some(Self::Theme),
            "mode" => Some(Self::Mode),
            "preview-font-size" => Some(Self::PreviewFontSize),
            "hover" => Some(Self::Hover),
            "selection" => Some(Self::Selection),
            "image-controls" => Some(Self::ImageControls),
            "diagram-controls" => Some(Self::DiagramControls),
            "code-controls" => Some(Self::CodeControls),
            _ => None,
        }
    }

    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Theme => "theme",
            Self::Mode => "mode",
            Self::PreviewFontSize => "preview-font-size",
            Self::Hover => "hover",
            Self::Selection => "selection",
            Self::ImageControls => "image-controls",
            Self::DiagramControls => "diagram-controls",
            Self::CodeControls => "code-controls",
        }
    }
}

impl ViewerSettingsState {
    #[must_use]
    pub fn new(dark: bool, mode: ViewerMode, interaction: ViewerInteractionConfig) -> Self {
        Self {
            dark,
            mode,
            interaction,
            typography: ViewerTypographyConfig::default(),
        }
    }
}

impl ViewerSettingsUpdate {
    pub fn from_field_id(
        field_id: &str,
        value: ViewerSettingsValue,
    ) -> Result<Self, ViewerSettingsUpdateError> {
        let field = ViewerSettingsField::from_id(field_id)
            .ok_or_else(|| ViewerSettingsUpdateError::UnknownField(field_id.to_string()))?;
        Ok(Self { field, value })
    }
}

impl Display for ViewerSettingsUpdateError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownField(field) => write!(formatter, "unknown viewer settings field {field}"),
            Self::InvalidValue { field, value } => {
                write!(
                    formatter,
                    "invalid viewer settings value {value:?} for {field:?}"
                )
            }
        }
    }
}

impl Error for ViewerSettingsUpdateError {}

#[path = "settings_update_apply.rs"]
mod apply;

#[cfg(test)]
#[path = "settings_update_tests.rs"]
mod tests;
