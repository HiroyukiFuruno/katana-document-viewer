use super::{StorybookMouseButton, StorybookMouseState};

#[test]
fn mouse_state_emits_single_press_edge() {
    let mut state = StorybookMouseState::default();

    assert_eq!(Some(StorybookMouseButton::Left), state.pressed(true, false));
    assert_eq!(None, state.pressed(true, false));
    assert_eq!(None, state.pressed(false, false));
    assert_eq!(
        Some(StorybookMouseButton::Right),
        state.pressed(false, true)
    );
}
