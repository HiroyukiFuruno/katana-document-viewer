use super::runtime_test_support::RuntimeTestData as Data;
use super::runtime_test_support::*;
use super::*;
use crate::{ArtifactFormat, ArtifactId, SourceRevision};
use katana_markdown_model::KmmNodeId;

#[test]
fn viewer_session_keeps_single_instance_and_resets_by_document_revision() {
    let first_input = Data::viewer_input("rev-1", Data::viewport());
    let second_input = Data::viewer_input("rev-2", Data::resized_viewport());
    let mut session = ViewerSession::new(&first_input, ViewerConfigRevision("cfg-1".to_string()));

    session.scroll_y = 120.0;
    session.apply_config(&first_input, ViewerConfigRevision("cfg-2".to_string()));
    assert_eq!(
        session.document_revision,
        SourceRevision("rev-1".to_string())
    );
    assert_eq!(session.scroll_y, 120.0);

    session.apply_document(&second_input);
    assert_eq!(
        session.document_revision,
        SourceRevision("rev-2".to_string())
    );
    assert_eq!(session.viewport, Data::resized_viewport());
    assert_eq!(session.scroll_y, 0.0);
    assert!(session.assets.requests.is_empty());
}

#[test]
fn layout_bottom_spacer_allows_last_anchor_to_align_to_top() {
    let anchor = ViewerRenderedAnchor {
        target: Data::viewer_target("last-heading", LAST_ANCHOR_Y),
        anchor_index: 0,
    };
    let result = ViewerLayoutEngine::from_anchors(
        Data::viewport(),
        vec![anchor.clone()],
        CONTENT_HEIGHT,
        0.0,
    );

    let scroll_y =
        ViewerLayoutEngine::scroll_y_for_rect(&result, Data::viewport(), anchor.target.rect);

    assert_eq!(result.bottom_spacer_height, 250.0);
    assert_eq!(scroll_y, LAST_ANCHOR_Y);
}

#[test]
fn asset_pipeline_prioritizes_visible_and_rejects_stale_results() {
    let visible = ArtifactId("visible-image".to_string());
    let hidden = ArtifactId("hidden-image".to_string());
    let references = vec![
        Data::asset_reference(&visible),
        Data::asset_reference(&hidden),
    ];

    let requests = ViewerAssetPipeline::load_requests(
        SourceRevision("rev-1".to_string()),
        &references,
        std::slice::from_ref(&visible),
    );
    let mut state = ViewerAssetPipeline::initial_state(SourceRevision("rev-2".to_string()));
    let accepted = ViewerAssetPipeline::accept_result(
        &mut state,
        ViewerAssetLoadResult {
            document_revision: SourceRevision("rev-1".to_string()),
            artifact_id: visible,
        },
    );

    assert_eq!(requests[0].priority, ViewerAssetLoadPriority::Visible);
    assert_eq!(requests[1].priority, ViewerAssetLoadPriority::Deferred);
    assert!(!accepted);
    assert!(state.loaded_artifacts.is_empty());
}

#[test]
fn asset_pipeline_reuses_existing_artifact_metadata() {
    let artifact = Data::artifact("html", ArtifactFormat::Html);

    let reference =
        ViewerAssetPipeline::reference_for_artifact(KmmNodeId("node-html".to_string()), &artifact);

    assert_eq!(reference.artifact_id, artifact.manifest.id);
    assert_eq!(reference.uri, artifact.uri);
    assert_eq!(reference.format, ArtifactFormat::Html);
}

#[test]
fn asset_pipeline_reuses_existing_artifacts_without_reparsing_bytes() {
    let artifacts = vec![
        Data::artifact("html", ArtifactFormat::Html),
        Data::artifact("pdf", ArtifactFormat::Pdf),
        Data::artifact("diagram", ArtifactFormat::Svg),
    ];

    let references = ViewerAssetPipeline::references_for_artifacts(
        KmmNodeId("node-rendered".to_string()),
        &artifacts,
    );

    assert_eq!(references.len(), artifacts.len());
    assert_eq!(references[0].format, ArtifactFormat::Html);
    assert_eq!(references[1].format, ArtifactFormat::Pdf);
    assert_eq!(references[2].format, ArtifactFormat::Svg);
}

#[test]
fn asset_pipeline_marks_near_viewport_priority() {
    let references = [
        Data::asset_reference_for_format("html", ArtifactFormat::Html),
        Data::asset_reference_for_format("pdf", ArtifactFormat::Pdf),
        Data::asset_reference_for_format("png", ArtifactFormat::Png),
        Data::asset_reference_for_format("jpeg", ArtifactFormat::Jpeg),
        Data::asset_reference_for_format("svg", ArtifactFormat::Svg),
    ];
    let visible = references[0].artifact_id.clone();
    let near_viewport = references[1].artifact_id.clone();

    let requests = ViewerAssetPipeline::load_requests_for_viewport(
        SourceRevision("rev-asset".to_string()),
        &references,
        std::slice::from_ref(&visible),
        std::slice::from_ref(&near_viewport),
    );

    assert_eq!(requests[0].priority, ViewerAssetLoadPriority::Visible);
    assert_eq!(requests[1].priority, ViewerAssetLoadPriority::NearViewport);
    assert_eq!(requests[2].priority, ViewerAssetLoadPriority::Deferred);
    assert_eq!(requests[3].format, ArtifactFormat::Jpeg);
    assert_eq!(requests[4].format, ArtifactFormat::Svg);
}

#[test]
fn search_navigation_wraps_and_returns_scroll_command() -> Result<(), &'static str> {
    let matches = vec![
        Data::search_target("first", 0, 100.0),
        Data::search_target("second", 1, LAST_ANCHOR_Y),
    ];
    let state = ViewerSearchEngine::state("needle", matches, Some(1));

    let Some(command) = ViewerSearchEngine::navigate(&state, ViewerSearchDirection::Next) else {
        return Err("next match command missing");
    };

    assert_eq!(command.target.index, 0);
    assert_eq!(command.scroll.target.rect.y, 100.0);
    Ok(())
}

#[test]
fn search_highlights_current_match_and_wraps_previous() -> Result<(), &'static str> {
    let matches = vec![
        Data::search_target("first", 0, 100.0),
        Data::search_target("second", 1, LAST_ANCHOR_Y),
    ];
    let state = ViewerSearchEngine::state("needle", matches, Some(0));

    let highlights = ViewerSearchEngine::highlights(&state);
    let Some(command) = ViewerSearchEngine::navigate(&state, ViewerSearchDirection::Previous)
    else {
        return Err("previous match command missing");
    };

    assert_eq!(highlights[0].kind, ViewerSearchHighlightKind::Current);
    assert_eq!(highlights[1].kind, ViewerSearchHighlightKind::Match);
    assert_eq!(command.target.index, 1);
    Ok(())
}

#[test]
fn bottom_spacer_allows_last_search_hit_to_align_to_top() {
    let target = Data::search_target("last-hit", 0, LAST_ANCHOR_Y);
    let anchor = ViewerRenderedAnchor {
        target: Data::command_target(&target),
        anchor_index: target.index,
    };
    let result = ViewerLayoutEngine::from_anchors(
        Data::viewport(),
        vec![anchor.clone()],
        CONTENT_HEIGHT,
        0.0,
    );

    let scroll_y =
        ViewerLayoutEngine::scroll_y_for_rect(&result, Data::viewport(), anchor.target.rect);

    assert_eq!(scroll_y, LAST_ANCHOR_Y);
}
