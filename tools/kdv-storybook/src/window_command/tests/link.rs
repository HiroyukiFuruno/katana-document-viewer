use super::support::{first_target_containing, storybook};
use crate::layout::preview_viewport_height;
use katana_document_viewer::{ViewerCommand, ViewerCommandFactory};

#[test]
fn link_command_rejects_external_links_without_scroll() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("katana/sample_basic.md");
    storybook.update_scene(1000, 900)?;
    let target = first_target_containing(&storybook, "Normal link")?;

    let command = ViewerCommandFactory::open_link(target.clone(), "https://example.com");

    let previous_scroll = storybook.scroll_y;
    assert!(storybook.apply_viewer_command(&command));
    assert_eq!(previous_scroll, storybook.scroll_y);
    assert_eq!("open-uri:https://example.com", storybook.last_command_label);

    let ViewerCommand::Link(command) = command else {
        return Err(std::io::Error::other("expected link command").into());
    };
    assert_eq!(target.artifact_id, command.target.artifact_id);

    Ok(())
}

#[test]
fn link_command_with_internal_anchor_scrolls_to_footnote() -> Result<(), Box<dyn std::error::Error>>
{
    let mut storybook = storybook("katana/sample_basic.md");
    storybook.update_scene(1000, 900)?;
    let target = first_target_containing(&storybook, "Normal link")?;
    let footnote = storybook
        .scene
        .as_ref()
        .and_then(|scene| scene.target_for_internal_anchor("#fn-1"))
        .ok_or_else(|| std::io::Error::other("missing internal anchor target"))?;
    let footnote_y = footnote.rect.y;

    let command = ViewerCommandFactory::open_link(target, "#fn-1");

    assert!(storybook.apply_viewer_command(&command));

    let viewport_height = preview_viewport_height(900) as f32;
    let scene = storybook.scene.as_ref().ok_or("scene missing")?;
    let max_scroll_y = (scene.content_height - viewport_height).max(0.0);
    let expected = footnote_y.min(max_scroll_y).max(0.0);
    assert_eq!(expected, storybook.scroll_y);

    Ok(())
}

#[test]
fn scroll_to_heading_command_scrolls_to_target() -> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("katana/sample_basic.md");
    storybook.update_scene(1000, 900)?;
    let footnote = first_target_containing(&storybook, "First footnote content")?;
    let footnote_y = footnote.rect.y;
    let command = ViewerCommandFactory::scroll_to_target(footnote.clone());

    assert!(storybook.apply_viewer_command(&command));

    let viewport_height = preview_viewport_height(900) as f32;
    let scene = storybook.scene.as_ref().ok_or("scene missing")?;
    let max_scroll_y = (scene.content_height - viewport_height).max(0.0);
    let expected = footnote_y.min(max_scroll_y).max(0.0);
    assert_eq!(expected, storybook.scroll_y);

    Ok(())
}
