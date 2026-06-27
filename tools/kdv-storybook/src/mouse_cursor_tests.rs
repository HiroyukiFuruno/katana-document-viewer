use super::StorybookMouseCursor;
use super::mouse_test_support::{
    WINDOW_HEIGHT, WINDOW_WIDTH, pointer_for_accordion, pointer_for_internal_diagram_action,
    pointer_for_link, pointer_for_media_action, sample_basic_scene, sample_diagram_controls_scene,
};
use crate::KucDiagramControlResolver;
use katana_ui_core::render_model::UiCursor;

#[test]
fn link_hover_uses_pointer_cursor_from_kuc_node() -> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_basic_scene()?;
    let hit = pointer_for_link(&scene, "Normal link", "https://github.com")?;

    let cursor = StorybookMouseCursor::cursor_for_hover(
        &scene,
        hit.scroll_y,
        hit.pointer.x,
        hit.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );

    assert_eq!(UiCursor::Pointer, cursor);
    Ok(())
}

#[test]
fn link_hover_resolves_viewer_target_for_block_hover() -> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_basic_scene()?;
    let hit = pointer_for_link(&scene, "Normal link", "https://github.com")?;

    let hovered_node_id = StorybookMouseCursor::hovered_node_id_for_hover(
        &scene,
        hit.scroll_y,
        hit.pointer.x,
        hit.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );

    assert!(
        hovered_node_id.as_ref().is_some_and(|node_id| scene
            .targets
            .iter()
            .any(|target| target.node_id.0 == *node_id)),
        "link hover must resolve to a KDV viewer target node id"
    );
    Ok(())
}

#[test]
fn sidebar_area_does_not_resolve_document_host_action_hover()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_basic_scene()?;

    let cursor = StorybookMouseCursor::cursor_for_hover(
        &scene,
        0.0,
        24.0,
        400.0,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );
    let hovered_action_node_id = StorybookMouseCursor::hovered_action_node_id_for_hover(
        &scene,
        0.0,
        24.0,
        400.0,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );

    assert_eq!(UiCursor::Default, cursor);
    assert_eq!(None, hovered_action_node_id);
    Ok(())
}

#[test]
fn media_control_hover_uses_pointer_cursor() -> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_diagram_controls_scene()?;
    let hit = pointer_for_media_action(&scene, "fullscreen")?;

    let cursor = StorybookMouseCursor::cursor_for_hover(
        &scene,
        hit.scroll_y,
        hit.pointer.x,
        hit.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );

    assert_eq!(UiCursor::Pointer, cursor);
    Ok(())
}

#[test]
fn internal_diagram_control_hover_resolves_kuc_control_node_for_border()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_diagram_controls_scene()?;
    let hit = pointer_for_internal_diagram_action(&scene, "zoom-in")?;

    let hovered_action_node_id = StorybookMouseCursor::hovered_action_node_id_for_hover(
        &scene,
        hit.scroll_y,
        hit.pointer.x,
        hit.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .ok_or_else(|| {
        let hovered_node_id = StorybookMouseCursor::hovered_node_id_for_hover(
            &scene,
            hit.scroll_y,
            hit.pointer.x,
            hit.pointer.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        );
        let hovered_node_action = hovered_node_id
            .as_ref()
            .and_then(|node_id| {
                KucDiagramControlResolver::internal_action_for_node(
                    scene.tree.root(),
                    &katana_ui_core::render_model::UiNodeId::new(node_id.as_str()),
                )
            })
            .map(|action| action.command.to_string());
        std::io::Error::other(format!(
            "missing internal diagram hover node: pointer=({:.1},{:.1}) scroll_y={:.1} hovered_node_id={:?} hovered_node_action={:?}",
            hit.pointer.x, hit.pointer.y, hit.scroll_y, hovered_node_id, hovered_node_action
        ))
    })?;
    let action = KucDiagramControlResolver::internal_action_for_node(
        scene.tree.root(),
        &hovered_action_node_id,
    )
    .ok_or_else(|| std::io::Error::other("hover node is not an internal diagram control"))?;

    assert_eq!("zoom-in", action.command);
    Ok(())
}

#[test]
fn accordion_header_hover_uses_pointer_cursor() -> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_basic_scene()?;
    let hit = pointer_for_accordion(&scene)?;

    let cursor = StorybookMouseCursor::cursor_for_hover(
        &scene,
        hit.scroll_y,
        hit.pointer.x,
        hit.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );

    assert_eq!(UiCursor::Pointer, cursor);
    Ok(())
}

#[test]
fn accordion_hover_resolves_viewer_target_for_block_hover() -> Result<(), Box<dyn std::error::Error>>
{
    let scene = sample_basic_scene()?;
    let hit = pointer_for_accordion(&scene)?;

    let hovered_node_id = StorybookMouseCursor::hovered_node_id_for_hover(
        &scene,
        hit.scroll_y,
        hit.pointer.x,
        hit.pointer.y,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );

    assert!(
        hovered_node_id.as_ref().is_some_and(|node_id| scene
            .targets
            .iter()
            .any(|target| target.node_id.0 == *node_id)),
        "accordion hover must resolve to a KDV viewer target node id"
    );
    Ok(())
}
