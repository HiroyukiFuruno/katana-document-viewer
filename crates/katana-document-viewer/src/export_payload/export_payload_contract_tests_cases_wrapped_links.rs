#[test]
fn pdf_payload_splits_wrapped_table_link_annotations_for_long_urls()
-> Result<(), Box<dyn std::error::Error>> {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let target = long_link_target();
    let graph = super::support::ExportPayloadContractTestSupport::graph_from_markdown(
        table_link_markdown(target),
    )?;
    let surface = crate::export_surface::DocumentSurfaceFactory::create(&graph, &theme);
    let matching = matching_annotations(&surface, target);

    assert!(
        matching.len() >= 2,
        "wrapped table links must create one PDF annotation per visible link segment: {}",
        matching.len()
    );
    assert!(
        matching.iter().all(|annotation| annotation.width > 0),
        "each wrapped link annotation must cover visible width"
    );
    assert_pdf_contains_wrapped_link_annotations(&graph, &theme, target, matching.len());
    Ok(())
}

fn matching_annotations<'a>(
    surface: &'a crate::export_surface::DocumentSurface,
    target: &str,
) -> Vec<&'a crate::export_surface::SurfaceLinkAnnotation> {
    surface
        .link_annotations
        .iter()
        .filter(|annotation| annotation.target == target)
        .collect()
}

fn assert_pdf_contains_wrapped_link_annotations(
    graph: &crate::forge::BuildGraph,
    theme: &crate::KdvThemeSnapshot,
    target: &str,
    expected_count: usize,
) {
    let pdf = create_payload(graph, crate::ExportFormat::Pdf, theme);
    let text = String::from_utf8_lossy(&pdf);
    assert!(
        text.matches(&format!("/URI ({target})")).count() >= expected_count,
        "PDF must emit every wrapped table link annotation: {text}"
    );
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

fn table_link_markdown(target: &str) -> String {
    format!("| Profile |\n| --- |\n| [{target}]({target}) |\n")
}

fn long_link_target() -> &'static str {
    "https://www.linkedin.com/in/katana-document-viewer-export-long-link-annotation-regression-case-with-wrapped-segments-and-pdf-uri-coverage"
}
