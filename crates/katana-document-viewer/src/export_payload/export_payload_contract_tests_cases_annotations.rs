use image::GenericImageView;

#[test]
fn pdf_payload_contains_link_annotations_for_markdown_links() {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph = must_graph_from_markdown("[リンク](https://example.com)\n".to_string());
    let pdf = create_payload(&graph, crate::ExportFormat::Pdf, &theme);
    let text = String::from_utf8_lossy(&pdf);

    assert!(text.contains("/Annots ["), "{text}");
    assert!(text.contains("/Subtype /Link"), "{text}");
    assert!(text.contains("/URI (https://example.com)"), "{text}");
}

#[test]
fn pdf_payload_contains_link_annotations_for_generic_html_links() {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph = must_graph_from_markdown(
        r#"<div><a href="https://example.com">HTML link</a></div>"#.to_string(),
    );
    let pdf = create_payload(&graph, crate::ExportFormat::Pdf, &theme);
    let text = String::from_utf8_lossy(&pdf);

    assert!(text.contains("/Annots ["), "{text}");
    assert!(text.contains("/Subtype /Link"), "{text}");
    assert!(text.contains("/URI (https://example.com)"), "{text}");
}

#[test]
fn export_formats_share_surface_semantics_with_declared_interaction_exceptions() {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph = must_graph_from_markdown(
        super::support::ExportPayloadContractTestMarkDowns::interaction_exception_markdown(),
    );
    let surface = crate::export_surface::DocumentSurfaceFactory::create(&graph, &theme);
    let payloads = all_payloads(&graph, &theme);
    let html_text = String::from_utf8_lossy(&payloads.html);
    let pdf_text = String::from_utf8_lossy(&payloads.pdf);

    assert_interaction_html_contract(&html_text);
    assert_interaction_pdf_contract(&pdf_text);
    assert_eq!(
        must_decoded_png_rgba(&payloads.png),
        surface.image.as_raw().as_slice()
    );
    assert_eq!(
        must_image_dimensions(&payloads.jpeg),
        surface.image.dimensions()
    );
    assert_non_pdf_payloads_do_not_embed_raw_markdown(&payloads);
}

fn assert_interaction_html_contract(html_text: &str) {
    assert!(html_text.contains(r#"<details data-kdv-accordion="true" open>"#));
    assert!(html_text.contains(r#"href="https://example.com""#));
}

fn assert_interaction_pdf_contract(pdf_text: &str) {
    assert!(pdf_text.contains("/Subtype /Link"), "{pdf_text}");
    assert!(
        pdf_text.contains("/URI (https://example.com)"),
        "{pdf_text}"
    );
}

fn assert_non_pdf_payloads_do_not_embed_raw_markdown(payloads: &AllPayloads) {
    super::support::ExportPayloadContractTestSupport::assert_raw_markdown_is_not_embedded_as_payload_text(
        "png",
        &payloads.png,
    );
    super::support::ExportPayloadContractTestSupport::assert_raw_markdown_is_not_embedded_as_payload_text(
        "jpeg",
        &payloads.jpeg,
    );
}

#[test]
fn pdf_payload_preserves_all_footnote_links_as_internal_targets() {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph = must_graph_from_markdown(
        super::support::ExportPayloadContractTestMarkDowns::multi_footnote_markdown(),
    );
    let payloads = all_payloads(&graph, &theme);
    let text = String::from_utf8_lossy(&payloads.pdf);

    assert!(
        text.matches("/Dest [").count() >= 4,
        "each footnote reference and backlink must become internal destinations: {text}"
    );
    assert!(text.contains("/F 4"), "{text}");
    assert!(!text.contains("/S /URI"), "{text}");
    assert!(!text.contains("/S /GoTo"), "{text}");
    assert_image_payloads_do_not_include_pdf_annotations(&payloads);
}

struct AllPayloads {
    html: Vec<u8>,
    pdf: Vec<u8>,
    png: Vec<u8>,
    jpeg: Vec<u8>,
}

fn all_payloads(graph: &crate::forge::BuildGraph, theme: &crate::KdvThemeSnapshot) -> AllPayloads {
    AllPayloads {
        html: create_payload(graph, crate::ExportFormat::Html, theme),
        pdf: create_payload(graph, crate::ExportFormat::Pdf, theme),
        png: create_payload(graph, crate::ExportFormat::Png, theme),
        jpeg: create_payload(graph, crate::ExportFormat::Jpeg, theme),
    }
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

fn assert_image_payloads_do_not_include_pdf_annotations(payloads: &AllPayloads) {
    let png_text = String::from_utf8_lossy(&payloads.png);
    let jpeg_text = String::from_utf8_lossy(&payloads.jpeg);
    assert!(
        !png_text.contains("/Subtype /Link"),
        "png payload must not include PDF link annotations"
    );
    assert!(
        !jpeg_text.contains("/Subtype /Link"),
        "jpeg payload must not include PDF link annotations"
    );
}

fn must_graph_from_markdown(markdown: String) -> crate::forge::BuildGraph {
    let graph = super::support::ExportPayloadContractTestSupport::graph_from_markdown(markdown);
    assert!(graph.is_ok());
    graph.unwrap_or(fallback_graph())
}

fn must_decoded_png_rgba(bytes: &[u8]) -> Vec<u8> {
    let decoded = super::support::ExportPayloadContractTestSupport::decoded_png_rgba(bytes);
    let failure = format!("{:?}", decoded.as_ref().err());
    assert!(decoded.is_ok(), "png decode failed: {failure}");
    decoded.ok().into_iter().flatten().collect()
}

fn must_image_dimensions(bytes: &[u8]) -> (u32, u32) {
    let decoded = image::load_from_memory(bytes);
    assert!(decoded.is_ok());
    decoded.map(|image| image.dimensions()).unwrap_or((0, 0))
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
