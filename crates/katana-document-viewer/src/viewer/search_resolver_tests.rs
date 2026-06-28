use super::runtime_test_support::RuntimeTestData as Data;
use super::*;
use crate::ArtifactId;

#[test]
fn resolves_search_matches_to_rendered_rects_after_layout() {
    let mut source = Data::search_target("needle", 0, 0.0).matched;
    source.range = ViewerTextRange { start: 2, end: 8 };
    let anchor = ViewerRenderedAnchor {
        target: Data::viewer_target("needle", 120.0),
        anchor_index: 0,
    };

    let targets = ViewerSearchLayoutResolver::resolve_matches(&[source], &[anchor]);

    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].rect.y, 120.0);
    assert!(targets[0].rect.x > 0.0);
    assert!(targets[0].rect.width > 0.0);
}

#[test]
fn artifact_search_matches_require_explicit_text_extraction() {
    let extraction = ViewerArtifactTextExtraction {
        artifact_id: ArtifactId("diagram-svg".to_string()),
        node_id: Data::search_target("diagram", 0, 0.0).matched.node_id,
        source: Data::search_target("diagram", 0, 0.0).matched.source,
        text: "diagram contains needle".to_string(),
    };

    let no_extraction = ViewerSearchLayoutResolver::matches_from_artifact_text("needle", &[]);
    let matches = ViewerSearchLayoutResolver::matches_from_artifact_text("needle", &[extraction]);

    assert!(no_extraction.is_empty());
    assert_eq!(matches.len(), 1);
    assert_eq!(
        matches[0].artifact_id,
        Some(ArtifactId("diagram-svg".to_string()))
    );
}

#[test]
fn artifact_search_matches_case_insensitively_and_all_occurrences() {
    let extraction = ViewerArtifactTextExtraction {
        artifact_id: ArtifactId("diagram-svg".to_string()),
        node_id: Data::search_target("diagram", 0, 0.0).matched.node_id,
        source: Data::search_target("diagram", 0, 0.0).matched.source,
        text: "Needle needle".to_string(),
    };

    let matches = ViewerSearchLayoutResolver::matches_from_artifact_text("needle", &[extraction]);

    assert_eq!(2, matches.len());
    assert_eq!("Needle", matches[0].text);
    assert_eq!("needle", matches[1].text);
}

#[test]
fn empty_artifact_query_returns_no_matches() {
    let extraction = ViewerArtifactTextExtraction {
        artifact_id: ArtifactId("diagram-svg".to_string()),
        node_id: Data::search_target("diagram", 0, 0.0).matched.node_id,
        source: Data::search_target("diagram", 0, 0.0).matched.source,
        text: "diagram contains needle".to_string(),
    };

    let matches = ViewerSearchLayoutResolver::matches_from_artifact_text("", &[extraction]);

    assert!(matches.is_empty());
}

#[test]
fn search_match_ignores_anchor_for_different_node() {
    let matched = Data::search_target("needle", 0, 0.0).matched;
    let anchor = ViewerRenderedAnchor {
        target: Data::viewer_target("other", 0.0),
        anchor_index: 0,
    };

    let targets = ViewerSearchLayoutResolver::resolve_matches(&[matched], &[anchor]);

    assert!(targets.is_empty());
}

#[test]
fn artifact_search_resolves_only_to_matching_artifact_anchor() {
    let artifact_id = ArtifactId("pdf-page".to_string());
    let mut matched = Data::search_target("pdf", 0, 0.0).matched;
    matched.artifact_id = Some(artifact_id.clone());
    let wrong_anchor = ViewerRenderedAnchor {
        target: target_with_artifact("pdf", 40.0, ArtifactId("html".to_string())),
        anchor_index: 0,
    };
    let right_anchor = ViewerRenderedAnchor {
        target: target_with_artifact("pdf", 80.0, artifact_id),
        anchor_index: 1,
    };

    let targets =
        ViewerSearchLayoutResolver::resolve_matches(&[matched], &[wrong_anchor, right_anchor]);

    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].rect.y, 80.0);
}

fn target_with_artifact(label: &str, y: f32, artifact_id: ArtifactId) -> ViewerTarget {
    let mut target = Data::viewer_target(label, y);
    target.artifact_id = artifact_id;
    target
}
