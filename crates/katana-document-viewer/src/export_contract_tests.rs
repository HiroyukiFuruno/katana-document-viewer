use super::*;

#[test]
fn export_contract_records_kmm_v0_2_owned_features_as_implemented() {
    let matrix = HtmlExportContractMatrix::v0_1();

    for feature in [
        "commonmark-strong",
        "commonmark-emphasis",
        "commonmark-inline-code",
        "commonmark-link",
        "commonmark-autolink",
        "commonmark-image",
        "commonmark-footnote",
        "commonmark-nested-list",
        "commonmark-blockquote-children",
        "gfm-strikethrough",
        "gfm-task-list",
        "katana-inline-html",
        "math-fenced",
        "math-inline",
        "math-dollar-block",
        "katana-html-entity",
        "krr-mermaid",
        "krr-drawio",
        "krr-plantuml",
    ] {
        assert!(matrix.contains_feature(feature, HtmlExportReadiness::Implemented));
    }
}

#[test]
fn export_contract_records_kdv_owned_gaps() {
    let matrix = HtmlExportContractMatrix::v0_1();

    for feature in ["katana-data-uri-svg", "export-pdf", "export-png-jpeg"] {
        assert!(matrix.contains_feature(feature, HtmlExportReadiness::Implemented));
    }
}

#[test]
fn export_contract_records_krr_backend_gaps() {
    let matrix = HtmlExportContractMatrix::v0_1();

    assert!(matrix.contains_feature("krr-zenuml", HtmlExportReadiness::RequiresKrrRender));
}

#[test]
fn v0_1_html_export_gate_rejects_remaining_gaps() {
    let matrix = HtmlExportContractMatrix::v0_1();
    let gaps: Vec<String> = matrix
        .kdv_owned_incomplete_entries()
        .iter()
        .map(|entry| format!("{}:{:?}", entry.feature, entry.readiness))
        .collect();

    assert!(
        matrix.is_kdv_owned_complete(),
        "v0.1.0 HTML export must not leave KDV-owned gaps while KRR intake remains pending: {gaps:?}"
    );
}

#[test]
fn incomplete_entries_collects_non_implemented_items_only() {
    let matrix = HtmlExportContractMatrix::v0_1();

    let incomplete = matrix.incomplete_entries();

    assert!(incomplete.iter().any(|entry| {
        entry.feature == "krr-zenuml" && entry.readiness == HtmlExportReadiness::RequiresKrrRender
    }));
    assert!(
        incomplete
            .iter()
            .all(|entry| entry.readiness != HtmlExportReadiness::Implemented)
    );
}

#[test]
fn kdv_owned_incomplete_entries_tracks_kdv_gaps() {
    let matrix = HtmlExportContractMatrix::v0_1();
    let kdv_incomplete = matrix.kdv_owned_incomplete_entries();

    assert_eq!(kdv_incomplete.len(), 0);
    assert!(
        !matrix
            .entries
            .iter()
            .any(|entry| entry.readiness == HtmlExportReadiness::RequiresKdvImplementation)
    );
}

#[test]
fn completeness_predicates_distinguish_krr_and_kdv_gaps() {
    let matrix = HtmlExportContractMatrix::v0_1();

    assert!(!matrix.is_complete());
    assert!(matrix.is_kdv_owned_complete());
}
