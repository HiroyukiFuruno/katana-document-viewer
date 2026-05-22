use super::*;
use crate::export_surface::DocumentSurfaceFactory;
use crate::{
    BuildGraph, BuildProfile, BuildRequest, DocumentSnapshotFactory, DocumentSource,
    KdvThemeSnapshot, RenderedDiagram, SourceKind, SourceRevision, SourceUri,
};
use flate2::read::ZlibDecoder;
use image::GenericImageView;
use katana_markdown_model::{
    CodeBlockRole, KatanaMarkdownModel, KmmNode, KmmNodeKind, MarkdownInput,
};
use std::io::Read;

#[test]
fn non_html_payloads_are_encoded_from_the_evaluated_document_surface()
-> Result<(), Box<dyn std::error::Error>> {
    let theme = KdvThemeSnapshot::katana_light();
    let graph = graph_with_rendered_diagram(contract_markdown())?;
    let surface = DocumentSurfaceFactory::create(&graph, &theme);
    let pdf = ExportPayloadFactory::create(&graph, ExportFormat::Pdf, &theme)?;
    let png = ExportPayloadFactory::create(&graph, ExportFormat::Png, &theme)?;
    let jpeg = ExportPayloadFactory::create(&graph, ExportFormat::Jpeg, &theme)?;

    assert_eq!(decoded_png_rgba(&png)?, surface.image.as_raw().as_slice());
    assert_eq!(decoded_pdf_rgb(&pdf)?, surface_rgb(&surface.pages[0]));
    assert_eq!(
        image::load_from_memory(&jpeg)?.dimensions(),
        surface.image.dimensions()
    );
    assert_raw_markdown_is_not_embedded_as_payload_text("pdf", &pdf);
    assert_raw_markdown_is_not_embedded_as_payload_text("png", &png);
    assert_raw_markdown_is_not_embedded_as_payload_text("jpeg", &jpeg);
    Ok(())
}

#[test]
fn non_html_payloads_do_not_embed_rendered_svg_source_text()
-> Result<(), Box<dyn std::error::Error>> {
    let theme = KdvThemeSnapshot::katana_light();
    let graph = graph_with_rendered_diagram_svg(diagram_markdown(), styled_svg())?;
    let pdf = ExportPayloadFactory::create(&graph, ExportFormat::Pdf, &theme)?;
    let png = ExportPayloadFactory::create(&graph, ExportFormat::Png, &theme)?;
    let jpeg = ExportPayloadFactory::create(&graph, ExportFormat::Jpeg, &theme)?;

    assert_svg_source_is_not_embedded_as_payload_text("pdf", &pdf);
    assert_svg_source_is_not_embedded_as_payload_text("png", &png);
    assert_svg_source_is_not_embedded_as_payload_text("jpeg", &jpeg);
    Ok(())
}

#[test]
fn pdf_payload_paginates_tall_native_surface() -> Result<(), Box<dyn std::error::Error>> {
    let theme = KdvThemeSnapshot::katana_light();
    let graph = graph_from_markdown(tall_markdown())?;
    let pdf = ExportPayloadFactory::create(&graph, ExportFormat::Pdf, &theme)?;

    assert!(
        pdf_page_count(&pdf) > 1,
        "PDF export must paginate tall native surfaces instead of emitting one huge page"
    );
    Ok(())
}

#[test]
fn pdf_payload_contains_link_annotations_for_markdown_links()
-> Result<(), Box<dyn std::error::Error>> {
    let theme = KdvThemeSnapshot::katana_light();
    let graph = graph_from_markdown("[リンク](https://example.com)\n".to_string())?;
    let pdf = ExportPayloadFactory::create(&graph, ExportFormat::Pdf, &theme)?;
    let text = String::from_utf8_lossy(&pdf);

    assert!(text.contains("/Annots ["), "{text}");
    assert!(text.contains("/Subtype /Link"), "{text}");
    assert!(text.contains("/URI (https://example.com)"), "{text}");
    Ok(())
}

fn graph_with_rendered_diagram(markdown: String) -> Result<BuildGraph, Box<dyn std::error::Error>> {
    graph_with_rendered_diagram_svg(
        markdown,
        "<svg><text>Rendered diagram</text></svg>".to_string(),
    )
}

fn graph_with_rendered_diagram_svg(
    markdown: String,
    svg: String,
) -> Result<BuildGraph, Box<dyn std::error::Error>> {
    let graph = graph_from_markdown(markdown)?;
    let node_id = diagram_node_id(&graph.snapshot.document.nodes)?;
    Ok(graph.with_rendered_diagrams(vec![RenderedDiagram {
        node_id,
        kind: "mermaid".to_string(),
        svg,
    }]))
}

fn graph_from_markdown(markdown: String) -> Result<BuildGraph, Box<dyn std::error::Error>> {
    let source = DocumentSource {
        uri: SourceUri("file://payload-contract.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("rev-1".to_string()),
        content: markdown,
    };
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "payload-contract.md",
        source.content.clone(),
    ))?;
    let snapshot = DocumentSnapshotFactory::from_kmm(source, document);
    Ok(BuildGraph::from_request(&BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    }))
}

fn contract_markdown() -> String {
    [
        "# 契約",
        "",
        "**太字** *斜体* ~~取り消し~~ `code` [リンク](https://example.com)",
        "",
        "> [!WARNING]",
        "> **危険** な操作です。",
        "",
        "- [/] 進行中",
        "- [-] 保留",
        "",
        "inline math: $a^2 + b^2 = c^2$",
        "",
        "```mermaid",
        "graph TD",
        "  A --> B",
        "```",
    ]
    .join("\n")
}

fn diagram_markdown() -> String {
    ["# 図形", "", "```mermaid", "graph TD", "  A --> B", "```"].join("\n")
}

fn tall_markdown() -> String {
    let mut lines = vec!["# 長い文書".to_string(), String::new()];
    for index in 1..=120 {
        lines.push(format!(
            "段落 {index}: PDFは巨大な1ページではなく複数ページに分割する。"
        ));
        lines.push(String::new());
    }
    lines.join("\n")
}

fn styled_svg() -> String {
    [
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="240" height="120">"#,
        r#"<style>#katana-mermaid-svg{font-family:trebuchet ms;}@keyframes edge-animation{from{stroke-dasharray:0;}}</style>"#,
        r#"<text x="16" y="48">Rendered diagram</text>"#,
        "</svg>",
    ]
    .join("")
}

fn pdf_page_count(bytes: &[u8]) -> usize {
    String::from_utf8_lossy(bytes)
        .matches("/Type /Page ")
        .count()
}

fn diagram_node_id(nodes: &[KmmNode]) -> Result<String, Box<dyn std::error::Error>> {
    for node in nodes {
        if matches!(
            node.kind,
            KmmNodeKind::CodeBlock(CodeBlockRole::Diagram { .. })
        ) {
            return Ok(node.id.0.clone());
        }
    }
    Err("diagram node is required".into())
}

fn decoded_png_rgba(bytes: &[u8]) -> Result<Vec<u8>, image::ImageError> {
    Ok(image::load_from_memory(bytes)?.to_rgba8().into_raw())
}

fn decoded_pdf_rgb(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let stream = pdf_image_stream(bytes)?;
    let mut decoder = ZlibDecoder::new(stream);
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded)?;
    Ok(decoded)
}

fn pdf_image_stream(bytes: &[u8]) -> Result<&[u8], Box<dyn std::error::Error>> {
    let image_object = find_bytes(bytes, b"5 0 obj").ok_or("missing PDF image object")?;
    let stream_start = find_bytes(&bytes[image_object..], b"stream\n").ok_or("missing stream")?
        + image_object
        + b"stream\n".len();
    let stream_end = find_bytes(&bytes[stream_start..], b"\nendstream")
        .ok_or("missing stream end")?
        + stream_start;
    Ok(&bytes[stream_start..stream_end])
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn surface_rgb(image: &image::RgbaImage) -> Vec<u8> {
    let mut rgb = Vec::with_capacity((image.width() * image.height() * 3) as usize);
    for pixel in image.pixels() {
        rgb.extend_from_slice(&[pixel[0], pixel[1], pixel[2]]);
    }
    rgb
}

fn assert_raw_markdown_is_not_embedded_as_payload_text(format: &str, bytes: &[u8]) {
    for needle in [
        b"[!WARNING]".as_slice(),
        b"```mermaid".as_slice(),
        b"graph TD".as_slice(),
    ] {
        assert!(
            !bytes.windows(needle.len()).any(|window| window == needle),
            "{format} payload still embeds raw marker {:?}",
            String::from_utf8_lossy(needle)
        );
    }
}

fn assert_svg_source_is_not_embedded_as_payload_text(format: &str, bytes: &[u8]) {
    for needle in [
        b"#katana-mermaid-svg".as_slice(),
        b"@keyframes".as_slice(),
        b"stroke-dasharray".as_slice(),
        b"font-family".as_slice(),
    ] {
        assert!(
            !bytes.windows(needle.len()).any(|window| window == needle),
            "{format} payload still embeds SVG source {:?}",
            String::from_utf8_lossy(needle)
        );
    }
}
