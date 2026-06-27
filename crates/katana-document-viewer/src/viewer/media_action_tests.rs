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
