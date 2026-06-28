use super::runtime_test_support::{CONTENT_HEIGHT, RuntimeTestData as Data};
use super::*;
use crate::{ArtifactId, SourceRevision};

const SCROLL_Y: f32 = 48.0;
const ZERO_SPACER: f32 = 0.0;

#[test]
fn asset_pipeline_accepts_current_revision_once() {
    let artifact_id = ArtifactId("current".to_string());
    let mut state = ViewerAssetPipeline::initial_state(SourceRevision("rev".to_string()));
    let result = ViewerAssetLoadResult {
        document_revision: SourceRevision("rev".to_string()),
        artifact_id: artifact_id.clone(),
    };

    assert!(ViewerAssetPipeline::accept_result(
        &mut state,
        result.clone()
    ));
    assert!(!ViewerAssetPipeline::accept_result(&mut state, result));
    assert_eq!(state.loaded_artifacts, vec![artifact_id]);
}

#[test]
fn layout_without_anchors_has_no_bottom_spacer() {
    let result =
        ViewerLayoutEngine::from_anchors(Data::viewport(), Vec::new(), CONTENT_HEIGHT, SCROLL_Y);

    assert_eq!(result.bottom_spacer_height, ZERO_SPACER);
}

#[test]
fn search_navigation_returns_none_without_matches() {
    let state = ViewerSearchEngine::state("needle", Vec::new(), None);

    assert_eq!(
        ViewerSearchEngine::navigate(&state, ViewerSearchDirection::Next),
        None
    );
}

#[test]
fn session_keeps_scroll_for_same_document_revision() {
    let input = Data::viewer_input("rev", Data::viewport());
    let mut session = ViewerSession::new(&input, ViewerConfigRevision("cfg".to_string()));
    session.scroll_y = SCROLL_Y;

    session.apply_document(&input);

    assert_eq!(session.scroll_y, SCROLL_Y);
}
