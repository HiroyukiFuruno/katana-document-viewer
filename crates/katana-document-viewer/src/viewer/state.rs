use crate::viewer::commands::{
    DiagramControlCommand, DiagramPanSource, DiagramZoomSource, ImageControlAction,
    ImageControlCommand, SlideshowCommand,
};
use crate::viewer::types::{
    DiagramViewportState, SlideshowState, ViewerInput, ViewerMode, ViewerStateSnapshot,
    ViewerVector, ViewerViewport,
};

const BUTTON_PAN_STEP: f32 = 50.0;
const BUTTON_ZOOM_STEP: f32 = 0.25;
const BUTTON_ZOOM_MIN: f32 = 0.25;
const BUTTON_ZOOM_MAX: f32 = 4.0;
const TRACKPAD_ZOOM_MIN: f32 = 0.5;
const TRACKPAD_ZOOM_MAX: f32 = 5.0;

pub struct ViewerStateEngine;

impl ViewerStateEngine {
    pub fn snapshot(
        input: &ViewerInput,
        content_height: f32,
        scroll_y: f32,
    ) -> ViewerStateSnapshot {
        let requested_page_index = Self::requested_slideshow_page(input, scroll_y);
        let slideshow = Self::slideshow_state(
            input.viewport,
            content_height,
            requested_page_index,
            input.interaction.hover_highlight_enabled,
            input.interaction.diagram_controls_enabled,
        );
        ViewerStateSnapshot {
            mode: input.mode.clone(),
            viewport: input.viewport,
            scroll_y,
            content_height,
            slideshow,
            interaction: input.interaction.clone(),
            search: input.search.clone(),
        }
    }

    pub(super) fn requested_slideshow_page(input: &ViewerInput, scroll_y: f32) -> usize {
        if input.mode != ViewerMode::Slideshow {
            return 0;
        }
        Self::page_index_for_scroll(scroll_y, input.viewport.height)
    }

    pub fn page_index_for_scroll(scroll_y: f32, viewport_height: f32) -> usize {
        if scroll_y <= 0.0 || viewport_height <= 0.0 {
            return 0;
        }
        (scroll_y / viewport_height).floor() as usize
    }

    pub fn slideshow_state(
        viewport: ViewerViewport,
        content_height: f32,
        requested_page_index: usize,
        hover_highlight_enabled: bool,
        diagram_controls_enabled: bool,
    ) -> SlideshowState {
        let max_page_index = Self::max_page_index(content_height, viewport.height);
        SlideshowState {
            current_page_index: requested_page_index.min(max_page_index),
            max_page_index,
            viewport_height: viewport.height,
            content_height,
            controls_visible: true,
            close_requested: false,
            hover_highlight_enabled,
            diagram_controls_enabled,
        }
    }

    pub fn apply_slideshow_command(
        mut state: SlideshowState,
        command: SlideshowCommand,
    ) -> SlideshowState {
        state.controls_visible = true;
        match command {
            SlideshowCommand::NextPage => {
                state.current_page_index = (state.current_page_index + 1).min(state.max_page_index);
            }
            SlideshowCommand::PreviousPage => {
                state.current_page_index = state.current_page_index.saturating_sub(1);
            }
            SlideshowCommand::Close => {
                state.close_requested = true;
            }
            SlideshowCommand::UpdateSettings(update) => {
                state.hover_highlight_enabled = update.hover_highlight_enabled;
                state.diagram_controls_enabled = update.diagram_controls_enabled;
            }
        }
        state
    }

    pub fn hide_slideshow_controls(mut state: SlideshowState) -> SlideshowState {
        state.controls_visible = false;
        state
    }

    pub fn show_slideshow_controls(mut state: SlideshowState) -> SlideshowState {
        state.controls_visible = true;
        state
    }

    pub fn apply_diagram_command(
        mut state: DiagramViewportState,
        command: &DiagramControlCommand,
    ) -> DiagramViewportState {
        match command {
            DiagramControlCommand::FullscreenOpen(_) => state.fullscreen_open = true,
            DiagramControlCommand::FullscreenClose(_) => state.fullscreen_open = false,
            DiagramControlCommand::Pan(pan) => {
                state.pan = Self::pan(state.pan, pan.source, pan.delta)
            }
            DiagramControlCommand::Zoom(zoom) => {
                state.zoom = Self::zoom(state.zoom, zoom.source, zoom.multiplier);
            }
            DiagramControlCommand::Reset(_) => {
                state = DiagramViewportState {
                    fullscreen_open: state.fullscreen_open,
                    ..DiagramViewportState::default()
                };
            }
            DiagramControlCommand::TrackpadHelp(_) => state.help_requested = true,
        }
        state
    }

    pub fn apply_image_command(
        mut state: DiagramViewportState,
        command: &ImageControlCommand,
    ) -> DiagramViewportState {
        match command.action {
            ImageControlAction::ZoomIn => {
                state.zoom = (state.zoom + BUTTON_ZOOM_STEP).min(BUTTON_ZOOM_MAX);
            }
            ImageControlAction::ZoomOut => {
                state.zoom = (state.zoom - BUTTON_ZOOM_STEP).max(BUTTON_ZOOM_MIN);
            }
            ImageControlAction::Fit => state = DiagramViewportState::default(),
            ImageControlAction::Copy
            | ImageControlAction::Open
            | ImageControlAction::RevealInOs => {}
        }
        state
    }

    pub(super) fn max_page_index(content_height: f32, viewport_height: f32) -> usize {
        if content_height <= 0.0 || viewport_height <= 0.0 {
            return 0;
        }
        (content_height / viewport_height).floor() as usize
    }

    fn pan(current: ViewerVector, source: DiagramPanSource, delta: ViewerVector) -> ViewerVector {
        match source {
            DiagramPanSource::ButtonUp => Self::add_pan(current, 0.0, -BUTTON_PAN_STEP),
            DiagramPanSource::ButtonDown => Self::add_pan(current, 0.0, BUTTON_PAN_STEP),
            DiagramPanSource::ButtonLeft => Self::add_pan(current, -BUTTON_PAN_STEP, 0.0),
            DiagramPanSource::ButtonRight => Self::add_pan(current, BUTTON_PAN_STEP, 0.0),
            DiagramPanSource::Drag | DiagramPanSource::SmoothScroll => {
                Self::add_pan(current, delta.x, delta.y)
            }
        }
    }

    fn add_pan(current: ViewerVector, x: f32, y: f32) -> ViewerVector {
        ViewerVector {
            x: current.x + x,
            y: current.y + y,
        }
    }

    fn zoom(current: f32, source: DiagramZoomSource, multiplier: f32) -> f32 {
        match source {
            DiagramZoomSource::ButtonIn => (current + BUTTON_ZOOM_STEP).min(BUTTON_ZOOM_MAX),
            DiagramZoomSource::ButtonOut => (current - BUTTON_ZOOM_STEP).max(BUTTON_ZOOM_MIN),
            DiagramZoomSource::Trackpad => {
                (current * multiplier).clamp(TRACKPAD_ZOOM_MIN, TRACKPAD_ZOOM_MAX)
            }
        }
    }
}

pub struct ViewerModeSwitch;

impl ViewerModeSwitch {
    pub fn document() -> ViewerMode {
        ViewerMode::Document
    }

    pub fn slideshow() -> ViewerMode {
        ViewerMode::Slideshow
    }
}
