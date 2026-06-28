use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiagramControlRequirement {
    FullscreenOpen,
    FullscreenClose,
    PanUp,
    PanDown,
    PanLeft,
    PanRight,
    ZoomIn,
    ZoomOut,
    Reset,
    TrackpadHelp,
    DragPan,
    SmoothScrollPan,
    TrackpadZoom,
}

const REQUIRED_DIAGRAM_CONTROLS: [DiagramControlRequirement; 13] = [
    DiagramControlRequirement::FullscreenOpen,
    DiagramControlRequirement::FullscreenClose,
    DiagramControlRequirement::PanUp,
    DiagramControlRequirement::PanDown,
    DiagramControlRequirement::PanLeft,
    DiagramControlRequirement::PanRight,
    DiagramControlRequirement::ZoomIn,
    DiagramControlRequirement::ZoomOut,
    DiagramControlRequirement::Reset,
    DiagramControlRequirement::TrackpadHelp,
    DiagramControlRequirement::DragPan,
    DiagramControlRequirement::SmoothScrollPan,
    DiagramControlRequirement::TrackpadZoom,
];

pub struct DiagramControlParity;

impl DiagramControlParity {
    pub fn required_controls() -> &'static [DiagramControlRequirement] {
        &REQUIRED_DIAGRAM_CONTROLS
    }

    pub fn is_complete(supported: &[DiagramControlRequirement]) -> bool {
        Self::required_controls()
            .iter()
            .all(|required| supported.contains(required))
    }
}
