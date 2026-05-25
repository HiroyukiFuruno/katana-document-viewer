use super::*;

#[test]
fn fixture_matrix_covers_required_categories() {
    let matrix = EvaluationFixtureMatrix::v0_1();

    for category in [
        FixtureCategory::CommonMark,
        FixtureCategory::Gfm,
        FixtureCategory::Math,
        FixtureCategory::GitHubAlert,
        FixtureCategory::KatanaCompatibility,
        FixtureCategory::ExternalRendering,
    ] {
        assert!(matrix.contains_category(category));
    }
}

#[test]
fn fixture_matrix_records_current_coverage_gaps() {
    let matrix = EvaluationFixtureMatrix::v0_1();

    assert!(matrix.status_count(CoverageStatus::MissingImplementation) > 0);
    assert_eq!(
        matrix.status_count(CoverageStatus::ExternalBackendRequired),
        0
    );
}

#[test]
fn fixture_matrix_paths_exist() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for fixture in EvaluationFixtureMatrix::v0_1().fixtures {
        assert!(root.join(fixture.path).exists());
    }
}

#[test]
fn backend_matrix_keeps_krr_and_kdv_export_roles_separate() {
    let matrix = BackendCapabilityMatrix::v0_1();

    assert!(matrix.capabilities.iter().any(|entry| {
        entry.feature == "mermaid" && entry.capability == BackendCapability::KrrDirect
    }));
    assert!(matrix.capabilities.iter().any(|entry| {
        entry.feature == "html-export" && entry.capability == BackendCapability::KdvManifestExport
    }));
    assert!(matrix.capabilities.iter().any(|entry| {
        entry.feature == "plantuml" && entry.capability == BackendCapability::KrrDirect
    }));
    assert!(matrix.capabilities.iter().any(|entry| {
        entry.feature == "math" && entry.capability == BackendCapability::KdvManifestExport
    }));
}

#[test]
fn coverage_matrix_marks_krr_adoption_as_complete() {
    let matrix = EvaluationCoverageMatrix::v0_1();

    assert!(matrix.is_complete());
    assert!(matrix.contains_feature("commonmark-emphasis", CoverageStatus::KmmDto));
    assert!(matrix.contains_feature("github-alert-note", CoverageStatus::KmmDto));
    assert!(matrix.contains_feature(
        "zenuml-mermaid-compat",
        CoverageStatus::KrrMermaidCompatibility
    ));
    assert!(matrix.contains_feature("plantuml-render", CoverageStatus::KrrDirect));
}

#[test]
fn v0_1_release_gate_rejects_remaining_markdown_coverage_gaps() {
    let matrix = EvaluationCoverageMatrix::v0_1();

    assert!(
        matrix.is_kdv_owned_complete(),
        "v0.1.0 must not leave KDV-owned coverage gaps while KRR intake remains pending: {:?}",
        kdv_owned_incomplete_features(&matrix)
    );
}

fn kdv_owned_incomplete_features(matrix: &EvaluationCoverageMatrix) -> Vec<String> {
    matrix
        .features
        .iter()
        .filter(|feature| feature.status == CoverageStatus::MissingImplementation)
        .map(|feature| feature.id.clone())
        .collect()
}
