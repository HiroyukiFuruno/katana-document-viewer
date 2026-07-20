use super::runtime_test_support::RuntimeTestData as Data;
use super::*;

const ZERO: f32 = 0.0;
const VIEWPORT_WIDTH: f32 = 640.0;
const VIEWPORT_HEIGHT: f32 = 320.0;
const TALL_CONTENT_HEIGHT: f32 = 900.0;
const TRACKPAD_ZOOM_HIGH: f32 = 10.0;
const TRACKPAD_ZOOM_LOW: f32 = 0.1;
const EXPECTED_MAX_TRACKPAD_ZOOM: f32 = 5.0;
const EXPECTED_MIN_TRACKPAD_ZOOM: f32 = 0.5;
const EXPECTED_BUTTON_ZOOM_OUT: f32 = 0.75;

#[test]
fn slideshow_controls_can_be_hidden_shown_and_zero_height_is_single_page() {
    let zero =
        ViewerStateEngine::slideshow_state(zero_viewport(), TALL_CONTENT_HEIGHT, 0, false, false);
    let hidden = ViewerStateEngine::hide_slideshow_controls(sample_slideshow_state());
    let shown = ViewerStateEngine::show_slideshow_controls(hidden);

    assert_eq!(zero.max_page_index, 0);
    assert!(shown.controls_visible);
}

#[test]
fn diagram_close_and_help_state_are_explicit() {
    let target = sample_target();
    let mut state = ViewerStateEngine::apply_diagram_command(
        DiagramViewportState::default(),
        &DiagramControlCommand::FullscreenOpen(target.clone()),
    );
    state = ViewerStateEngine::apply_diagram_command(
        state,
        &DiagramControlCommand::FullscreenClose(target.clone()),
    );
    state = ViewerStateEngine::apply_diagram_command(
        state,
        &DiagramControlCommand::TrackpadHelp(target.clone()),
    );

    assert!(!state.fullscreen_open);
    assert!(state.help_requested);
}

#[test]
fn diagram_pan_sources_update_state() {
    assert_ne!(
        pan_state(DiagramPanSource::ButtonUp).pan,
        DiagramViewportState::default().pan
    );
    assert_ne!(
        pan_state(DiagramPanSource::ButtonDown).pan,
        DiagramViewportState::default().pan
    );
    assert_ne!(
        pan_state(DiagramPanSource::ButtonLeft).pan,
        DiagramViewportState::default().pan
    );
    assert_ne!(
        pan_state(DiagramPanSource::Drag).pan,
        DiagramViewportState::default().pan
    );
}

#[test]
fn diagram_zoom_sources_clamp_to_declared_bounds() {
    let target = sample_target();
    let button_out = zoom_state(
        DiagramZoomSource::ButtonOut,
        TRACKPAD_ZOOM_LOW,
        target.clone(),
    );
    let trackpad_high = zoom_state(
        DiagramZoomSource::Trackpad,
        TRACKPAD_ZOOM_HIGH,
        target.clone(),
    );
    let trackpad_low = zoom_state(DiagramZoomSource::Trackpad, TRACKPAD_ZOOM_LOW, target);

    assert_eq!(button_out.zoom, EXPECTED_BUTTON_ZOOM_OUT);
    assert_eq!(trackpad_high.zoom, EXPECTED_MAX_TRACKPAD_ZOOM);
    assert_eq!(trackpad_low.zoom, EXPECTED_MIN_TRACKPAD_ZOOM);
}

#[test]
fn slideshow_state_does_not_define_dedicated_theme() -> Result<(), serde_json::Error> {
    let state = sample_slideshow_state();

    let value = serde_json::to_value(state)?;

    assert!(value.get("theme").is_none());
    assert!(value.get("theme_id").is_none());
    Ok(())
}

#[test]
fn slideshow_page_index_isolated_from_non_slideshow_mode() {
    let input = Data::viewer_input("state-non-slideshow", sample_viewport());
    let target_page = super::ViewerStateEngine::requested_slideshow_page(&input, 320.0);

    assert_eq!(0, target_page);
}

#[test]
fn page_index_for_scroll_handles_non_positive_inputs() {
    let viewport = sample_viewport();
    assert_eq!(
        0,
        super::ViewerStateEngine::page_index_for_scroll(-1.0, viewport.height)
    );
    assert_eq!(
        0,
        super::ViewerStateEngine::page_index_for_scroll(120.0, -1.0)
    );
}

#[test]
fn slideshow_command_updates_state_for_all_variants() -> Result<(), String> {
    let mut state = sample_slideshow_state();

    state = super::ViewerStateEngine::apply_slideshow_command(state, SlideshowCommand::NextPage);
    state =
        super::ViewerStateEngine::apply_slideshow_command(state, SlideshowCommand::PreviousPage);
    state = super::ViewerStateEngine::apply_slideshow_command(
        state,
        SlideshowCommand::UpdateSettings(crate::viewer::SlideshowSettingsUpdate {
            hover_highlight_enabled: false,
            diagram_controls_enabled: true,
        }),
    );
    state = super::ViewerStateEngine::apply_slideshow_command(state, SlideshowCommand::Close);

    assert!(state.close_requested);
    assert!(!state.hover_highlight_enabled);
    assert!(state.diagram_controls_enabled);
    Ok(())
}

#[test]
fn max_page_index_clamps_invalid_layout() {
    assert_eq!(0, super::ViewerStateEngine::max_page_index(100.0, 0.0));
    assert_eq!(0, super::ViewerStateEngine::max_page_index(0.0, 100.0));
}

fn sample_slideshow_state() -> SlideshowState {
    ViewerStateEngine::slideshow_state(sample_viewport(), TALL_CONTENT_HEIGHT, 0, false, false)
}

fn sample_viewport() -> ViewerViewport {
    ViewerViewport {
        width: VIEWPORT_WIDTH,
        height: VIEWPORT_HEIGHT,
    }
}

fn zero_viewport() -> ViewerViewport {
    ViewerViewport {
        width: VIEWPORT_WIDTH,
        height: ZERO,
    }
}

fn pan_state(source: DiagramPanSource) -> DiagramViewportState {
    ViewerStateEngine::apply_diagram_command(
        DiagramViewportState::default(),
        &DiagramControlCommand::Pan(DiagramPanCommand {
            target: sample_target(),
            delta: ViewerVector {
                x: VIEWPORT_WIDTH,
                y: VIEWPORT_HEIGHT,
            },
            source,
        }),
    )
}

fn zoom_state(
    source: DiagramZoomSource,
    multiplier: f32,
    target: ViewerTarget,
) -> DiagramViewportState {
    ViewerStateEngine::apply_diagram_command(
        DiagramViewportState::default(),
        &DiagramControlCommand::Zoom(DiagramZoomCommand {
            target,
            multiplier,
            source,
        }),
    )
}

fn sample_target() -> ViewerTarget {
    Data::viewer_target("state", ZERO)
}
