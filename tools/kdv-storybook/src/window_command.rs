use super::StorybookWindow;
use crate::layout::preview_viewport_height;
use crate::sidebar_settings_task_change::StorybookTaskStateChangeInput;
use crate::window_host_event::StorybookHostEvent;
use katana_document_viewer::{
    CopyTextCommand, CopyTextSource, DiagramControlCommand, HostCommand, ImageControlAction,
    ImageControlCommand, SlideshowCommand, ViewerCommand, ViewerMode, ViewerScrollCommand,
    ViewerStateEngine, ViewerTarget, ViewerTaskControlTarget, ViewerTaskState,
};
#[cfg(all(not(test), any(target_os = "macos", target_os = "linux")))]
use std::io::Write;
#[cfg(all(not(test), any(target_os = "macos", target_os = "linux")))]
use std::process::{Command, Stdio};

impl StorybookWindow {
    pub(super) fn apply_viewer_command(&mut self, command: &ViewerCommand) -> bool {
        match command {
            ViewerCommand::ScrollToHeading(scroll) => self.apply_scroll_command(scroll),
            ViewerCommand::Link(link) => self.apply_link_command(&link.uri),
            ViewerCommand::Task(task) => {
                self.apply_task_state_command(&task.target, task.task_target.as_ref(), task.state)
            }
            ViewerCommand::Diagram(diagram) => self.apply_diagram_command(diagram),
            ViewerCommand::Image(image) => self.apply_image_command(image),
            ViewerCommand::Slideshow(slideshow) => self.apply_slideshow_command(slideshow),
            ViewerCommand::Host(host) => self.apply_host_command(host),
            _ => false,
        }
    }

    fn apply_scroll_command(&mut self, command: &ViewerScrollCommand) -> bool {
        let window_height = self
            .frame_size
            .map_or(self.args.height, |(_, height)| height);
        let viewport_height = preview_viewport_height(window_height) as f32;
        let max_scroll_y = self.scene.as_ref().map_or(command.target.rect.y, |scene| {
            (scene.content_height - viewport_height).max(0.0)
        });
        self.scroll_y = command.target.rect.y.min(max_scroll_y).max(0.0);
        true
    }

    fn apply_link_command(&mut self, uri: &str) -> bool {
        if uri.starts_with('#')
            && let Some(target) = self
                .scene
                .as_ref()
                .and_then(|scene| scene.target_for_internal_anchor(uri))
        {
            let viewport_height = self
                .frame_size
                .map_or(self.args.height, |(_, height)| height);
            let max_scroll_y = self.scene.as_ref().map_or(target.rect.y, |scene| {
                (scene.content_height - preview_viewport_height(viewport_height) as f32).max(0.0)
            });
            self.scroll_y = target.rect.y.min(max_scroll_y).max(0.0);
            return true;
        }
        self.last_command_label = format!("open-uri:{uri}");
        true
    }

    fn apply_image_command(&mut self, command: &ImageControlCommand) -> bool {
        match command.action {
            ImageControlAction::ZoomIn | ImageControlAction::ZoomOut | ImageControlAction::Fit => {
                let key = command.target.node_id.0.clone();
                let current = self.image_viewports.get(&key).copied().unwrap_or_default();
                let next = ViewerStateEngine::apply_image_command(current, command);
                self.image_viewports.insert(key, next);
                self.invalidate_lazy_scene(false);
            }
            ImageControlAction::Copy
            | ImageControlAction::Open
            | ImageControlAction::RevealInOs => {
                self.last_command_label = format!("image:{}", image_action_label(command.action));
            }
        }
        true
    }

    fn apply_host_command(&mut self, command: &HostCommand) -> bool {
        match command {
            HostCommand::CopyText(copy) => self.apply_copy_text_command(copy),
            HostCommand::OpenUri(uri) => {
                self.last_command_label = format!("open-uri:{uri}");
                true
            }
            HostCommand::RevealPath(path) => {
                self.last_command_label = format!("reveal-path:{path}");
                true
            }
        }
    }

    fn apply_copy_text_command(&mut self, command: &CopyTextCommand) -> bool {
        if let Err(error) = write_clipboard_text(&command.text) {
            eprintln!("[kdv-storybook] clipboard write failed: {error}");
            return false;
        }
        self.last_command_label = "host".to_string();
        if command.source == CopyTextSource::Code {
            self.copied_code_node_ids
                .insert(command.target.node_id.0.clone());
            self.invalidate_lazy_scene_preserving_asset_job();
        }
        true
    }

    fn apply_task_state_command(
        &mut self,
        target: &ViewerTarget,
        task_target: Option<&ViewerTaskControlTarget>,
        state: ViewerTaskState,
    ) -> bool {
        let Some(scene) = self.scene.as_ref() else {
            return false;
        };
        let document_id = scene.document_id.clone();
        let state_id = task_target.map_or_else(
            || target.artifact_id.0.clone(),
            |target| target.state_id.clone(),
        );
        let previous_state = self
            .task_state_overrides
            .get(&state_id)
            .copied()
            .or_else(|| source_task_state(&target.source.raw.text));
        let change = StorybookTaskStateChangeInput {
            document_id: document_id.as_str(),
            target,
            previous_state,
            next_state: state,
        };
        self.task_state_overrides.insert(state_id, state);
        self.settings_state.record_task_change(change);
        self.invalidate_lazy_scene(false);
        true
    }

    fn apply_diagram_command(&mut self, command: &DiagramControlCommand) -> bool {
        let Some(target) = diagram_target(command) else {
            return false;
        };
        let key = target.node_id.0.clone();
        let current = self
            .diagram_viewports
            .get(&key)
            .copied()
            .unwrap_or_default();
        let next = ViewerStateEngine::apply_diagram_command(current, command);
        self.diagram_viewports.insert(key, next);
        if command.requires_host_propagation() {
            self.host_events
                .push(StorybookHostEvent::DiagramFullscreen {
                    node_id: target.node_id.0.clone(),
                    open: next.fullscreen_open,
                });
        }
        if self.current_scene_has_resolved_assets() {
            self.invalidate_loaded_scene();
        } else {
            self.invalidate_lazy_scene_preserving_asset_job();
        }
        true
    }

    fn apply_slideshow_command(&mut self, command: &SlideshowCommand) -> bool {
        match command {
            SlideshowCommand::PreviousPage => self.apply_slideshow_page_delta(-1),
            SlideshowCommand::NextPage => self.apply_slideshow_page_delta(1),
            SlideshowCommand::UpdateSettings(_) => false,
            SlideshowCommand::Close => {
                if self.mode != ViewerMode::Slideshow {
                    return false;
                }
                self.mode = ViewerMode::Document;
                self.scroll_y = 0.0;
                self.last_command_label = "slideshow".to_string();
                self.invalidate_lazy_scene(false);
                true
            }
        }
    }

    fn apply_slideshow_page_delta(&mut self, delta: isize) -> bool {
        if self.mode != ViewerMode::Slideshow {
            return false;
        }
        let Some(scene) = self.scene.as_ref() else {
            return false;
        };
        let window_height = self
            .frame_size
            .map_or(self.args.height, |(_, height)| height);
        let viewport_height = preview_viewport_height(window_height) as f32;
        let current = scene.slideshow_current_page as isize;
        let max_page = scene.slideshow_max_page as isize;
        let next_page = (current + delta).clamp(0, max_page) as usize;
        if next_page == scene.slideshow_current_page {
            return false;
        }
        self.scroll_y = next_page as f32 * viewport_height;
        self.last_command_label = "slideshow".to_string();
        self.apply_slideshow_page_scroll(viewport_height);
        true
    }
}

fn diagram_target(command: &DiagramControlCommand) -> Option<&ViewerTarget> {
    match command {
        DiagramControlCommand::FullscreenOpen(target)
        | DiagramControlCommand::FullscreenClose(target)
        | DiagramControlCommand::Reset(target)
        | DiagramControlCommand::TrackpadHelp(target) => Some(target),
        DiagramControlCommand::Pan(pan) => Some(&pan.target),
        DiagramControlCommand::Zoom(zoom) => Some(&zoom.target),
    }
}

fn source_task_state(source: &str) -> Option<ViewerTaskState> {
    [
        ViewerTaskState::Empty,
        ViewerTaskState::Done,
        ViewerTaskState::Progress,
        ViewerTaskState::Blocked,
    ]
    .into_iter()
    .find(|state| source.contains(state.marker()))
}

fn image_action_label(action: ImageControlAction) -> &'static str {
    match action {
        ImageControlAction::Copy => "copy",
        ImageControlAction::Open => "open",
        ImageControlAction::Fit => "fit",
        ImageControlAction::RevealInOs => "reveal-in-os",
        ImageControlAction::ZoomIn => "zoom-in",
        ImageControlAction::ZoomOut => "zoom-out",
    }
}

#[cfg(all(not(test), target_os = "macos"))]
const CLIPBOARD_READ_ARGS: &[&str] = &[];
#[cfg(all(not(test), target_os = "macos"))]
const CLIPBOARD_READ_COMMAND: &str = "/usr/bin/pbpaste";
#[cfg(all(not(test), target_os = "macos"))]
const CLIPBOARD_WRITE_ARGS: &[&str] = &[];
#[cfg(all(not(test), target_os = "macos"))]
const CLIPBOARD_WRITE_COMMAND: &str = "/usr/bin/pbcopy";
#[cfg(all(not(test), target_os = "linux"))]
const CLIPBOARD_READ_ARGS: &[&str] = &["-selection", "clipboard", "-out"];
#[cfg(all(not(test), target_os = "linux"))]
const CLIPBOARD_READ_COMMAND: &str = "xclip";
#[cfg(all(not(test), target_os = "linux"))]
const CLIPBOARD_WRITE_ARGS: &[&str] = &["-selection", "clipboard"];
#[cfg(all(not(test), target_os = "linux"))]
const CLIPBOARD_WRITE_COMMAND: &str = "xclip";

#[cfg(all(not(test), any(target_os = "macos", target_os = "linux")))]
pub(super) fn write_clipboard_text(text: &str) -> Result<(), std::io::Error> {
    let mut child = Command::new(CLIPBOARD_WRITE_COMMAND)
        .args(CLIPBOARD_WRITE_ARGS)
        .stdin(Stdio::piped())
        .spawn()?;
    let Some(stdin) = child.stdin.as_mut() else {
        return Err(std::io::Error::other(format!(
            "{CLIPBOARD_WRITE_COMMAND} stdin is unavailable"
        )));
    };
    stdin.write_all(text.as_bytes())?;
    let status = child.wait()?;
    if status.success() {
        return Ok(());
    }
    Err(std::io::Error::other(format!(
        "{CLIPBOARD_WRITE_COMMAND} exited with status {status}"
    )))
}

#[cfg(all(not(test), any(target_os = "macos", target_os = "linux")))]
pub(super) fn read_clipboard_text() -> Result<String, std::io::Error> {
    let output = Command::new(CLIPBOARD_READ_COMMAND)
        .args(CLIPBOARD_READ_ARGS)
        .output()?;
    if output.status.success() {
        return String::from_utf8(output.stdout)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error));
    }
    Err(std::io::Error::other(format!(
        "{CLIPBOARD_READ_COMMAND} exited with status {}",
        output.status,
    )))
}

#[cfg(all(not(test), not(any(target_os = "macos", target_os = "linux"))))]
pub(super) fn write_clipboard_text(_text: &str) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "storybook clipboard write is unsupported on this platform",
    ))
}

#[cfg(all(not(test), not(any(target_os = "macos", target_os = "linux"))))]
pub(super) fn read_clipboard_text() -> Result<String, std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "storybook clipboard read is unsupported on this platform",
    ))
}

#[cfg(test)]
pub(super) fn write_clipboard_text(_text: &str) -> Result<(), std::io::Error> {
    Ok(())
}

#[cfg(test)]
pub(super) fn read_clipboard_text() -> Result<String, std::io::Error> {
    Ok(String::new())
}

#[cfg(test)]
#[path = "window_command/tests.rs"]
mod tests;
