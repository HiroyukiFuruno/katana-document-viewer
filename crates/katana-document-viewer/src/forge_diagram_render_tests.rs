#[path = "forge_diagram_render_pipeline_tests.rs"]
mod pipeline_tests;
#[path = "forge_diagram_render_test_support.rs"]
mod support;

pub(crate) use support::{
    DiagramRenderTestSupport, ErrorDiagramEngine, PanicDiagramEngine, RecordingDiagramEngine,
};
