use crate::settings_action::StorybookSettingsField;
use minifb::{Key, KeyRepeat, Window};

pub struct StorybookInteractionKeys;

impl StorybookInteractionKeys {
    pub fn apply<F>(window: &Window, mut apply_field: F) -> Result<bool, Box<dyn std::error::Error>>
    where
        F: FnMut(StorybookSettingsField) -> Result<bool, Box<dyn std::error::Error>>,
    {
        let mut changed = false;
        changed |= Self::apply_key(
            window,
            Key::H,
            StorybookSettingsField::Hover,
            &mut apply_field,
        )?;
        changed |= Self::apply_key(
            window,
            Key::S,
            StorybookSettingsField::Selection,
            &mut apply_field,
        )?;
        changed |= Self::apply_key(
            window,
            Key::I,
            StorybookSettingsField::ImageControls,
            &mut apply_field,
        )?;
        changed |= Self::apply_key(
            window,
            Key::G,
            StorybookSettingsField::DiagramControls,
            &mut apply_field,
        )?;
        Ok(changed)
    }

    fn apply_key<F>(
        window: &Window,
        key: Key,
        field: StorybookSettingsField,
        apply_field: &mut F,
    ) -> Result<bool, Box<dyn std::error::Error>>
    where
        F: FnMut(StorybookSettingsField) -> Result<bool, Box<dyn std::error::Error>>,
    {
        if !window.is_key_pressed(key, KeyRepeat::No) {
            return Ok(false);
        }
        apply_field(field)
    }
}
