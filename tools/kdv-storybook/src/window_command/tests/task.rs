use super::support::{find_node_state_id, find_node_value, first_target_containing, storybook};
use katana_document_viewer::{ArtifactId, ViewerCommandFactory};

#[test]
fn task_command_updates_kuc_checkbox_state() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("katana/sample_basic.md");
    storybook.update_scene(1000, 900)?;
    let state_id = find_node_state_id(&storybook, "kdv-task-checkbox", "[ ]")?;
    let mut target = first_target_containing(&storybook, "[ ]")?;
    target.artifact_id = ArtifactId(state_id.clone());
    let command = ViewerCommandFactory::set_task_state_from_marker(target, "[x]")
        .ok_or_else(|| std::io::Error::other("missing task command"))?;

    assert!(storybook.apply_viewer_command(&command));
    storybook.update_scene(1000, 900)?;

    let marker = find_node_value(&storybook, state_id.as_str())?;
    assert_eq!("[x]", marker);
    assert_eq!(1, storybook.settings_state.task_change_count());
    let last_task = storybook.settings_state.last_task_change_label();
    assert!(last_task.starts_with(&format!("{state_id} ")));
    assert!(last_task.contains("[ ] -> [x]"));
    assert!(
        storybook
            .settings_state
            .last_task_source_label()
            .contains("sample_basic.md")
    );
    assert!(
        storybook
            .settings_state
            .last_task_location_label()
            .contains("katana/sample_basic.md:")
    );
    assert!(
        storybook
            .settings_state
            .last_task_location_label()
            .contains(&format!("artifact={state_id}"))
    );
    assert!(
        storybook
            .settings_state
            .last_task_target_label()
            .contains(&format!("artifact={state_id}"))
    );
    assert!(
        storybook
            .settings_state
            .last_task_span_label()
            .contains("bytes")
    );
    Ok(())
}

#[test]
fn task_session_state_records_external_state_transition() -> Result<(), Box<dyn std::error::Error>>
{
    let mut storybook = storybook("katana/sample_basic.md");
    storybook.update_scene(1000, 900)?;
    let state_id = find_node_state_id(&storybook, "kdv-task-checkbox", "[ ]")?;
    let mut target = first_target_containing(&storybook, "[ ]")?;
    target.artifact_id = ArtifactId(state_id.clone());

    let done = ViewerCommandFactory::set_task_state_from_marker(target.clone(), "[x]")
        .ok_or_else(|| std::io::Error::other("missing done command"))?;
    let blocked = ViewerCommandFactory::set_task_state_from_marker(target, "[-]")
        .ok_or_else(|| std::io::Error::other("missing blocked command"))?;

    assert!(storybook.apply_viewer_command(&done));
    storybook.update_scene(1000, 900)?;
    assert!(storybook.apply_viewer_command(&blocked));

    assert_eq!(2, storybook.settings_state.task_change_count());
    let last_task = storybook.settings_state.last_task_change_label();
    assert!(last_task.starts_with(&format!("{state_id} ")));
    assert!(last_task.contains("[x] -> [-]"));
    assert_eq!(
        2,
        storybook.settings_state.task_change_location_labels().len()
    );
    Ok(())
}

#[test]
fn task_session_state_clears_when_fixture_state_resets() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("katana/sample_basic.md");
    storybook.update_scene(1000, 900)?;
    let state_id = find_node_state_id(&storybook, "kdv-task-checkbox", "[ ]")?;
    let mut target = first_target_containing(&storybook, "[ ]")?;
    target.artifact_id = ArtifactId(state_id);
    let command = ViewerCommandFactory::set_task_state_from_marker(target, "[x]")
        .ok_or_else(|| std::io::Error::other("missing task command"))?;

    assert!(storybook.apply_viewer_command(&command));
    assert_eq!(1, storybook.settings_state.task_change_count());

    storybook.reset_fixture_state();

    assert_eq!(0, storybook.settings_state.task_change_count());
    assert_eq!("none", storybook.settings_state.last_task_change_label());
    assert_eq!("none", storybook.settings_state.last_task_location_label());
    assert_eq!("none", storybook.settings_state.last_task_target_label());
    assert_eq!("none", storybook.settings_state.last_task_span_label());
    Ok(())
}
