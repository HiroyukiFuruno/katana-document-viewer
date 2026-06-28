use image::GenericImageView;

#[test]
fn non_html_payloads_use_wrapped_surface_for_japanese_no_space_paragraphs()
-> Result<(), Box<dyn std::error::Error>> {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph = must_graph_from_markdown(
        super::support::ExportPayloadContractTestMarkDowns::japanese_overlap_repro_markdown(),
    );
    let blocks = crate::export_surface::SurfaceBlockFactory::create(&graph, &theme);

    assert_japanese_paragraph_reserves_wrapped_lines(&blocks)?;
    assert_non_html_payloads_share_surface(&graph, &theme);
    Ok(())
}

fn assert_japanese_paragraph_reserves_wrapped_lines(
    blocks: &[crate::export_surface::SurfaceBlock],
) -> Result<(), Box<dyn std::error::Error>> {
    let paragraph_line = first_surface_line_index(blocks, "これはPDF出力")
        .ok_or_else(|| std::io::Error::other("Japanese paragraph should be emitted"))?;
    let next_heading_line = first_surface_line_index(blocks, "次の見出し")
        .ok_or_else(|| std::io::Error::other("next heading should be emitted"))?;
    assert!(
        next_heading_line > paragraph_line + 1,
        "Japanese no-space paragraph must reserve wrapped surface lines before next heading"
    );
    Ok(())
}

fn assert_non_html_payloads_share_surface(
    graph: &crate::forge::BuildGraph,
    theme: &crate::KdvThemeSnapshot,
) {
    let surface = crate::export_surface::DocumentSurfaceFactory::create(graph, theme);
    let pdf = create_payload(graph, crate::ExportFormat::Pdf, theme);
    let png = create_payload(graph, crate::ExportFormat::Png, theme);
    let jpeg = create_payload(graph, crate::ExportFormat::Jpeg, theme);

    assert_eq!(
        must_decoded_png_rgba(&png),
        surface.image.as_raw().as_slice()
    );
    assert_eq!(
        must_decoded_pdf_rgb(&pdf),
        super::support::ExportPayloadContractTestSupport::surface_rgb(&surface.pages[0])
    );
    assert_eq!(must_image_dimensions(&jpeg), surface.image.dimensions());
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

fn first_surface_line_index(
    blocks: &[crate::export_surface::SurfaceBlock],
    marker: &str,
) -> Option<usize> {
    blocks
        .iter()
        .enumerate()
        .find_map(|(index, block)| match block {
            crate::export_surface::SurfaceBlock::Line(line) if line.text.contains(marker) => {
                Some(index)
            }
            _ => None,
        })
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
