#[test]
fn export_payload_preserves_markdown_links_and_table_link_fidelity() {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph = must_graph_from_markdown(
        super::support::ExportPayloadContractTestMarkDowns::export_fidelity_repro_markdown(),
    );
    let surface = crate::export_surface::DocumentSurfaceFactory::create(&graph, &theme);
    let html = create_payload(&graph, crate::ExportFormat::Html, &theme);
    let pdf = create_payload(&graph, crate::ExportFormat::Pdf, &theme);
    let html_text = String::from_utf8_lossy(&html);
    let pdf_text = String::from_utf8_lossy(&pdf);

    assert_html_export_fidelity_contract(&html_text);
    assert_pdf_export_fidelity_contract(&surface, &graph, &theme, &pdf_text);
}

fn assert_html_export_fidelity_contract(html_text: &str) {
    assert!(
        html_text.contains(r#"<a href="https://example.com/katana">KatanA</a>"#),
        "{html_text}"
    );
    assert!(
        html_text.contains(r#"<table data-kdv-table="katana">"#),
        "{html_text}"
    );
    assert!(
        html_text.contains(r#"<a href="https://example.com/pdf">PDF docs</a>"#),
        "{html_text}"
    );
    assert!(
        html_text.contains(r#"<a href="https://example.com/html""#),
        "{html_text}"
    );
}

fn assert_pdf_export_fidelity_contract(
    surface: &crate::export_surface::DocumentSurface,
    graph: &crate::forge::BuildGraph,
    theme: &crate::KdvThemeSnapshot,
    pdf_text: &str,
) {
    assert!(
        surface_has_table_block(graph, theme),
        "PDF surface must preserve markdown tables as table blocks"
    );
    assert!(
        surface
            .link_annotations
            .iter()
            .any(|annotation| annotation.target == "https://example.com/pdf"),
        "PDF surface must retain links inside table cells"
    );
    assert!(pdf_text.contains("/URI (https://example.com/katana)"));
    assert!(pdf_text.contains("/URI (https://example.com/pdf)"));
    assert!(pdf_text.matches("/Subtype /Link").count() >= 2);
}

fn surface_has_table_block(
    graph: &crate::forge::BuildGraph,
    theme: &crate::KdvThemeSnapshot,
) -> bool {
    crate::export_surface::SurfaceBlockFactory::create(graph, theme)
        .iter()
        .any(|block| matches!(block, crate::export_surface::SurfaceBlock::Table(_)))
}

fn create_payload(
    graph: &crate::forge::BuildGraph,
    format: crate::ExportFormat,
    theme: &crate::KdvThemeSnapshot,
) -> Vec<u8> {
    let payload = crate::export_payload::ExportPayloadFactory::create(graph, format, theme);
    let failure = format!("{:?}", payload.as_ref().err());
    assert!(
        payload.is_ok(),
        "payload creation failed for {format:?}: {failure}"
    );
    payload.ok().into_iter().flatten().collect()
}

fn must_graph_from_markdown(markdown: String) -> crate::forge::BuildGraph {
    let graph = super::support::ExportPayloadContractTestSupport::graph_from_markdown(markdown);
    assert!(graph.is_ok());
    graph.unwrap_or(fallback_graph())
}

fn fallback_graph() -> crate::forge::BuildGraph {
    let source = crate::DocumentSource {
        uri: crate::SourceUri("file:///fallback.md".to_string()),
        kind: crate::SourceKind::Markdown,
        revision: crate::SourceRevision("fallback".to_string()),
        content: String::new(),
    };
    let document = katana_markdown_model::KmmDocument {
        path: std::path::PathBuf::from("fallback.md"),
        fingerprint: katana_markdown_model::TextFingerprint {
            algorithm: "manual".to_string(),
            value: "fallback".to_string(),
        },
        nodes: Vec::new(),
    };
    let snapshot = crate::DocumentSnapshotFactory::from_kmm(source, document);
    crate::BuildGraph::from_request(&crate::BuildRequest {
        snapshot,
        profile: crate::BuildProfile::markdown_export(),
        theme: crate::KdvThemeSnapshot::katana_light(),
    })
}
