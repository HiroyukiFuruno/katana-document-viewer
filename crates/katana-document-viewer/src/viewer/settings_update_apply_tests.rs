use super::{ViewerSettingsField, update_interaction};
use crate::viewer::ViewerInteractionConfig;

#[test]
fn non_interaction_fields_leave_interaction_state_unchanged() {
    let expected = ViewerInteractionConfig::default();
    for field in [
        ViewerSettingsField::Dark,
        ViewerSettingsField::Theme,
        ViewerSettingsField::Mode,
        ViewerSettingsField::PreviewFontSize,
    ] {
        let mut interaction = expected.clone();
        update_interaction(field, &mut interaction, false);
        assert_eq!(expected, interaction);
    }
}
