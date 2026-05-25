use image::GenericImageView;

#[test]
fn non_html_payloads_are_encoded_from_the_evaluated_document_surface()
-> Result<(), Box<dyn std::error::Error>> {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph = super::support::ExportPayloadContractTestSupport::graph_with_rendered_diagram(
        super::support::ExportPayloadContractTestMarkDowns::contract_markdown(),
    )?;
    let surface = crate::export_surface::DocumentSurfaceFactory::create(&graph, &theme);
    let payloads = non_html_payloads(&graph, &theme)?;

    assert_eq!(
        super::support::ExportPayloadContractTestSupport::decoded_png_rgba(&payloads.png)?,
        surface.image.as_raw().as_slice()
    );
    assert_eq!(
        super::support::ExportPayloadContractTestSupport::decoded_pdf_rgb(&payloads.pdf)?,
        super::support::ExportPayloadContractTestSupport::surface_rgb(&surface.pages[0])
    );
    assert_eq!(
        image::load_from_memory(&payloads.jpeg)?.dimensions(),
        surface.image.dimensions()
    );
    assert_payloads_do_not_embed_raw_markdown(&payloads);
    Ok(())
}

#[test]
fn non_html_payloads_do_not_embed_rendered_svg_source_text()
-> Result<(), Box<dyn std::error::Error>> {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph = super::support::ExportPayloadContractTestSupport::graph_with_rendered_diagram_svg(
        super::support::ExportPayloadContractTestMarkDowns::diagram_markdown(),
        styled_svg(),
    )?;
    let payloads = non_html_payloads(&graph, &theme)?;

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
    Ok(())
}

struct NonHtmlPayloads {
    pdf: Vec<u8>,
    png: Vec<u8>,
    jpeg: Vec<u8>,
}

fn non_html_payloads(
    graph: &crate::forge::BuildGraph,
    theme: &crate::KdvThemeSnapshot,
) -> Result<NonHtmlPayloads, Box<dyn std::error::Error>> {
    Ok(NonHtmlPayloads {
        pdf: create_payload(graph, crate::ExportFormat::Pdf, theme)?,
        png: create_payload(graph, crate::ExportFormat::Png, theme)?,
        jpeg: create_payload(graph, crate::ExportFormat::Jpeg, theme)?,
    })
}

fn create_payload(
    graph: &crate::forge::BuildGraph,
    format: crate::ExportFormat,
    theme: &crate::KdvThemeSnapshot,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(crate::export_payload::ExportPayloadFactory::create(
        graph, format, theme,
    )?)
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
fn pdf_payload_paginates_tall_native_surface() -> Result<(), Box<dyn std::error::Error>> {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph = super::support::ExportPayloadContractTestSupport::graph_from_markdown(
        super::support::ExportPayloadContractTestMarkDowns::tall_markdown(),
    )?;
    let pdf = crate::export_payload::ExportPayloadFactory::create(
        &graph,
        crate::ExportFormat::Pdf,
        &theme,
    )?;

    assert!(
        super::support::ExportPayloadContractTestSupport::pdf_page_count(&pdf) > 1,
        "PDF export must paginate tall native surfaces instead of emitting one huge page"
    );
    Ok(())
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
