use super::TaskStateExpectation;

#[test]
fn rendered_task_requires_marker_state_and_visual_contracts() {
    let task = TaskStateExpectation::new("[/]", "in-progress", "in-progress-slash");
    let marker = r#"<li data-kdv-task-marker="[/]">"#;
    let state = r#" data-kdv-task-state="in-progress""#;
    let visual = r#" data-kdv-task-visual="in-progress-slash""#;

    assert!(!task.is_rendered_in(""));
    assert!(!task.is_rendered_in(marker));
    assert!(!task.is_rendered_in(&format!("{marker}{state}")));
    assert!(task.is_rendered_in(&format!("{marker}{state}{visual}")));
}
