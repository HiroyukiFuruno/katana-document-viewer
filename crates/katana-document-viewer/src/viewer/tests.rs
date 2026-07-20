use super::viewer_test_support::{sample_target, target_at};
use super::*;

const EXPECTED_BUTTON_PAN_X: f32 = 50.0;
const EXPECTED_BUTTON_ZOOM: f32 = 1.25;

#[test]
fn diagram_parity_requires_all_katana_controls() {
    let required = DiagramControlParity::required_controls();

    assert!(required.contains(&DiagramControlRequirement::FullscreenOpen));
    assert!(required.contains(&DiagramControlRequirement::FullscreenClose));
    assert!(required.contains(&DiagramControlRequirement::PanUp));
    assert!(required.contains(&DiagramControlRequirement::PanDown));
    assert!(required.contains(&DiagramControlRequirement::PanLeft));
    assert!(required.contains(&DiagramControlRequirement::PanRight));
    assert!(required.contains(&DiagramControlRequirement::ZoomIn));
    assert!(required.contains(&DiagramControlRequirement::ZoomOut));
    assert!(required.contains(&DiagramControlRequirement::Reset));
    assert!(required.contains(&DiagramControlRequirement::TrackpadHelp));
    assert!(required.contains(&DiagramControlRequirement::DragPan));
    assert!(required.contains(&DiagramControlRequirement::SmoothScrollPan));
    assert!(required.contains(&DiagramControlRequirement::TrackpadZoom));
    assert!(DiagramControlParity::is_complete(required));
}

#[test]
fn diagram_state_applies_pan_zoom_fullscreen_and_reset() {
    let target = sample_target();
    let state = state_after_basic_diagram_controls(&target);

    assert!(state.fullscreen_open);
    assert_eq!(
        state.pan,
        ViewerVector {
            x: EXPECTED_BUTTON_PAN_X,
            y: 0.0,
        }
    );
    assert_eq!(state.zoom, EXPECTED_BUTTON_ZOOM);

    let reset =
        ViewerStateEngine::apply_diagram_command(state, &DiagramControlCommand::Reset(target));
    assert_eq!(
        reset,
        DiagramViewportState {
            fullscreen_open: true,
            ..DiagramViewportState::default()
        }
    );
}

#[test]
fn diagram_reset_keeps_non_fullscreen_viewport_closed() {
    let target = sample_target();
    let state = ViewerStateEngine::apply_diagram_command(
        DiagramViewportState::default(),
        &button_zoom_in(&target),
    );

    let reset =
        ViewerStateEngine::apply_diagram_command(state, &DiagramControlCommand::Reset(target));
    assert_eq!(reset, DiagramViewportState::default());
}

fn state_after_basic_diagram_controls(target: &ViewerTarget) -> DiagramViewportState {
    let mut state = ViewerStateEngine::apply_diagram_command(
        DiagramViewportState::default(),
        &DiagramControlCommand::FullscreenOpen(target.clone()),
    );
    state = ViewerStateEngine::apply_diagram_command(state, &button_right_pan(target));
    ViewerStateEngine::apply_diagram_command(state, &button_zoom_in(target))
}

#[test]
fn image_state_applies_zoom_and_fit() {
    let target = sample_target();
    let mut state = ViewerStateEngine::apply_image_command(
        DiagramViewportState::default(),
        &ImageControlCommand {
            target: target.clone(),
            action: ImageControlAction::ZoomIn,
        },
    );
    assert_eq!(EXPECTED_BUTTON_ZOOM, state.zoom);

    state = ViewerStateEngine::apply_image_command(
        state,
        &ImageControlCommand {
            target,
            action: ImageControlAction::Fit,
        },
    );
    assert_eq!(DiagramViewportState::default(), state);
}

#[test]
fn image_zoom_out_is_clamped_and_host_only_actions_preserve_viewport() {
    let target = sample_target();
    let state = ViewerStateEngine::apply_image_command(
        DiagramViewportState::default(),
        &ImageControlCommand {
            target: target.clone(),
            action: ImageControlAction::ZoomOut,
        },
    );
    assert!(state.zoom < 1.0);

    for action in [
        ImageControlAction::Copy,
        ImageControlAction::Open,
        ImageControlAction::RevealInOs,
    ] {
        assert_eq!(
            state,
            ViewerStateEngine::apply_image_command(
                state,
                &ImageControlCommand {
                    target: target.clone(),
                    action,
                },
            )
        );
    }
}

fn button_right_pan(target: &ViewerTarget) -> DiagramControlCommand {
    DiagramControlCommand::Pan(DiagramPanCommand {
        target: target.clone(),
        delta: ViewerVector { x: 0.0, y: 0.0 },
        source: DiagramPanSource::ButtonRight,
    })
}

fn button_zoom_in(target: &ViewerTarget) -> DiagramControlCommand {
    DiagramControlCommand::Zoom(DiagramZoomCommand {
        target: target.clone(),
        multiplier: 1.0,
        source: DiagramZoomSource::ButtonIn,
    })
}

#[test]
fn viewer_mode_switches_without_window_side_effects() {
    let document_mode = ViewerModeSwitch::document();
    let slideshow_mode = ViewerModeSwitch::slideshow();
    let state = ViewerStateEngine::apply_slideshow_command(
        SlideshowState::default(),
        SlideshowCommand::Close,
    );

    assert_eq!(document_mode, ViewerMode::Document);
    assert_eq!(slideshow_mode, ViewerMode::Slideshow);
    assert!(state.close_requested);
}

#[test]
fn hit_test_returns_node_metadata_and_miss_state() {
    let target = sample_target();
    let index = ViewerHitTestIndex::new(vec![target.clone()]);

    let hit = index.hit_test(ViewerPoint { x: 12.0, y: 14.0 });
    let miss = index.hit_test(ViewerPoint { x: 80.0, y: 90.0 });

    assert_eq!(hit, ViewerHitTestResponse::Hit(target));
    assert_eq!(
        miss,
        ViewerHitTestResponse::Miss(ViewerPoint { x: 80.0, y: 90.0 })
    );
}

#[test]
fn hit_test_y_index_preserves_original_target_order_for_overlaps() {
    let first = target_at("first", 0.0, 10.0);
    let second = target_at("second", 0.0, 0.0);
    let index = ViewerHitTestIndex::new(vec![first.clone(), second]);

    let hit = index.hit_test(ViewerPoint { x: 12.0, y: 12.0 });

    assert_eq!(ViewerHitTestResponse::Hit(first), hit);
}
