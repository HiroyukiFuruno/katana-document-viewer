use super::browser_session_types::BrowserSessionCommand;
use katana_render_runtime::HtmlBrowserInput;
use std::sync::mpsc::Receiver;

pub(super) fn receive_command(
    commands: &Receiver<BrowserSessionCommand>,
    pending: Option<BrowserSessionCommand>,
) -> Option<BrowserSessionCommand> {
    pending.or_else(|| commands.recv().ok())
}

pub(super) fn coalesce_command(
    mut command: BrowserSessionCommand,
    commands: &Receiver<BrowserSessionCommand>,
) -> (BrowserSessionCommand, Option<BrowserSessionCommand>) {
    while let Ok(next) = commands.try_recv() {
        match merge_command(&mut command, next) {
            Ok(()) => {}
            Err(next) => return (command, Some(next)),
        }
    }
    (command, None)
}

fn merge_command(
    command: &mut BrowserSessionCommand,
    next: BrowserSessionCommand,
) -> Result<(), BrowserSessionCommand> {
    match (command, next) {
        (
            BrowserSessionCommand::Input(HtmlBrowserInput::Scroll { delta_x, delta_y }),
            BrowserSessionCommand::Input(HtmlBrowserInput::Scroll {
                delta_x: next_x,
                delta_y: next_y,
            }),
        ) => {
            *delta_x += next_x;
            *delta_y += next_y;
            Ok(())
        }
        (command @ BrowserSessionCommand::Resize(_), BrowserSessionCommand::Resize(viewport)) => {
            *command = BrowserSessionCommand::Resize(viewport);
            Ok(())
        }
        (_, next) => Err(next),
    }
}

#[cfg(test)]
#[path = "browser_session_worker_coalescing_tests.rs"]
mod tests;
