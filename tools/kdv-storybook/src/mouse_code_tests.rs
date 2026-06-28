use super::StorybookMouse;
use super::mouse_test_support::{
    WINDOW_HEIGHT, WINDOW_WIDTH, pointer_for_media_action, sample_basic_scene,
};
use katana_document_viewer::{CopyTextSource, HostCommand, ViewerCommand};

#[test]
fn mouse_left_click_on_code_copy_returns_host_copy_command()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_basic_scene()?;
    let hit = pointer_for_media_action(&scene, "copy-code")?;

    let command = StorybookMouse::command_for_click(
        &scene,
        hit.scroll_y,
        hit.pointer,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .ok_or_else(|| std::io::Error::other("missing code copy command"))?;

    let ViewerCommand::Host(HostCommand::CopyText(copy)) = command else {
        return Err(std::io::Error::other("expected code copy command").into());
    };
    assert_eq!(CopyTextSource::Code, copy.source);
    assert_eq!(copy.target.source.raw.text, copy.text);
    assert!(!copy.text.trim().is_empty());
    Ok(())
}
