mod artifact_search;
mod asset;
mod code_block_metrics;
pub mod commands;
mod hit_test;
mod image_surface;
mod layout;
mod media_action;
mod media_control_spec;
mod node_plan;
mod parity;
mod search;
mod search_matcher;
mod search_resolver;
mod session;
mod settings_update;
mod slideshow_action;
mod state;
mod toc;
mod types;

pub use artifact_search::ViewerArtifactSearchResolver;
pub use asset::{
    ViewerAssetLoadPriority, ViewerAssetLoadRequest, ViewerAssetLoadResult, ViewerAssetPipeline,
    ViewerAssetReference, ViewerAssetState,
};
pub use code_block_metrics::ViewerCodeBlockMetrics;
pub use commands::{
    CopyTextCommand, CopyTextSource, DiagramControlCommand, DiagramPanCommand, DiagramPanSource,
    DiagramZoomCommand, DiagramZoomSource, HostCommand, ImageControlAction, ImageControlCommand,
    LinkCommand, SlideshowCommand, SlideshowSettingsUpdate, TaskStateCommand, ViewerCommand,
    ViewerCommandFactory, ViewerScrollCommand, ViewerTaskControlTarget, ViewerTaskState,
};
pub use hit_test::{ViewerHitTestIndex, ViewerRectFactory, ViewerTocCommandFactory};
pub use image_surface::{
    VIEWER_DIAGRAM_DISPLAY_MAX_WIDTH, VIEWER_DIAGRAM_DISPLAY_SCALE, ViewerImageSurface,
    ViewerImageSurfaceError, ViewerImageSurfaceFactory,
};
pub use layout::{
    ViewerLayoutEngine, ViewerLayoutResult, ViewerRenderedAnchor, ViewerVisibleRange,
};
pub use media_action::{ViewerMediaControlAction, ViewerMediaControlKind};
pub use media_control_spec::{
    ViewerDiagramControlSlot, ViewerMediaControlSet, ViewerMediaControlSpec,
};
pub use node_plan::{
    VIEWER_TEXT_COLOR_CHANNELS, ViewerCodeHighlighter, ViewerDiagramKind, ViewerHtmlAlignment,
    ViewerHtmlRole, ViewerNode, ViewerNodeKind, ViewerNodePlan, ViewerNodePlanner, ViewerTextSpan,
    ViewerTextStyle,
};
pub use parity::{DiagramControlParity, DiagramControlRequirement};
pub use search::{
    ViewerSearchCommand, ViewerSearchDirection, ViewerSearchEngine, ViewerSearchHighlight,
    ViewerSearchHighlightKind, ViewerSearchMatch, ViewerSearchMatchId, ViewerSearchState,
    ViewerSearchTarget, ViewerTextRange,
};
pub use search_matcher::{ViewerSearchTextMatch, ViewerSearchTextMatcher};
pub use search_resolver::{ViewerArtifactTextExtraction, ViewerSearchLayoutResolver};
pub use session::{ViewerConfigRevision, ViewerSession};
pub use settings_update::{
    ViewerSettingsField, ViewerSettingsState, ViewerSettingsUpdate, ViewerSettingsUpdateError,
    ViewerSettingsValue, ViewerTypographyConfig,
};
pub use slideshow_action::ViewerSlideshowControlAction;
pub use state::{ViewerModeSwitch, ViewerStateEngine};
pub use toc::ViewerTocModel;
pub use types::{
    DiagramViewportState, SlideshowState, ViewerHitTestResponse, ViewerInput,
    ViewerInteractionConfig, ViewerMode, ViewerPoint, ViewerRect, ViewerStateSnapshot,
    ViewerTarget, ViewerTocItem, ViewerVector, ViewerViewport,
};

#[cfg(test)]
#[path = "slideshow_tests.rs"]
mod slideshow_tests;

#[cfg(test)]
#[path = "runtime_test_support.rs"]
mod runtime_test_support;

#[cfg(test)]
#[path = "runtime_tests.rs"]
mod runtime_tests;

#[cfg(test)]
#[path = "e2e_tests.rs"]
mod e2e_tests;

#[cfg(test)]
#[path = "runtime_edge_tests.rs"]
mod runtime_edge_tests;

#[cfg(test)]
#[path = "node_plan_tests.rs"]
mod node_plan_tests;

#[cfg(test)]
#[path = "commands_tests.rs"]
mod commands_tests;

#[cfg(test)]
#[path = "commands_factory_tests.rs"]
mod commands_factory_tests;

#[cfg(test)]
#[path = "commands_factory_navigation_tests.rs"]
mod commands_factory_navigation_tests;

#[cfg(test)]
#[path = "media_action_tests.rs"]
mod media_action_tests;

#[cfg(test)]
#[path = "media_control_spec_tests.rs"]
mod media_control_spec_tests;

#[cfg(test)]
#[path = "search_resolver_tests.rs"]
mod search_resolver_tests;

#[cfg(test)]
#[path = "state_runtime_tests.rs"]
mod state_runtime_tests;

#[cfg(test)]
#[path = "toc_tests.rs"]
mod toc_tests;

#[cfg(test)]
#[path = "viewer_test_support.rs"]
mod viewer_test_support;

#[cfg(test)]
#[path = "viewer_toc_command_tests.rs"]
mod viewer_toc_command_tests;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
