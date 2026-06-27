use crate::preview_interaction_command_support::{
    build_scene, build_search_scene, collect_code_actions, collect_diagram_actions,
    collect_image_actions, collect_link_targets, collect_task_markers, search_state, target,
};
use katana_document_viewer::{
    ViewerCommand, ViewerCommandFactory, ViewerInteractionConfig, ViewerSearchDirection,
    ViewerTaskState, ViewerTocItem,
};
use katana_ui_core::render_model::UiTaskMarker;

#[test]
fn kuc_scene_task_actions_reach_kdv_task_commands() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene("katana/sample.md", ViewerInteractionConfig::default())?;
    let markers = collect_task_markers(scene.tree.root());

    assert!(markers.contains(&UiTaskMarker::Empty));
    assert!(markers.contains(&UiTaskMarker::Done));
    assert!(markers.contains(&UiTaskMarker::Progress));
    assert!(markers.contains(&UiTaskMarker::Blocked));
    for marker in markers {
        assert!(matches!(
            ViewerCommandFactory::set_task_state(target(), viewer_task_state(marker)),
            ViewerCommand::Task(_)
        ));
    }
    assert_task_toggle_command()
}

#[test]
fn kuc_scene_media_actions_reach_kdv_media_commands() -> Result<(), Box<dyn std::error::Error>> {
    assert_image_actions()?;
    assert_diagram_actions()?;
    assert_code_actions()?;
    Ok(())
}

#[test]
fn kuc_scene_link_spans_reach_kdv_link_commands() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene(
        "direct/html-alignment.html",
        ViewerInteractionConfig::default(),
    )?;
    let links = collect_link_targets(scene.tree.root());

    assert!(
        links
            .iter()
            .any(|value| value == "https://example.com/docs")
    );
    for link in links {
        assert_link_command(&link)?;
    }
    Ok(())
}

#[test]
fn kuc_scene_search_targets_reach_kdv_search_command() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_search_scene("direct/sample.md", "Direct")?;
    let state = search_state(scene.search_targets);

    let command = ViewerCommandFactory::navigate_search(&state, ViewerSearchDirection::Next);

    let Some(ViewerCommand::Search(search)) = command else {
        return Err(std::io::Error::other("expected search command").into());
    };
    assert_eq!(ViewerSearchDirection::Next, search.direction);
    assert_eq!(state.matches[0], search.target);
    assert_eq!(state.matches[0].rect, search.scroll.target.rect);
    Ok(())
}

#[test]
fn kuc_scene_target_reaches_kdv_toc_scroll_command() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene("katana/sample.md", ViewerInteractionConfig::default())?;
    let Some(target) = scene.targets.first().cloned() else {
        return Err(std::io::Error::other("expected scene target").into());
    };
    let item = ViewerTocItem {
        node_id: target.node_id.clone(),
        level: 1,
        text: "Storybook TOC".to_string(),
        source: target.source.clone(),
        anchor_rect: target.rect,
        anchor_index: 0,
    };

    let command = ViewerCommandFactory::scroll_to_toc_item(item);

    let ViewerCommand::ScrollToHeading(scroll) = command else {
        return Err(std::io::Error::other("expected TOC scroll command").into());
    };
    assert_eq!(target.node_id, scroll.target.node_id);
    assert_eq!(target.source, scroll.target.source);
    assert_eq!(target.rect, scroll.target.rect);
    Ok(())
}

#[test]
fn slideshow_controls_reach_kdv_slideshow_commands() {
    assert_eq!(
        ViewerCommand::Slideshow(katana_document_viewer::SlideshowCommand::NextPage),
        ViewerCommandFactory::next_slideshow_page()
    );
    assert_eq!(
        ViewerCommand::Slideshow(katana_document_viewer::SlideshowCommand::PreviousPage),
        ViewerCommandFactory::previous_slideshow_page()
    );
    assert_eq!(
        ViewerCommand::Slideshow(katana_document_viewer::SlideshowCommand::Close),
        ViewerCommandFactory::close_slideshow()
    );
}

fn assert_image_actions() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene(
        "direct/kdv-icon.png",
        ViewerInteractionConfig {
            image_controls_enabled: true,
            ..ViewerInteractionConfig::default()
        },
    )?;
    let actions = collect_image_actions(scene.tree.root());
    for expected in ["fit", "open", "copy", "reveal-in-os", "zoom-in", "zoom-out"] {
        assert!(actions.iter().any(|value| value == expected), "{expected}");
    }
    for action in actions {
        assert!(ViewerCommandFactory::image_control_from_action(target(), &action).is_some());
    }
    Ok(())
}

fn assert_diagram_actions() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene(
        "katana/sample_diagrams.md",
        ViewerInteractionConfig {
            diagram_controls_enabled: true,
            ..ViewerInteractionConfig::default()
        },
    )?;
    let actions = collect_diagram_actions(scene.tree.root());
    for expected in diagram_action_values() {
        assert!(actions.iter().any(|value| value == expected), "{expected}");
    }
    assert!(
        actions
            .iter()
            .all(|action| diagram_action_values().contains(&action.as_str())),
        "KDV host must only receive diagram host controls; pan/zoom/reset stay inside KUC: {actions:?}"
    );
    for action in actions {
        assert!(
            ViewerCommandFactory::diagram_control_from_action(target(), &action, false).is_some()
        );
    }
    Ok(())
}

fn assert_code_actions() -> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene("katana/sample_basic.md", ViewerInteractionConfig::default())?;
    let actions = collect_code_actions(scene.tree.root());

    assert!(actions.iter().any(|value| value == "copy-code"));
    for action in actions {
        assert!(ViewerCommandFactory::code_control_from_action(target(), &action).is_some());
    }
    Ok(())
}

fn diagram_action_values() -> [&'static str; 1] {
    ["fullscreen"]
}

fn assert_link_command(uri: &str) -> Result<(), Box<dyn std::error::Error>> {
    let command = ViewerCommandFactory::open_link(target(), uri);
    let ViewerCommand::Link(link) = command else {
        return Err(std::io::Error::other("expected link command").into());
    };
    assert_eq!(uri, link.uri);
    Ok(())
}

fn assert_task_toggle_command() -> Result<(), Box<dyn std::error::Error>> {
    let command = ViewerCommandFactory::toggle_task(target(), ViewerTaskState::Empty);
    let ViewerCommand::Task(task) = command else {
        return Err(std::io::Error::other("expected task command").into());
    };
    assert_eq!(ViewerTaskState::Done, task.state);
    Ok(())
}

fn viewer_task_state(marker: UiTaskMarker) -> ViewerTaskState {
    match marker {
        UiTaskMarker::Empty => ViewerTaskState::Empty,
        UiTaskMarker::Done => ViewerTaskState::Done,
        UiTaskMarker::Progress => ViewerTaskState::Progress,
        UiTaskMarker::Blocked => ViewerTaskState::Blocked,
    }
}
