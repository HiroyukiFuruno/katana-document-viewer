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
    assert_pdf_export_fidelity_contract(&surface, &graph, &theme, &pdf, &pdf_text);
    assert_html_visible_content_matches_pdf_surface(&html_text, &graph, &theme);
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
    pdf: &[u8],
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
    assert_eq!(
        must_decoded_pdf_rgb(pdf),
        super::support::ExportPayloadContractTestSupport::surface_rgb(&surface.pages[0]),
        "PDF image stream must render the same surface used for export fidelity"
    );
}

fn assert_html_visible_content_matches_pdf_surface(
    html_text: &str,
    graph: &crate::forge::BuildGraph,
    theme: &crate::KdvThemeSnapshot,
) {
    let html_visible = normalized_visible_text(html_visible_text(html_text));
    let surface_visible = normalized_visible_text(
        crate::export_surface::SurfaceBlockFactory::create(graph, theme)
            .iter()
            .map(crate::export_surface::SurfaceBlock::text_for_tests)
            .collect::<Vec<_>>()
            .join("\n"),
    );
    let expected = [
        "Export fidelity repro",
        "A normal markdown link: KatanA",
        "A bare URL that should be link-capable: https://example.com/docs",
        "Feature Expected Link",
        "PDF table rendered as table grid with header/body styling PDF docs",
        "HTML link clickable anchor https://example.com/html",
    ];

    assert_visible_sequence("HTML export", &html_visible, &expected);
    assert_visible_sequence("PDF surface", &surface_visible, &expected);
}

fn assert_visible_sequence(label: &str, visible: &str, expected: &[&str]) {
    let mut offset = 0usize;
    for token in expected {
        let index = visible[offset..].find(token);
        assert!(
            index.is_some(),
            "{label} missing visible token `{token}` in `{visible}`"
        );
        let index = index.unwrap_or(0);
        offset += index + token.len();
    }
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

fn must_decoded_pdf_rgb(bytes: &[u8]) -> Vec<u8> {
    let decoded = super::support::ExportPayloadContractTestSupport::decoded_pdf_rgb(bytes);
    let failure = format!("{:?}", decoded.as_ref().err());
    assert!(decoded.is_ok(), "pdf decode failed: {failure}");
    decoded.ok().into_iter().flatten().collect()
}

fn html_visible_text(html: &str) -> String {
    let body = html
        .split_once("<body")
        .and_then(|(_, rest)| rest.split_once('>'))
        .map(|(_, rest)| rest)
        .unwrap_or(html);
    let body = body.split_once("</body>").map_or(body, |(body, _)| body);
    let mut text = String::with_capacity(body.len());
    let mut in_tag = false;
    for character in body.chars() {
        match character {
            '<' => {
                in_tag = true;
                text.push(' ');
            }
            '>' => {
                in_tag = false;
                text.push(' ');
            }
            _ if !in_tag => text.push(character),
            _ => {}
        }
    }
    decode_basic_html_entities(&text)
}

fn decode_basic_html_entities(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

fn normalized_visible_text(value: String) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
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
