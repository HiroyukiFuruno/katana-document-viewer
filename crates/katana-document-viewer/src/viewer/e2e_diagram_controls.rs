use super::*;

const PAN_DELTA: ViewerVector = ViewerVector { x: 640.0, y: 32.0 };

pub(super) fn supported_diagram_controls(target: &ViewerTarget) -> Vec<DiagramControlRequirement> {
    let mut controls = Vec::new();
    push_fullscreen_controls(&mut controls, target);
    push_pan_controls(&mut controls, target);
    push_zoom_controls(&mut controls, target);
    push_reset_control(&mut controls, target);
    push_help_control(&mut controls, target);
    controls
}

fn push_fullscreen_controls(controls: &mut Vec<DiagramControlRequirement>, target: &ViewerTarget) {
    let opened = apply(DiagramViewportState::default(), fullscreen_open(target));
    if opened.fullscreen_open {
        controls.push(DiagramControlRequirement::FullscreenOpen);
    }
    let closed = apply(
        opened,
        DiagramControlCommand::FullscreenClose(target.clone()),
    );
    if !closed.fullscreen_open {
        controls.push(DiagramControlRequirement::FullscreenClose);
    }
}

fn push_pan_controls(controls: &mut Vec<DiagramControlRequirement>, target: &ViewerTarget) {
    for (source, requirement) in pan_requirements() {
        let state = apply(
            DiagramViewportState::default(),
            DiagramControlCommand::Pan(DiagramPanCommand {
                target: target.clone(),
                delta: PAN_DELTA,
                source,
            }),
        );
        if state.pan != DiagramViewportState::default().pan {
            controls.push(requirement);
        }
    }
}

fn push_zoom_controls(controls: &mut Vec<DiagramControlRequirement>, target: &ViewerTarget) {
    for (source, multiplier, requirement) in zoom_requirements() {
        let state = apply(
            DiagramViewportState::default(),
            DiagramControlCommand::Zoom(DiagramZoomCommand {
                target: target.clone(),
                multiplier,
                source,
            }),
        );
        if state.zoom != DiagramViewportState::default().zoom {
            controls.push(requirement);
        }
    }
}

fn push_reset_control(controls: &mut Vec<DiagramControlRequirement>, target: &ViewerTarget) {
    let zoomed = apply(DiagramViewportState::default(), zoom_in(target));
    let reset = apply(zoomed, DiagramControlCommand::Reset(target.clone()));
    if reset == DiagramViewportState::default() {
        controls.push(DiagramControlRequirement::Reset);
    }
}

fn push_help_control(controls: &mut Vec<DiagramControlRequirement>, target: &ViewerTarget) {
    let state = apply(
        DiagramViewportState::default(),
        DiagramControlCommand::TrackpadHelp(target.clone()),
    );
    if state.help_requested {
        controls.push(DiagramControlRequirement::TrackpadHelp);
    }
}

fn pan_requirements() -> [(DiagramPanSource, DiagramControlRequirement); 6] {
    [
        (DiagramPanSource::ButtonUp, DiagramControlRequirement::PanUp),
        (
            DiagramPanSource::ButtonDown,
            DiagramControlRequirement::PanDown,
        ),
        (
            DiagramPanSource::ButtonLeft,
            DiagramControlRequirement::PanLeft,
        ),
        (
            DiagramPanSource::ButtonRight,
            DiagramControlRequirement::PanRight,
        ),
        (DiagramPanSource::Drag, DiagramControlRequirement::DragPan),
        (
            DiagramPanSource::SmoothScroll,
            DiagramControlRequirement::SmoothScrollPan,
        ),
    ]
}

fn zoom_requirements() -> [(DiagramZoomSource, f32, DiagramControlRequirement); 3] {
    [
        (
            DiagramZoomSource::ButtonIn,
            1.25,
            DiagramControlRequirement::ZoomIn,
        ),
        (
            DiagramZoomSource::ButtonOut,
            0.75,
            DiagramControlRequirement::ZoomOut,
        ),
        (
            DiagramZoomSource::Trackpad,
            1.1,
            DiagramControlRequirement::TrackpadZoom,
        ),
    ]
}

fn fullscreen_open(target: &ViewerTarget) -> DiagramControlCommand {
    DiagramControlCommand::FullscreenOpen(target.clone())
}

fn zoom_in(target: &ViewerTarget) -> DiagramControlCommand {
    DiagramControlCommand::Zoom(DiagramZoomCommand {
        target: target.clone(),
        multiplier: 1.25,
        source: DiagramZoomSource::ButtonIn,
    })
}

fn apply(state: DiagramViewportState, command: DiagramControlCommand) -> DiagramViewportState {
    ViewerStateEngine::apply_diagram_command(state, &command)
}
