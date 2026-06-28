use crate::preview_interaction_command_metadata_support::{
    SceneActionTarget, collect_action_targets,
};
use crate::preview_interaction_command_support::build_scene;
use katana_document_viewer::{
    DiagramControlCommand, DiagramPanSource, DiagramZoomSource, ImageControlAction, ViewerCommand,
    ViewerCommandFactory, ViewerInteractionConfig, ViewerMediaControlKind, ViewerTarget,
};
use std::collections::BTreeSet;

#[test]
fn viewer_media_actions_resolve_real_viewer_targets() -> Result<(), Box<dyn std::error::Error>> {
    assert_image_targets()?;
    assert_diagram_targets()?;
    Ok(())
}

#[test]
fn math_scene_does_not_emit_image_or_diagram_controls() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene(
        "katana/sample_basic.md",
        ViewerInteractionConfig {
            image_controls_enabled: true,
            diagram_controls_enabled: true,
            code_controls_enabled: true,
            ..ViewerInteractionConfig::default()
        },
    )?;

    assert_math_target_exists(&scene.targets)?;
    assert!(
        collect_action_targets(&scene, ViewerMediaControlKind::Image).is_empty(),
        "math-only media fixture must not expose image controls"
    );
    assert!(
        collect_action_targets(&scene, ViewerMediaControlKind::Diagram).is_empty(),
        "math-only media fixture must not expose diagram controls"
    );
    let code_targets = collect_action_targets(&scene, ViewerMediaControlKind::Code);
    assert!(
        !code_targets.is_empty(),
        "fixture must still expose code copy controls for real code blocks"
    );
    assert_no_math_control_targets(&code_targets)?;
    Ok(())
}

fn assert_image_targets() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene(
        "direct/kdv-icon.png",
        ViewerInteractionConfig {
            image_controls_enabled: true,
            ..ViewerInteractionConfig::default()
        },
    )?;
    let targets = collect_action_targets(&scene, ViewerMediaControlKind::Image);

    assert!(!targets.is_empty());
    assert_unique_action_target_pairs(&targets);
    for action in targets {
        assert_image_command_roundtrip(action)?;
    }
    Ok(())
}

fn assert_diagram_targets() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene(
        "katana/sample_diagrams.md",
        ViewerInteractionConfig {
            diagram_controls_enabled: true,
            ..ViewerInteractionConfig::default()
        },
    )?;
    let targets = collect_action_targets(&scene, ViewerMediaControlKind::Diagram);

    assert!(!targets.is_empty());
    assert_unique_action_target_pairs(&targets);
    assert_multiple_diagram_targets(&targets);
    for action in targets {
        assert_diagram_command_roundtrip(action)?;
    }
    Ok(())
}

fn assert_image_command_roundtrip(
    action: SceneActionTarget,
) -> Result<(), Box<dyn std::error::Error>> {
    assert_real_target(&action.target);
    let command =
        ViewerCommandFactory::image_control_from_action(action.target.clone(), &action.action)
            .ok_or_else(|| {
                std::io::Error::other(format!("missing image action: {}", action.action))
            })?;
    let ViewerCommand::Image(image) = command else {
        return Err(std::io::Error::other("expected image command").into());
    };
    assert_eq!(action.target, image.target);
    assert_eq!(expected_image_action(&action.action)?, image.action);
    Ok(())
}

fn assert_real_target(target: &ViewerTarget) {
    assert_ne!("storybook-interaction-node", target.node_id.0);
    assert!(!target.source.raw.text.is_empty());
    assert!(target.rect.width > 0.0);
    assert!(target.rect.height > 0.0);
}

fn assert_diagram_command_roundtrip(
    action: SceneActionTarget,
) -> Result<(), Box<dyn std::error::Error>> {
    assert_real_target(&action.target);
    let command = ViewerCommandFactory::diagram_control_from_action(
        action.target.clone(),
        &action.action,
        false,
    )
    .ok_or_else(|| std::io::Error::other(format!("missing diagram action: {}", action.action)))?;
    if action.action == "copy-source" {
        let ViewerCommand::Host(katana_document_viewer::HostCommand::CopyText(copy)) = command
        else {
            return Err(std::io::Error::other("expected diagram source copy command").into());
        };
        assert_eq!(
            katana_document_viewer::CopyTextSource::DiagramSource,
            copy.source
        );
        assert_eq!(action.target, copy.target);
        assert_eq!(action.target.source.raw.text, copy.text);
        return Ok(());
    }
    let ViewerCommand::Diagram(command) = command else {
        return Err(std::io::Error::other("expected diagram command").into());
    };
    assert_expected_diagram_command(command, &action)
}

fn assert_expected_diagram_command(
    command: DiagramControlCommand,
    action: &SceneActionTarget,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        DiagramControlCommand::FullscreenOpen(target) if action.action == "fullscreen" => {
            assert_eq!(action.target, target);
        }
        DiagramControlCommand::Reset(target) if action.action == "reset-view" => {
            assert_eq!(action.target, target);
        }
        DiagramControlCommand::TrackpadHelp(target) if action.action == "trackpad-help" => {
            assert_eq!(action.target, target);
        }
        DiagramControlCommand::Pan(command) => {
            assert_eq!(action.target, command.target);
            assert_eq!(expected_pan_source(&action.action)?, command.source);
        }
        DiagramControlCommand::Zoom(command) => {
            assert_eq!(action.target, command.target);
            assert_eq!(expected_zoom_source(&action.action)?, command.source);
        }
        _ => return Err(std::io::Error::other("unexpected diagram command").into()),
    }
    Ok(())
}

fn assert_unique_action_target_pairs(actions: &[SceneActionTarget]) {
    let mut seen = BTreeSet::new();
    for action in actions {
        assert!(
            seen.insert((action.target.node_id.0.clone(), action.action.clone())),
            "{action:#?}"
        );
    }
}

fn assert_multiple_diagram_targets(actions: &[SceneActionTarget]) {
    let node_ids = actions
        .iter()
        .map(|action| action.target.node_id.0.as_str())
        .collect::<BTreeSet<_>>();
    assert!(node_ids.len() >= 2, "{node_ids:#?}");
}

fn assert_math_target_exists(targets: &[ViewerTarget]) -> Result<(), Box<dyn std::error::Error>> {
    if targets
        .iter()
        .any(|target| target.source.raw.text.contains(r"\frac"))
    {
        return Ok(());
    }
    Err(std::io::Error::other("expected math source target").into())
}

fn assert_no_math_control_targets(
    actions: &[SceneActionTarget],
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(action) = actions
        .iter()
        .find(|action| action.target.source.raw.text.contains(r"\frac"))
    {
        return Err(std::io::Error::other(format!(
            "math target must not expose media/code control action: {action:#?}"
        ))
        .into());
    }
    Ok(())
}

fn expected_image_action(action: &str) -> Result<ImageControlAction, Box<dyn std::error::Error>> {
    match action {
        "fit" => Ok(ImageControlAction::Fit),
        "open" => Ok(ImageControlAction::Open),
        "copy" => Ok(ImageControlAction::Copy),
        "reveal-in-os" => Ok(ImageControlAction::RevealInOs),
        "zoom-in" => Ok(ImageControlAction::ZoomIn),
        "zoom-out" => Ok(ImageControlAction::ZoomOut),
        _ => Err(std::io::Error::other(format!("unexpected image action: {action}")).into()),
    }
}

fn expected_pan_source(action: &str) -> Result<DiagramPanSource, Box<dyn std::error::Error>> {
    match action {
        "pan-up" => Ok(DiagramPanSource::ButtonUp),
        "pan-down" => Ok(DiagramPanSource::ButtonDown),
        "pan-left" => Ok(DiagramPanSource::ButtonLeft),
        "pan-right" => Ok(DiagramPanSource::ButtonRight),
        _ => Err(std::io::Error::other(format!("unexpected pan action: {action}")).into()),
    }
}

fn expected_zoom_source(action: &str) -> Result<DiagramZoomSource, Box<dyn std::error::Error>> {
    match action {
        "zoom-in" => Ok(DiagramZoomSource::ButtonIn),
        "zoom-out" => Ok(DiagramZoomSource::ButtonOut),
        _ => Err(std::io::Error::other(format!("unexpected zoom action: {action}")).into()),
    }
}
