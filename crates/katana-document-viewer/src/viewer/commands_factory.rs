use super::{
    CopyTextCommand, CopyTextSource, DiagramControlCommand, DiagramPanCommand, DiagramPanSource,
    DiagramZoomCommand, DiagramZoomSource, HostCommand, ImageControlAction, ImageControlCommand,
    LinkCommand, SlideshowCommand, SlideshowSettingsUpdate, TaskStateCommand, ViewerCommand,
    ViewerCommandFactory, ViewerTaskControlTarget, ViewerTaskState,
};
use crate::viewer::hit_test::ViewerTocCommandFactory;
use crate::viewer::search::{ViewerSearchDirection, ViewerSearchEngine, ViewerSearchState};
use crate::viewer::types::{ViewerTarget, ViewerTocItem, ViewerVector};

impl ViewerCommandFactory {
    pub fn open_link(target: ViewerTarget, uri: impl Into<String>) -> ViewerCommand {
        ViewerCommand::Link(LinkCommand {
            target,
            uri: uri.into(),
        })
    }

    pub fn toggle_task(target: ViewerTarget, current: ViewerTaskState) -> ViewerCommand {
        Self::set_task_state(target, current.toggled_by_click())
    }

    pub fn toggle_task_control(
        target: ViewerTarget,
        task_target: ViewerTaskControlTarget,
        current: ViewerTaskState,
    ) -> ViewerCommand {
        Self::set_task_control_state(target, task_target, current.toggled_by_click())
    }

    pub fn set_task_state(target: ViewerTarget, state: ViewerTaskState) -> ViewerCommand {
        ViewerCommand::Task(TaskStateCommand {
            target,
            task_target: None,
            state,
        })
    }

    pub fn set_task_control_state(
        target: ViewerTarget,
        task_target: ViewerTaskControlTarget,
        state: ViewerTaskState,
    ) -> ViewerCommand {
        ViewerCommand::Task(TaskStateCommand {
            target,
            task_target: Some(task_target),
            state,
        })
    }

    pub fn set_task_state_from_marker(target: ViewerTarget, marker: &str) -> Option<ViewerCommand> {
        ViewerTaskState::from_marker(marker).map(|state| Self::set_task_state(target, state))
    }

    pub fn image_control_from_action(target: ViewerTarget, action: &str) -> Option<ViewerCommand> {
        Some(ViewerCommand::Image(ImageControlCommand {
            target,
            action: image_action(action)?,
        }))
    }

    pub fn diagram_control_from_action(
        target: ViewerTarget,
        action: &str,
        fullscreen_open: bool,
    ) -> Option<ViewerCommand> {
        if action == "copy-source" {
            return Some(Self::copy_text_command(
                target,
                CopyTextSource::DiagramSource,
            ));
        }
        Some(ViewerCommand::Diagram(diagram_action(
            target,
            action,
            fullscreen_open,
        )?))
    }

    pub fn code_control_from_action(target: ViewerTarget, action: &str) -> Option<ViewerCommand> {
        if action != "copy-code" {
            return None;
        }
        Some(Self::copy_text_command(target, CopyTextSource::Code))
    }

    pub fn scroll_to_toc_item(item: ViewerTocItem) -> ViewerCommand {
        ViewerTocCommandFactory::scroll_to(item)
    }

    pub fn scroll_to_target(target: ViewerTarget) -> ViewerCommand {
        ViewerCommand::ScrollToHeading(super::ViewerScrollCommand { target })
    }

    pub fn navigate_search(
        state: &ViewerSearchState,
        direction: ViewerSearchDirection,
    ) -> Option<ViewerCommand> {
        ViewerSearchEngine::navigate(state, direction).map(ViewerCommand::Search)
    }

    pub fn next_slideshow_page() -> ViewerCommand {
        ViewerCommand::Slideshow(SlideshowCommand::NextPage)
    }

    pub fn previous_slideshow_page() -> ViewerCommand {
        ViewerCommand::Slideshow(SlideshowCommand::PreviousPage)
    }

    pub fn close_slideshow() -> ViewerCommand {
        ViewerCommand::Slideshow(SlideshowCommand::Close)
    }

    pub fn update_slideshow_settings(
        hover_highlight_enabled: bool,
        diagram_controls_enabled: bool,
    ) -> ViewerCommand {
        ViewerCommand::Slideshow(SlideshowCommand::UpdateSettings(SlideshowSettingsUpdate {
            hover_highlight_enabled,
            diagram_controls_enabled,
        }))
    }

    fn copy_text_command(target: ViewerTarget, source: CopyTextSource) -> ViewerCommand {
        ViewerCommand::Host(HostCommand::CopyText(CopyTextCommand {
            text: target.source.raw.text.clone(),
            target,
            source,
        }))
    }
}

fn image_action(action: &str) -> Option<ImageControlAction> {
    match action {
        "fit" => Some(ImageControlAction::Fit),
        "open" => Some(ImageControlAction::Open),
        "copy" => Some(ImageControlAction::Copy),
        "reveal-in-os" => Some(ImageControlAction::RevealInOs),
        "zoom-in" => Some(ImageControlAction::ZoomIn),
        "zoom-out" => Some(ImageControlAction::ZoomOut),
        _ => None,
    }
}

fn diagram_action(
    target: ViewerTarget,
    action: &str,
    fullscreen_open: bool,
) -> Option<DiagramControlCommand> {
    match action {
        "fullscreen" => Some(fullscreen_command(target, fullscreen_open)),
        "pan-up" => Some(pan_command(target, DiagramPanSource::ButtonUp)),
        "pan-down" => Some(pan_command(target, DiagramPanSource::ButtonDown)),
        "pan-left" => Some(pan_command(target, DiagramPanSource::ButtonLeft)),
        "pan-right" => Some(pan_command(target, DiagramPanSource::ButtonRight)),
        "zoom-in" => Some(zoom_command(target, DiagramZoomSource::ButtonIn)),
        "zoom-out" => Some(zoom_command(target, DiagramZoomSource::ButtonOut)),
        "reset-view" => Some(DiagramControlCommand::Reset(target)),
        "trackpad-help" => Some(DiagramControlCommand::TrackpadHelp(target)),
        _ => None,
    }
}

fn fullscreen_command(target: ViewerTarget, open: bool) -> DiagramControlCommand {
    if open {
        return DiagramControlCommand::FullscreenClose(target);
    }
    DiagramControlCommand::FullscreenOpen(target)
}

fn pan_command(target: ViewerTarget, source: DiagramPanSource) -> DiagramControlCommand {
    DiagramControlCommand::Pan(DiagramPanCommand {
        target,
        delta: ViewerVector { x: 0.0, y: 0.0 },
        source,
    })
}

fn zoom_command(target: ViewerTarget, source: DiagramZoomSource) -> DiagramControlCommand {
    DiagramControlCommand::Zoom(DiagramZoomCommand {
        target,
        multiplier: 1.0,
        source,
    })
}
