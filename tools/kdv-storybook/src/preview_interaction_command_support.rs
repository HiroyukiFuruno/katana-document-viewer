use crate::catalog::StorybookFixture;
use crate::layout::preview_content_width;
use crate::media_host_action::StorybookMediaHostAction;
use crate::preview::{PreviewBuilder, PreviewScene};
use katana_document_viewer::{
    ArtifactId, ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
    ViewerInteractionConfig, ViewerMediaControlKind, ViewerMode, ViewerRect, ViewerSearchEngine,
    ViewerSearchState, ViewerSearchTarget, ViewerTarget, ViewerViewport,
};
use katana_ui_core::render_model::{UiHostActionPlan, UiNode};
use std::path::PathBuf;

#[path = "preview_interaction_task_support.rs"]
mod preview_interaction_task_support;

pub(crate) use preview_interaction_task_support::collect_task_markers;

const TEST_WINDOW_WIDTH: usize = 1_000;

thread_local! {
    static SHARED_PREVIEW_BUILDER: PreviewBuilder = PreviewBuilder::default();
}

pub(crate) fn build_scene(
    path: &str,
    interaction: ViewerInteractionConfig,
) -> Result<PreviewScene, Box<dyn std::error::Error>> {
    shared_preview_builder().build(
        &StorybookFixture {
            label: path.to_string(),
            path: fixture_path(&format!("assets/fixtures/{path}")),
        },
        ViewerViewport {
            width: preview_content_width(TEST_WINDOW_WIDTH) as f32,
            height: 20_000.0,
        },
        true,
        interaction,
    )
}

pub(crate) fn build_search_scene(
    path: &str,
    query: &str,
) -> Result<PreviewScene, Box<dyn std::error::Error>> {
    shared_preview_builder().build_with_mode_and_search(
        &StorybookFixture {
            label: path.to_string(),
            path: fixture_path(&format!("assets/fixtures/{path}")),
        },
        ViewerViewport {
            width: preview_content_width(TEST_WINDOW_WIDTH) as f32,
            height: 20_000.0,
        },
        true,
        ViewerInteractionConfig::default(),
        ViewerMode::Document,
        ViewerSearchEngine::state(query, Vec::new(), None),
    )
}

fn shared_preview_builder() -> PreviewBuilder {
    SHARED_PREVIEW_BUILDER.with(Clone::clone)
}

pub(crate) fn collect_diagram_actions(node: &UiNode) -> Vec<String> {
    collect_media_actions(node, ViewerMediaControlKind::Diagram)
}

pub(crate) fn collect_image_actions(node: &UiNode) -> Vec<String> {
    collect_media_actions(node, ViewerMediaControlKind::Image)
}

pub(crate) fn collect_code_actions(node: &UiNode) -> Vec<String> {
    collect_media_actions(node, ViewerMediaControlKind::Code)
}

fn collect_media_actions(node: &UiNode, kind: ViewerMediaControlKind) -> Vec<String> {
    UiHostActionPlan::collect_from_root(node)
        .into_iter()
        .filter_map(|action| {
            StorybookMediaHostAction::from_host_action_plan(&action)
                .map(|action| action.into_viewer_action())
        })
        .filter(|action| action.kind == kind)
        .map(|action| action.command)
        .collect()
}

pub(crate) fn collect_link_targets(node: &UiNode) -> Vec<String> {
    let mut values = node
        .props()
        .text
        .spans
        .iter()
        .filter(|span| !span.link_target.is_empty())
        .map(|span| span.link_target.clone())
        .collect::<Vec<_>>();
    for child in node.children() {
        values.extend(collect_link_targets(child));
    }
    values
}

pub(crate) fn target() -> ViewerTarget {
    ViewerTarget {
        node_id: KmmNodeId("storybook-interaction-node".to_string()),
        source: source_span(),
        artifact_id: ArtifactId("storybook-interaction-artifact".to_string()),
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: 64.0,
            height: 24.0,
        },
    }
}

pub(crate) fn search_state(targets: Vec<ViewerSearchTarget>) -> ViewerSearchState {
    assert!(!targets.is_empty(), "fixture must expose search targets");
    ViewerSearchEngine::state("Direct", targets, None)
}

fn source_span() -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange { start: 0, end: 1 },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn { line: 1, column: 2 },
        },
        raw: RawSnippet {
            text: "x".to_string(),
        },
    }
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../{path}"))
}
