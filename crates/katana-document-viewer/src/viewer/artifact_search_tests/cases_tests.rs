use super::support::{node_with_artifact, node_without_artifact, text_artifact};
use crate::{
    ArtifactBytes, ArtifactDiagnostics, ArtifactFactory, ArtifactFormat, ArtifactId, DocumentId,
    SourceRevision, ViewerArtifactSearchResolver,
};

#[test]
fn svg_artifact_text_resolves_to_matching_viewer_node_rect() {
    let artifact_id = ArtifactId("diagram-svg".to_string());
    let node = node_with_artifact("diagram", artifact_id.clone(), 128.0);
    let artifact = text_artifact(
        artifact_id.clone(),
        ArtifactFormat::Svg,
        br#"<svg><text>Diagram Needle</text></svg>"#,
    );

    let targets = ViewerArtifactSearchResolver::resolve_targets("needle", &[node], &[artifact]);

    assert_eq!(1, targets.len());
    assert_eq!(128.0, targets[0].rect.y);
    assert_eq!("Needle", targets[0].matched.text);
    assert_eq!(Some(artifact_id), targets[0].matched.artifact_id);
}

#[test]
fn artifact_text_ignores_artifacts_without_matching_node() {
    let node = node_with_artifact("diagram", ArtifactId("node-artifact".to_string()), 128.0);
    let artifact = text_artifact(
        ArtifactId("other-artifact".to_string()),
        ArtifactFormat::Svg,
        br#"<svg><text>Diagram Needle</text></svg>"#,
    );

    let targets = ViewerArtifactSearchResolver::resolve_targets("needle", &[node], &[artifact]);

    assert!(targets.is_empty());
}

#[test]
fn non_text_artifact_formats_do_not_create_search_targets() {
    let artifact_id = ArtifactId("diagram-png".to_string());
    let node = node_with_artifact("diagram", artifact_id.clone(), 128.0);
    let artifact = text_artifact(artifact_id, ArtifactFormat::Png, b"needle");

    let targets = ViewerArtifactSearchResolver::resolve_targets("needle", &[node], &[artifact]);

    assert!(targets.is_empty());
}

#[test]
fn explicit_artifact_text_extraction_searches_non_text_formats() {
    let artifact_id = ArtifactId("diagram-png".to_string());
    let node = node_with_artifact("diagram", artifact_id.clone(), 128.0);
    let artifact = text_artifact(artifact_id.clone(), ArtifactFormat::Png, b"raster bytes")
        .with_text_extraction("Raster Needle");

    let targets = ViewerArtifactSearchResolver::resolve_targets("needle", &[node], &[artifact]);

    assert_eq!(1, targets.len());
    assert_eq!(128.0, targets[0].rect.y);
    assert_eq!("Needle", targets[0].matched.text);
    assert_eq!(Some(artifact_id), targets[0].matched.artifact_id);
}

#[test]
fn text_extraction_whitespace_is_filtered_out_before_search() {
    let artifact_id = ArtifactId("diagram-empty-text".to_string());
    let node = node_with_artifact("diagram", artifact_id.clone(), 128.0);
    let artifact = text_artifact(artifact_id, ArtifactFormat::Svg, br#"<svg></svg>"#)
        .with_text_extraction("   ");

    let targets = ViewerArtifactSearchResolver::resolve_targets("needle", &[node], &[artifact]);

    assert!(targets.is_empty());
}

#[test]
fn empty_explicit_text_extraction_is_rejected() {
    let artifact = text_artifact(
        ArtifactId("diagram-empty-extraction".to_string()),
        ArtifactFormat::Png,
        b"raster bytes",
    )
    .with_text_extraction("");

    assert_eq!(None, ViewerArtifactSearchResolver::artifact_text(&artifact));
}

#[test]
fn invalid_html_bytes_do_not_parse_text_extraction() {
    let artifact = ArtifactFactory::image_asset_with_id(
        ArtifactId("invalid-html".to_string()),
        ArtifactFormat::Html,
        DocumentId("document".to_string()),
        SourceRevision("rev".to_string()),
        ArtifactBytes {
            bytes: vec![0xff, 0xfe, 0xfd],
        },
        "test",
        ArtifactDiagnostics {
            entries: Vec::new(),
        },
    );

    assert_eq!(None, ViewerArtifactSearchResolver::artifact_text(&artifact));
}

#[test]
fn empty_query_skips_extractions() {
    let artifact_id = ArtifactId("diagram-html".to_string());
    let node = node_with_artifact("diagram", artifact_id.clone(), 128.0);
    let artifact = text_artifact(artifact_id, ArtifactFormat::Html, b"<p>Needle</p>");

    assert!(ViewerArtifactSearchResolver::resolve_targets("", &[node], &[artifact]).is_empty());
}

#[test]
fn node_without_artifact_id_is_skipped() {
    let node = node_without_artifact("diagram", 128.0);

    let artifact = text_artifact(
        ArtifactId("diagram-svg".to_string()),
        ArtifactFormat::Svg,
        b"<svg><text>Diagram Needle</text></svg>",
    );

    assert!(
        ViewerArtifactSearchResolver::resolve_targets("needle", &[node], &[artifact]).is_empty()
    );
}

#[test]
fn artifacts_with_diagnostics_do_not_contribute_search_targets() {
    let artifact_id = ArtifactId("diagram-bad".to_string());
    let node = node_with_artifact("diagram", artifact_id.clone(), 128.0);
    let mut artifact = text_artifact(
        artifact_id,
        ArtifactFormat::Svg,
        b"<svg><text>Diagram Needle</text></svg>",
    );
    artifact
        .manifest
        .diagnostics
        .entries
        .push(crate::ArtifactDiagnostic {
            severity: crate::DiagnosticSeverity::Error,
            code: "E001".to_string(),
            message: "simulated diagnostics".to_string(),
        });

    let targets = ViewerArtifactSearchResolver::resolve_targets("needle", &[node], &[artifact]);
    assert!(targets.is_empty());
}
