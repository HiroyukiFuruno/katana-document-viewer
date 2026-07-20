use crate::ArtifactId;
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};

#[test]
fn image_control_action_becomes_image_command() -> Result<(), String> {
    let image_target = target("image-node");
    let command = crate::viewer::ViewerCommandFactory::image_control_from_action(
        image_target.clone(),
        "reveal-in-os",
    );
    let invalid = crate::viewer::ViewerCommandFactory::image_control_from_action(
        image_target.clone(),
        "pan-up",
    );

    assert!(invalid.is_none());
    let Some(command) = command else {
        return Err("image action must create command".to_string());
    };
    let crate::viewer::ViewerCommand::Image(image) = command else {
        return Err("expected image command".to_string());
    };
    assert_eq!(image_target, image.target);
    assert_eq!(crate::viewer::ImageControlAction::RevealInOs, image.action);
    Ok(())
}

#[test]
fn image_control_action_rejects_unknown_command() {
    assert!(
        crate::viewer::ViewerCommandFactory::image_control_from_action(
            target("image-node"),
            "unknown",
        )
        .is_none()
    );
}

#[test]
fn task_marker_marker_roundtrip() -> Result<(), String> {
    let target = target("task-target");
    let done =
        crate::viewer::ViewerCommandFactory::set_task_state_from_marker(target.clone(), "[x]")
            .ok_or_else(|| "missing done task command".to_string())?;
    let unknown = crate::viewer::ViewerCommandFactory::set_task_state_from_marker(target, "[?]");

    let crate::viewer::ViewerCommand::Task(crate::viewer::TaskStateCommand { state, .. }) = done
    else {
        return Err("expected task command".to_string());
    };
    assert_eq!(crate::viewer::ViewerTaskState::Done, state);
    assert!(unknown.is_none());
    Ok(())
}

#[test]
fn diagram_control_action_rejects_unknown_actions() -> Result<(), String> {
    let diagram_target = target("diagram-node");
    let command = crate::viewer::ViewerCommandFactory::diagram_control_from_action(
        diagram_target,
        "does-not-exist",
        false,
    );

    assert!(command.is_none());
    Ok(())
}

#[test]
fn diagram_control_action_becomes_diagram_command() -> Result<(), String> {
    let diagram_target = target("diagram-node");
    let command = crate::viewer::ViewerCommandFactory::diagram_control_from_action(
        diagram_target.clone(),
        "pan-up",
        false,
    );
    let invalid = crate::viewer::ViewerCommandFactory::diagram_control_from_action(
        diagram_target.clone(),
        "copy",
        false,
    );

    assert!(invalid.is_none());
    let Some(command) = command else {
        return Err("diagram action must create command".to_string());
    };
    assert_pan_command(
        command,
        &diagram_target,
        crate::viewer::DiagramPanSource::ButtonUp,
    )
}

#[test]
fn diagram_source_copy_action_becomes_host_copy_command() -> Result<(), String> {
    let diagram_target = target("diagram-source-node");
    let command = crate::viewer::ViewerCommandFactory::diagram_control_from_action(
        diagram_target.clone(),
        "copy-source",
        false,
    );

    let Some(crate::viewer::ViewerCommand::Host(crate::viewer::HostCommand::CopyText(copy))) =
        command
    else {
        return Err("expected host copy command".to_string());
    };
    assert_eq!(crate::viewer::CopyTextSource::DiagramSource, copy.source);
    assert_eq!(diagram_target, copy.target);
    assert_eq!(diagram_target.source.raw.text, copy.text);
    Ok(())
}

#[test]
fn code_copy_action_becomes_host_copy_command() -> Result<(), String> {
    let code_target = target("code-node");
    let command = crate::viewer::ViewerCommandFactory::code_control_from_action(
        code_target.clone(),
        "copy-code",
    );
    let invalid =
        crate::viewer::ViewerCommandFactory::code_control_from_action(code_target.clone(), "copy");

    assert!(invalid.is_none());
    let Some(crate::viewer::ViewerCommand::Host(crate::viewer::HostCommand::CopyText(copy))) =
        command
    else {
        return Err("expected code copy command".to_string());
    };
    assert_eq!(crate::viewer::CopyTextSource::Code, copy.source);
    assert_eq!(code_target, copy.target);
    assert_eq!(code_target.source.raw.text, copy.text);
    Ok(())
}

#[test]
fn scroll_to_target_preserves_the_rendered_target() -> Result<(), String> {
    let heading = target("heading-node");
    let command = crate::viewer::ViewerCommandFactory::scroll_to_target(heading.clone());

    let crate::viewer::ViewerCommand::ScrollToHeading(scroll) = command else {
        return Err("expected heading scroll command".to_string());
    };
    assert_eq!(heading, scroll.target);
    Ok(())
}

#[test]
fn fullscreen_action_uses_current_diagram_state() -> Result<(), String> {
    let diagram_target = target("diagram-fullscreen-node");
    let open = crate::viewer::ViewerCommandFactory::diagram_control_from_action(
        diagram_target.clone(),
        "fullscreen",
        false,
    );
    let close = crate::viewer::ViewerCommandFactory::diagram_control_from_action(
        diagram_target.clone(),
        "fullscreen",
        true,
    );

    assert_fullscreen_command(open, &diagram_target, false)?;
    assert_fullscreen_command(close, &diagram_target, true)
}

#[test]
fn only_fullscreen_diagram_command_requires_host_propagation() -> Result<(), String> {
    let diagram_target = target("diagram-host-propagation-node");
    let fullscreen = crate::viewer::ViewerCommandFactory::diagram_control_from_action(
        diagram_target.clone(),
        "fullscreen",
        false,
    );
    let pan = crate::viewer::ViewerCommandFactory::diagram_control_from_action(
        diagram_target.clone(),
        "pan-up",
        false,
    );
    let zoom = crate::viewer::ViewerCommandFactory::diagram_control_from_action(
        diagram_target.clone(),
        "zoom-in",
        false,
    );
    let reset = crate::viewer::ViewerCommandFactory::diagram_control_from_action(
        diagram_target.clone(),
        "reset-view",
        false,
    );
    let help = crate::viewer::ViewerCommandFactory::diagram_control_from_action(
        diagram_target,
        "trackpad-help",
        false,
    );

    assert!(diagram_command(fullscreen)?.requires_host_propagation());
    assert!(!diagram_command(pan)?.requires_host_propagation());
    assert!(!diagram_command(zoom)?.requires_host_propagation());
    assert!(!diagram_command(reset)?.requires_host_propagation());
    assert!(!diagram_command(help)?.requires_host_propagation());
    Ok(())
}

fn diagram_command(
    command: Option<crate::viewer::ViewerCommand>,
) -> Result<crate::viewer::DiagramControlCommand, String> {
    let Some(crate::viewer::ViewerCommand::Diagram(command)) = command else {
        return Err("expected diagram command".to_string());
    };
    Ok(command)
}

fn assert_pan_command(
    command: crate::viewer::ViewerCommand,
    target: &crate::viewer::ViewerTarget,
    source: crate::viewer::DiagramPanSource,
) -> Result<(), String> {
    let crate::viewer::ViewerCommand::Diagram(crate::viewer::DiagramControlCommand::Pan(pan)) =
        command
    else {
        return Err("expected diagram pan command".to_string());
    };
    assert_eq!(*target, pan.target);
    assert_eq!(source, pan.source);
    Ok(())
}

fn assert_fullscreen_command(
    command: Option<crate::viewer::ViewerCommand>,
    target: &crate::viewer::ViewerTarget,
    opened: bool,
) -> Result<(), String> {
    let Some(command) = command else {
        return Err("fullscreen action must create command".to_string());
    };
    match command {
        crate::viewer::ViewerCommand::Diagram(
            crate::viewer::DiagramControlCommand::FullscreenOpen(value),
        ) if !opened => assert_eq!(*target, value),
        crate::viewer::ViewerCommand::Diagram(
            crate::viewer::DiagramControlCommand::FullscreenClose(value),
        ) if opened => assert_eq!(*target, value),
        _ => return Err("expected matching fullscreen command".to_string()),
    }
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
