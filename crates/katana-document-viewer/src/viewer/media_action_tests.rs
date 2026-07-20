use crate::ArtifactId;
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};

#[test]
fn viewer_media_action_parses_host_action_without_kuc_media_semantics() -> Result<(), String> {
    let action = crate::viewer::ViewerMediaControlAction::from_host_action(
        "viewer.diagram.zoom-in",
        "diagram-node",
    )
    .ok_or_else(|| "diagram action must parse".to_string())?;

    assert_eq!(crate::viewer::ViewerMediaControlKind::Diagram, action.kind);
    assert_eq!("diagram-node", action.node_id);
    assert_eq!("zoom-in", action.command);
    Ok(())
}

#[test]
fn viewer_media_action_rejects_mismatched_target_payload() {
    let command = crate::viewer::ViewerCommandFactory::media_control_from_host_action(
        target("other-node"),
        "viewer.diagram.zoom-in",
        "diagram-node",
        false,
    );

    assert!(command.is_none());
}

#[test]
fn viewer_media_action_creates_diagram_command_from_host_action() -> Result<(), String> {
    let target = target("diagram-node");
    let command = crate::viewer::ViewerCommandFactory::media_control_from_host_action(
        target.clone(),
        "viewer.diagram.zoom-in",
        "diagram-node",
        false,
    )
    .ok_or_else(|| "diagram command must be created".to_string())?;

    let crate::viewer::ViewerCommand::Diagram(crate::viewer::DiagramControlCommand::Zoom(zoom)) =
        command
    else {
        return Err("expected diagram zoom command".to_string());
    };
    assert_eq!(target, zoom.target);
    assert_eq!(crate::viewer::DiagramZoomSource::ButtonIn, zoom.source);
    Ok(())
}

#[test]
fn viewer_media_action_rejects_unknown_host_prefix() -> Result<(), String> {
    let action = crate::viewer::ViewerMediaControlAction::from_host_action(
        "viewer.media.fit",
        "diagram-node",
    );
    assert!(action.is_none());

    let command = crate::viewer::ViewerCommandFactory::media_control_from_host_action(
        target("diagram-node"),
        "viewer.media.fit",
        "diagram-node",
        false,
    );
    assert!(command.is_none());
    Ok(())
}

#[test]
fn viewer_media_action_image_open_command() -> Result<(), String> {
    let image_target = target("image-node");
    let image = crate::viewer::ViewerCommandFactory::media_control_from_host_action(
        image_target.clone(),
        "viewer.image.open",
        "image-node",
        false,
    )
    .ok_or_else(|| "image action must create command".to_string())?;
    let crate::viewer::ViewerCommand::Image(crate::viewer::ImageControlCommand {
        action: crate::viewer::ImageControlAction::Open,
        ..
    }) = image
    else {
        return Err("expected image command".to_string());
    };
    Ok(())
}

#[test]
fn viewer_media_action_code_copy_command() -> Result<(), String> {
    let code_target = target("code-node");
    let code = crate::viewer::ViewerCommandFactory::media_control_from_host_action(
        code_target.clone(),
        "viewer.code.copy-code",
        "code-node",
        false,
    )
    .ok_or_else(|| "code action must create command".to_string())?;
    let crate::viewer::ViewerCommand::Host(crate::viewer::HostCommand::CopyText(copy)) = code
    else {
        return Err("expected host copy command".to_string());
    };
    assert_eq!(code_target, copy.target);
    Ok(())
}

#[test]
fn viewer_media_action_host_action_id_is_generated_for_each_kind() {
    assert_eq!(
        "viewer.image.open".to_string(),
        crate::viewer::ViewerMediaControlAction::host_action_id_for(
            crate::viewer::ViewerMediaControlKind::Image,
            "open"
        )
    );
    assert_eq!(
        "viewer.diagram.reset-view".to_string(),
        crate::viewer::ViewerMediaControlAction::host_action_id_for(
            crate::viewer::ViewerMediaControlKind::Diagram,
            "reset-view"
        )
    );
    assert_eq!(
        "viewer.code.copy-code".to_string(),
        crate::viewer::ViewerMediaControlAction::host_action_id_for(
            crate::viewer::ViewerMediaControlKind::Code,
            "copy-code"
        )
    );
}

#[test]
fn viewer_media_action_constructor_builds_its_host_action_id() {
    let action = crate::viewer::ViewerMediaControlAction::new(
        crate::viewer::ViewerMediaControlKind::Image,
        "image-node",
        "fit",
    );

    assert_eq!("image-node", action.node_id);
    assert_eq!("fit", action.command);
    assert_eq!("viewer.image.fit", action.host_action_id());
}

#[test]
fn viewer_media_action_from_host_action_handles_all_media_prefixes() -> Result<(), String> {
    let image = crate::viewer::ViewerMediaControlAction::from_host_action(
        "viewer.image.open",
        "diagram-node",
    )
    .ok_or_else(|| "image action must parse".to_string())?;
    let code = crate::viewer::ViewerMediaControlAction::from_host_action(
        "viewer.code.copy-code",
        "diagram-node",
    )
    .ok_or_else(|| "code action must parse".to_string())?;
    let diagram = crate::viewer::ViewerMediaControlAction::from_host_action(
        "viewer.diagram.pan-up",
        "diagram-node",
    )
    .ok_or_else(|| "diagram action must parse".to_string())?;

    assert_eq!(crate::viewer::ViewerMediaControlKind::Image, image.kind);
    assert_eq!(crate::viewer::ViewerMediaControlKind::Code, code.kind);
    assert_eq!(crate::viewer::ViewerMediaControlKind::Diagram, diagram.kind);

    Ok(())
}

fn target(node_id: &str) -> crate::viewer::ViewerTarget {
    crate::viewer::ViewerTarget {
        node_id: KmmNodeId(node_id.to_string()),
        artifact_id: ArtifactId(format!("artifact-{node_id}")),
        source: SourceSpan {
            byte_range: ByteRange { start: 0, end: 1 },
            line_column_range: LineColumnRange {
                start: LineColumn { line: 1, column: 1 },
                end: LineColumn { line: 1, column: 2 },
            },
            raw: RawSnippet {
                text: format!("source-{node_id}"),
            },
        },
        rect: crate::viewer::ViewerRect {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 24.0,
        },
    }
}
