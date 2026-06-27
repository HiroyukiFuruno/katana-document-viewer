#[path = "node_plan/builder.rs"]
mod builder;
#[path = "node_plan/classifier.rs"]
mod classifier;
#[path = "node_plan/code_highlighter.rs"]
mod code_highlighter;
#[path = "node_plan/builder_media_height.rs"]
mod media_height;
#[path = "node_plan/metrics.rs"]
mod metrics;
#[path = "node_plan/builder_planned_node.rs"]
mod planned_node;
#[path = "node_plan/search_highlight.rs"]
mod search_highlight;
#[path = "node_plan/types.rs"]
mod types;

pub use builder::ViewerNodePlanner;
pub use code_highlighter::ViewerCodeHighlighter;
pub use types::{
    VIEWER_TEXT_COLOR_CHANNELS, ViewerDiagramKind, ViewerHtmlAlignment, ViewerHtmlRole, ViewerNode,
    ViewerNodeKind, ViewerNodePlan, ViewerTextSpan, ViewerTextStyle,
};
