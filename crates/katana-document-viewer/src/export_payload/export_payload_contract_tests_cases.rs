use image::GenericImageView;

#[test]
fn non_html_payloads_are_encoded_from_the_evaluated_document_surface() {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph = must_graph_with_rendered_diagram(
        super::support::ExportPayloadContractTestMarkDowns::contract_markdown(),
    );
    let surface = crate::export_surface::DocumentSurfaceFactory::create(&graph, &theme);
    let payloads = non_html_payloads(&graph, &theme);

    assert_eq!(
        must_decoded_png_rgba(&payloads.png),
        surface.image.as_raw().as_slice()
    );
    assert_eq!(
        must_decoded_pdf_rgb(&payloads.pdf),
        super::support::ExportPayloadContractTestSupport::surface_rgb(&surface.pages[0])
    );
    assert_eq!(
        must_image_dimensions(&payloads.jpeg),
        surface.image.dimensions()
    );
    assert_payloads_do_not_embed_raw_markdown(&payloads);
}

#[test]
fn non_html_payloads_do_not_embed_rendered_svg_source_text() {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph = must_graph_with_rendered_diagram_svg(
        super::support::ExportPayloadContractTestMarkDowns::diagram_markdown(),
        styled_svg(),
    );
    let payloads = non_html_payloads(&graph, &theme);

    super::support::ExportPayloadContractTestSupport::assert_svg_source_is_not_embedded_as_payload_text(
        "pdf",
        &payloads.pdf,
    );
    super::support::ExportPayloadContractTestSupport::assert_svg_source_is_not_embedded_as_payload_text(
        "png",
        &payloads.png,
    );
    super::support::ExportPayloadContractTestSupport::assert_svg_source_is_not_embedded_as_payload_text(
        "jpeg",
        &payloads.jpeg,
    );
}

struct NonHtmlPayloads {
    pdf: Vec<u8>,
    png: Vec<u8>,
    jpeg: Vec<u8>,
}

fn non_html_payloads(
    graph: &crate::forge::BuildGraph,
    theme: &crate::KdvThemeSnapshot,
) -> NonHtmlPayloads {
    NonHtmlPayloads {
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

fn assert_payloads_do_not_embed_raw_markdown(payloads: &NonHtmlPayloads) {
    super::support::ExportPayloadContractTestSupport::assert_raw_markdown_is_not_embedded_as_payload_text(
        "pdf",
        &payloads.pdf,
    );
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
fn pdf_payload_paginates_tall_native_surface() {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph = must_graph_from_markdown(
        super::support::ExportPayloadContractTestMarkDowns::tall_markdown(),
    );
    let pdf = create_payload(&graph, crate::ExportFormat::Pdf, &theme);

    assert!(
        super::support::ExportPayloadContractTestSupport::pdf_page_count(&pdf) > 1,
        "PDF export must paginate tall native surfaces instead of emitting one huge page"
    );
}

fn must_graph_with_rendered_diagram(markdown: String) -> crate::forge::BuildGraph {
    let graph =
        super::support::ExportPayloadContractTestSupport::graph_with_rendered_diagram(markdown);
    assert!(graph.is_ok());
    graph.unwrap_or(fallback_graph())
}

fn must_graph_with_rendered_diagram_svg(markdown: String, svg: String) -> crate::forge::BuildGraph {
    let graph = super::support::ExportPayloadContractTestSupport::graph_with_rendered_diagram_svg(
        markdown, svg,
    );
    assert!(graph.is_ok());
    graph.unwrap_or(fallback_graph())
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

fn must_decoded_pdf_rgb(bytes: &[u8]) -> Vec<u8> {
    let decoded = super::support::ExportPayloadContractTestSupport::decoded_pdf_rgb(bytes);
    let failure = format!("{:?}", decoded.as_ref().err());
    assert!(decoded.is_ok(), "pdf decode failed: {failure}");
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

fn styled_svg() -> String {
    r###"<svg xmlns="http://www.w3.org/2000/svg">
        <style>
            .bg {
                animation-name: pulse;
                animation-delay: 1s;
                animation-duration: 2s;
                animation-iteration-count: infinite;
            }
        </style>
        <rect class="bg" width="200" height="100" fill="#1f2937" />
    </svg>"###
        .to_string()
}
