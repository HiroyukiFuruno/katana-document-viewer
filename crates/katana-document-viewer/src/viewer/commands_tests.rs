use crate::ArtifactId;
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};

#[test]
fn converts_task_state_marker_to_variant() {
    assert_eq!(
        Some(crate::viewer::ViewerTaskState::Empty),
        crate::viewer::ViewerTaskState::from_marker("[ ]")
    );
    assert_eq!(
        Some(crate::viewer::ViewerTaskState::Done),
        crate::viewer::ViewerTaskState::from_marker("[x]")
    );
    assert_eq!(
        Some(crate::viewer::ViewerTaskState::Done),
        crate::viewer::ViewerTaskState::from_marker("[X]")
    );
    assert_eq!(
        Some(crate::viewer::ViewerTaskState::Progress),
        crate::viewer::ViewerTaskState::from_marker("[/]")
    );
    assert_eq!(
        Some(crate::viewer::ViewerTaskState::Blocked),
        crate::viewer::ViewerTaskState::from_marker("[-]")
    );
    assert_eq!(None, crate::viewer::ViewerTaskState::from_marker("[?]"));
}

#[test]
fn renders_task_state_marker_as_text() {
    assert_eq!("[ ]", crate::viewer::ViewerTaskState::Empty.marker());
    assert_eq!("[x]", crate::viewer::ViewerTaskState::Done.marker());
    assert_eq!("[/]", crate::viewer::ViewerTaskState::Progress.marker());
    assert_eq!("[-]", crate::viewer::ViewerTaskState::Blocked.marker());
}

#[test]
fn link_click_command_preserves_target_metadata() -> Result<(), String> {
    let target = target("link-node");
    let command =
        crate::viewer::ViewerCommandFactory::open_link(target.clone(), "https://example.com/docs");

    let crate::viewer::ViewerCommand::Link(link) = command else {
        return Err("expected link command".to_string());
    };
    assert_eq!(target, link.target);
    assert_eq!("https://example.com/docs", link.uri);
    Ok(())
}

#[test]
fn task_left_click_toggles_between_empty_and_done_commands() -> Result<(), String> {
    let target = target("task-node");
    let done = crate::viewer::ViewerCommandFactory::toggle_task(
        target.clone(),
        crate::viewer::ViewerTaskState::Empty,
    );
    let empty = crate::viewer::ViewerCommandFactory::toggle_task(
        target.clone(),
        crate::viewer::ViewerTaskState::Progress,
    );

    assert_task_command(done, &target, crate::viewer::ViewerTaskState::Done)?;
    assert_task_command(empty, &target, crate::viewer::ViewerTaskState::Empty)?;
    Ok(())
}

#[test]
fn task_control_command_preserves_typed_task_target() -> Result<(), String> {
    let target = target("task-control-node");
    let task_target = crate::viewer::ViewerTaskControlTarget {
        node_id: target.node_id.clone(),
        row_index: 2,
        state_id: "ui-task-state:task-control-node:2".to_string(),
    };
    let command = crate::viewer::ViewerCommandFactory::toggle_task_control(
        target.clone(),
        task_target.clone(),
        crate::viewer::ViewerTaskState::Empty,
    );

    let crate::viewer::ViewerCommand::Task(task) = command else {
        return Err("expected task command".to_string());
    };
    assert_eq!(target, task.target);
    assert_eq!(Some(task_target), task.task_target);
    assert_eq!(crate::viewer::ViewerTaskState::Done, task.state);
    Ok(())
}

#[test]
fn task_context_menu_marker_becomes_state_command() -> Result<(), String> {
    let task_target = target("task-context-node");
    let command =
        crate::viewer::ViewerCommandFactory::set_task_state_from_marker(task_target.clone(), "[-]");
    let invalid =
        crate::viewer::ViewerCommandFactory::set_task_state_from_marker(task_target.clone(), "[?]");

    assert!(invalid.is_none());
    let Some(command) = command else {
        return Err("task marker must create command".to_string());
    };
    assert_task_command(
        command,
        &task_target,
        crate::viewer::ViewerTaskState::Blocked,
    )
}

fn assert_task_command(
    command: crate::viewer::ViewerCommand,
    target: &crate::viewer::ViewerTarget,
    state: crate::viewer::ViewerTaskState,
) -> Result<(), String> {
    let crate::viewer::ViewerCommand::Task(task) = command else {
        return Err("expected task command".to_string());
    };
    assert_eq!(*target, task.target);
    assert_eq!(None, task.task_target);
    assert_eq!(state, task.state);
    Ok(())
}

fn target(node_id: &str) -> crate::viewer::ViewerTarget {
    crate::viewer::ViewerTarget {
        node_id: KmmNodeId(node_id.to_string()),
        source: source(node_id),
        artifact_id: ArtifactId(format!("artifact-{node_id}")),
        rect: crate::viewer::ViewerRect {
            x: 1.0,
            y: 2.0,
            width: 30.0,
            height: 12.0,
        },
    }
}

fn source(raw: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: raw.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: raw.len() + 1,
            },
        },
        raw: RawSnippet {
            text: raw.to_string(),
        },
    }
}
