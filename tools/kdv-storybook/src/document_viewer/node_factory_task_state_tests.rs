use super::{KdvTaskState, task_context_menu};
use katana_ui_core::render_model::UiTaskMarker;

#[test]
fn task_state_uses_kuc_marker_contract() -> Result<(), String> {
    let state = KdvTaskState::from_marker("[X]").ok_or("known marker")?;

    assert_eq!("[x]", state.marker());
    assert_eq!(Some(UiTaskMarker::Done), UiTaskMarker::from_marker("[X]"));
    Ok(())
}

#[test]
fn task_state_reports_expected_attributes() -> Result<(), String> {
    let done = KdvTaskState::from_marker("[x]").ok_or("done marker")?;
    let progress = KdvTaskState::from_marker("[/]").ok_or("progress marker")?;
    let blocked = KdvTaskState::from_marker("[-]").ok_or("blocked marker")?;
    let empty = KdvTaskState::from_marker("[ ]").ok_or("empty marker")?;

    assert_eq!("[x]", done.marker());
    assert_eq!("kdv-task-blocked", blocked.style_class());
    assert_eq!("実施中", progress.accessibility_label());
    assert!(done.is_active());
    assert!(!empty.is_active());
    Ok(())
}

#[test]
fn task_context_menu_marks_selected_state_checked() -> Result<(), String> {
    let menu = task_context_menu(KdvTaskState::from_marker("[x]").ok_or("done marker")?);
    let selected_items = menu
        .items
        .iter()
        .filter(|item| item.checked)
        .collect::<Vec<_>>();

    assert_eq!(1, selected_items.len());
    assert_eq!("ui.task.state.done", selected_items[0].id.as_str());
    assert_eq!(4, menu.items.len());
    Ok(())
}
