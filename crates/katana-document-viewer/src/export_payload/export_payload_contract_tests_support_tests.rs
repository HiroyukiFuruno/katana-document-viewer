use super::*;

#[test]
fn graph_without_diagram_node_reports_missing_node() {
    let source = DocumentSource {
        uri: SourceUri("file:///diagram-contract.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("revision".to_string()),
        content: "# no diagram".to_string(),
    };
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "diagram-contract.md",
        source.content.clone(),
    ));
    assert!(document.is_ok());
    let document = document.unwrap_or(fallback_document());
    let snapshot = DocumentSnapshotFactory::from_kmm(source, document);

    assert!(ExportPayloadContractTestSupport::diagram_node_id(&snapshot.document.nodes).is_err());
}

#[test]
fn rendered_diagram_helper_reports_missing_diagram_node() {
    let result = ExportPayloadContractTestSupport::graph_with_rendered_diagram_svg(
        "# no diagram".to_string(),
        "<svg/>".to_string(),
    );

    assert!(result.is_err());
}

#[test]
fn find_bytes_reports_none_for_missing_needle() {
    assert!(ExportPayloadContractTestSupport::find_bytes(b"hello", b"world").is_none());
    assert_eq!(
        ExportPayloadContractTestSupport::find_bytes(b"hello", b"ll"),
        Some(2)
    );
}

#[test]
fn pdf_image_stream_reports_missing_sections() {
    assert!(ExportPayloadContractTestSupport::pdf_image_stream(b"").is_err());
    assert!(ExportPayloadContractTestSupport::pdf_image_stream(b"5 0 obj").is_err());
    assert!(ExportPayloadContractTestSupport::pdf_image_stream(b"5 0 obj\nstream\nabc").is_err());
}

#[test]
fn binary_decoders_report_invalid_payloads() {
    assert!(ExportPayloadContractTestSupport::decoded_png_rgba(b"not png").is_err());
    assert!(ExportPayloadContractTestSupport::decoded_pdf_rgb(b"not pdf").is_err());
    assert!(
        ExportPayloadContractTestSupport::decoded_pdf_rgb(b"5 0 obj\nstream\nbad\nendstream")
            .is_err()
    );
}

fn fallback_document() -> katana_markdown_model::KmmDocument {
    katana_markdown_model::KmmDocument {
        path: std::path::PathBuf::from("diagram-contract.md"),
        fingerprint: katana_markdown_model::TextFingerprint {
            algorithm: "manual".to_string(),
            value: "fallback".to_string(),
        },
        nodes: Vec::new(),
    }
}
