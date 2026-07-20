use super::{coalesce_command, merge_command, receive_command};
use crate::browser_session::browser_session_types::BrowserSessionCommand;
use katana_render_runtime::{HtmlBrowserInput, HtmlBrowserViewport};
use std::sync::mpsc;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn adjacent_scroll_commands_are_coalesced_without_reordering_pointer_input() -> TestResult {
    let (sender, receiver) = mpsc::sync_channel(4);
    sender.send(scroll(2.0, 3.0))?;
    sender.send(scroll(5.0, 7.0))?;
    sender.send(BrowserSessionCommand::Input(
        HtmlBrowserInput::PointerDown {
            x: 10.0,
            y: 20.0,
            button: 0,
        },
    ))?;

    let first = receive_command(&receiver, None).ok_or("missing first command")?;
    let (command, pending) = coalesce_command(first, &receiver);

    assert_scroll(command, 7.0, 10.0)?;
    assert!(matches!(
        pending,
        Some(BrowserSessionCommand::Input(
            HtmlBrowserInput::PointerDown { .. }
        ))
    ));
    Ok(())
}

#[test]
fn adjacent_resize_commands_keep_only_the_latest_viewport() -> TestResult {
    let initial = HtmlBrowserViewport::new(320, 240, 1.0)?;
    let mut command = BrowserSessionCommand::Resize(initial);
    let latest = HtmlBrowserViewport::new(640, 480, 2.0)?;

    merge_command(&mut command, BrowserSessionCommand::Resize(latest))
        .map_err(|_| "resize commands did not merge")?;

    assert!(matches!(command, BrowserSessionCommand::Resize(value) if value == latest));
    Ok(())
}

#[test]
fn pending_command_is_received_before_the_channel_and_disconnect_returns_none() -> TestResult {
    let (sender, receiver) = mpsc::sync_channel(1);
    let pending = BrowserSessionCommand::Refresh;

    assert!(matches!(
        receive_command(&receiver, Some(pending)),
        Some(BrowserSessionCommand::Refresh)
    ));
    drop(sender);
    assert!(receive_command(&receiver, None).is_none());
    Ok(())
}

fn scroll(delta_x: f32, delta_y: f32) -> BrowserSessionCommand {
    BrowserSessionCommand::Input(HtmlBrowserInput::Scroll { delta_x, delta_y })
}

fn assert_scroll(command: BrowserSessionCommand, expected_x: f32, expected_y: f32) -> TestResult {
    match command {
        BrowserSessionCommand::Input(HtmlBrowserInput::Scroll { delta_x, delta_y }) => {
            assert_eq!((delta_x, delta_y), (expected_x, expected_y));
            Ok(())
        }
        _ => Err("expected a coalesced scroll command".into()),
    }
}
