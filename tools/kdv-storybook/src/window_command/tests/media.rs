use super::support::{media_control_button_label, media_control_target, storybook};
use crate::window_host_event::StorybookHostEvent;
use katana_document_viewer::{
    DiagramViewportState, ImageControlAction, ImageControlCommand, ViewerCommand,
    ViewerCommandFactory, ViewerMediaControlKind,
};

#[test]
fn image_control_command_refreshes_scene() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("direct/kdv-icon.png");
    storybook.update_scene(1000, 900)?;
    let target = media_control_target(&storybook, ViewerMediaControlKind::Image, "zoom-in")?;
    let zoom_command = ViewerCommandFactory::image_control_from_action(target.clone(), "zoom-in")
        .ok_or_else(|| std::io::Error::other("missing image command"))?;

    assert!(storybook.apply_viewer_command(&zoom_command));

    let state = storybook
        .image_viewports
        .get(target.node_id.0.as_str())
        .ok_or_else(|| std::io::Error::other("missing image viewport"))?;
    assert!(state.zoom > 1.0);
    assert!(
        storybook.scene.is_none(),
        "image zoom must invalidate scene so KUC receives the new transform"
    );

    storybook.image_viewports.insert(
        target.node_id.0.clone(),
        DiagramViewportState {
            zoom: 2.0,
            ..DiagramViewportState::default()
        },
    );

    assert!(
        storybook.apply_viewer_command(&ViewerCommand::Image(ImageControlCommand {
            target: target.clone(),
            action: ImageControlAction::Fit,
        },))
    );

    assert_eq!(
        Some(&DiagramViewportState::default()),
        storybook.image_viewports.get(target.node_id.0.as_str())
    );

    storybook.update_scene(1000, 900)?;
    let target = media_control_target(&storybook, ViewerMediaControlKind::Image, "copy")?;
    let command = ViewerCommandFactory::image_control_from_action(target, "copy")
        .ok_or_else(|| std::io::Error::other("missing image command"))?;

    assert!(storybook.apply_viewer_command(&command));

    assert!(
        storybook.scene.is_some(),
        "image copy must not trigger a preview reload"
    );
    assert_eq!("image:copy", storybook.last_command_label);
    Ok(())
}

#[test]
fn image_control_non_reloading_commands_cover_open_copy_and_reveal()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("direct/kdv-icon.png");
    storybook.update_scene(1000, 900)?;

    for (action, expected_label) in [
        ("open", "image:open"),
        ("copy", "image:copy"),
        ("reveal-in-os", "image:reveal-in-os"),
    ] {
        let target = media_control_target(&storybook, ViewerMediaControlKind::Image, action)?;
        let command = ViewerCommandFactory::image_control_from_action(target, action)
            .ok_or_else(|| std::io::Error::other("missing image command"))?;

        assert!(storybook.apply_viewer_command(&command));
        assert!(
            storybook.scene.is_some(),
            "{action} must not rebuild direct image scene"
        );
        assert_eq!(expected_label, storybook.last_command_label);
    }
    Ok(())
}

#[test]
fn image_control_zoom_out_refreshes_scene() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("direct/kdv-icon.png");
    storybook.update_scene(1000, 900)?;
    let target = media_control_target(&storybook, ViewerMediaControlKind::Image, "zoom-out")?;
    storybook.image_viewports.insert(
        target.node_id.0.clone(),
        DiagramViewportState {
            zoom: 2.0,
            ..DiagramViewportState::default()
        },
    );
    let command = ViewerCommandFactory::image_control_from_action(target.clone(), "zoom-out")
        .ok_or_else(|| std::io::Error::other("missing image command"))?;

    assert!(storybook.apply_viewer_command(&command));

    let state = storybook
        .image_viewports
        .get(target.node_id.0.as_str())
        .ok_or_else(|| std::io::Error::other("missing image viewport"))?;
    assert!(state.zoom < 2.0);
    assert!(
        storybook.scene.is_none(),
        "image zoom-out must invalidate scene so KUC receives the new transform"
    );
    Ok(())
}

#[test]
fn code_copy_host_command_refreshes_scene() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("katana/sample_basic.md");
    storybook.update_scene(1000, 900)?;
    let target = media_control_target(&storybook, ViewerMediaControlKind::Code, "copy-code")?;
    let command = ViewerCommandFactory::code_control_from_action(target.clone(), "copy-code")
        .ok_or_else(|| std::io::Error::other("missing code host command"))?;

    assert!(storybook.apply_viewer_command(&command));
    assert!(storybook.copied_code_node_ids.contains(&target.node_id.0));
    assert!(
        storybook.scene.is_none(),
        "code copy must invalidate scene so KUC receives copied state"
    );
    assert_eq!("host", storybook.last_command_label);

    storybook.update_scene(1000, 900)?;
    assert_eq!(
        "✓",
        media_control_button_label(
            &storybook,
            ViewerMediaControlKind::Code,
            "copy-code",
            &target.node_id.0,
        )?
    );
    Ok(())
}

#[test]
fn code_copy_keeps_pending_asset_job() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("katana/sample.md");
    storybook.update_scene(1000, 900)?;
    let pending_key = storybook
        .asset_job
        .as_ref()
        .ok_or("asset job missing")?
        .key()
        .clone();
    let target = media_control_target(&storybook, ViewerMediaControlKind::Code, "copy-code")?;
    let command = ViewerCommandFactory::code_control_from_action(target, "copy-code")
        .ok_or_else(|| std::io::Error::other("missing code host command"))?;

    assert!(storybook.apply_viewer_command(&command));

    let preserved_key = storybook
        .asset_job
        .as_ref()
        .ok_or("asset job must stay pending after code copy")?
        .key()
        .clone();
    assert_eq!(pending_key, preserved_key);
    assert!(
        storybook.scene.is_none(),
        "code copy still rebuilds scene so KUC receives copied state"
    );
    Ok(())
}

#[test]
fn diagram_control_keeps_pending_asset_job() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("katana/sample.md");
    storybook.update_scene(1000, 900)?;
    let pending_key = storybook
        .asset_job
        .as_ref()
        .ok_or("asset job missing")?
        .key()
        .clone();
    let target = media_control_target(&storybook, ViewerMediaControlKind::Diagram, "fullscreen")?;
    let command =
        ViewerCommandFactory::diagram_control_from_action(target.clone(), "fullscreen", false)
            .ok_or_else(|| std::io::Error::other("missing diagram command"))?;

    assert!(storybook.apply_viewer_command(&command));

    let state = storybook
        .diagram_viewports
        .get(target.node_id.0.as_str())
        .ok_or("diagram viewport state missing")?;
    assert!(state.fullscreen_open);
    let preserved_key = storybook
        .asset_job
        .as_ref()
        .ok_or("asset job must stay pending after diagram control")?
        .key()
        .clone();
    assert_eq!(pending_key, preserved_key);
    assert!(
        storybook.scene.is_none(),
        "diagram control still rebuilds scene so KUC receives viewport state"
    );
    Ok(())
}

#[test]
fn only_diagram_fullscreen_records_host_event() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("katana/sample.md");
    storybook.update_scene(1000, 900)?;
    let fullscreen_target =
        media_control_target(&storybook, ViewerMediaControlKind::Diagram, "fullscreen")?;
    let fullscreen = ViewerCommandFactory::diagram_control_from_action(
        fullscreen_target.clone(),
        "fullscreen",
        false,
    )
    .ok_or_else(|| std::io::Error::other("missing fullscreen command"))?;

    assert!(storybook.apply_viewer_command(&fullscreen));
    assert_eq!(
        vec![StorybookHostEvent::DiagramFullscreen {
            node_id: fullscreen_target.node_id.0.clone(),
            open: true,
        }],
        storybook.host_events
    );

    storybook.host_events.clear();
    let close = ViewerCommandFactory::diagram_control_from_action(
        fullscreen_target.clone(),
        "fullscreen",
        true,
    )
    .ok_or_else(|| std::io::Error::other("missing fullscreen close command"))?;

    assert!(storybook.apply_viewer_command(&close));
    assert_eq!(
        vec![StorybookHostEvent::DiagramFullscreen {
            node_id: fullscreen_target.node_id.0.clone(),
            open: false,
        }],
        storybook.host_events
    );

    storybook.host_events.clear();
    for action in [
        "pan-up",
        "pan-down",
        "pan-left",
        "pan-right",
        "zoom-in",
        "zoom-out",
        "reset-view",
        "trackpad-help",
    ] {
        let command = ViewerCommandFactory::diagram_control_from_action(
            fullscreen_target.clone(),
            action,
            true,
        )
        .ok_or_else(|| std::io::Error::other("missing diagram command"))?;

        assert!(storybook.apply_viewer_command(&command), "{action}");
        assert!(
            storybook.host_events.is_empty(),
            "{action} must stay inside the diagram control layer"
        );
    }
    Ok(())
}
