use crate::{
    BuildGraph, BuildProfile, BuildRequest, DocumentSnapshotFactory, DocumentSource,
    KdvThemeSnapshot, RenderedDiagram, SourceKind, SourceRevision, SourceUri,
};
use flate2::read::ZlibDecoder;
use katana_markdown_model::{
    CodeBlockRole, KatanaMarkdownModel, KmmDocument, KmmNode, KmmNodeKind, MarkdownInput,
    TextFingerprint,
};
use std::io::Read;

#[path = "export_payload_contract_tests_support_markdowns.rs"]
mod export_payload_contract_tests_support_markdowns;
pub(crate) use export_payload_contract_tests_support_markdowns::ExportPayloadContractTestMarkDowns;

pub(crate) struct ExportPayloadContractTestSupport;

impl ExportPayloadContractTestSupport {
    pub(crate) fn graph_with_rendered_diagram(
        markdown: String,
    ) -> Result<BuildGraph, Box<dyn std::error::Error>> {
        Self::graph_with_rendered_diagram_svg(
            markdown,
            "<svg><text>Rendered diagram</text></svg>".to_string(),
        )
    }

    pub(crate) fn graph_with_rendered_diagram_svg(
        markdown: String,
        svg: String,
    ) -> Result<BuildGraph, Box<dyn std::error::Error>> {
        let graph = Self::graph_from_markdown(markdown)?;
        let node_id = Self::diagram_node_id(&graph.snapshot.document.nodes)?;
        Ok(graph.with_rendered_diagrams(vec![RenderedDiagram {
            node_id,
            kind: "mermaid".to_string(),
            svg,
        }]))
    }

    pub(crate) fn graph_from_markdown(
        markdown: String,
    ) -> Result<BuildGraph, Box<dyn std::error::Error>> {
        let source = DocumentSource {
            uri: SourceUri("file://payload-contract.md".to_string()),
            kind: SourceKind::Markdown,
            revision: SourceRevision("rev-1".to_string()),
            content: markdown,
        };
        let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
            "payload-contract.md",
            source.content.clone(),
        ));
        assert!(document.is_ok());
        let document = document.unwrap_or(fallback_document());
        let snapshot = DocumentSnapshotFactory::from_kmm(source, document);
        Ok(BuildGraph::from_request(&BuildRequest {
            snapshot,
            profile: BuildProfile::markdown_export(),
            theme: KdvThemeSnapshot::katana_light(),
        }))
    }

    pub(crate) fn decoded_png_rgba(bytes: &[u8]) -> Result<Vec<u8>, image::ImageError> {
        Ok(image::load_from_memory(bytes)?.to_rgba8().into_raw())
    }

    pub(crate) fn decoded_pdf_rgb(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let stream = Self::pdf_image_stream(bytes)?;
        let mut decoder = ZlibDecoder::new(stream);
        let mut decoded = Vec::new();
        decoder.read_to_end(&mut decoded)?;
        Ok(decoded)
    }

    pub(crate) fn pdf_image_stream(bytes: &[u8]) -> Result<&[u8], Box<dyn std::error::Error>> {
        let image_object = Self::find_bytes(bytes, PDF_IMAGE_OBJECT_ID.as_bytes())
            .ok_or("missing PDF image object")?;
        let stream_start = Self::find_bytes(&bytes[image_object..], STREAM_START_MARKER.as_bytes())
            .ok_or("missing stream")?
            + image_object
            + STREAM_START_MARKER.len();
        let stream_end = Self::find_bytes(&bytes[stream_start..], STREAM_END_MARKER.as_bytes())
            .ok_or("missing stream end")?
            + stream_start;
        Ok(&bytes[stream_start..stream_end])
    }

    pub(crate) fn surface_rgb(image: &image::RgbaImage) -> Vec<u8> {
        let mut rgb =
            Vec::with_capacity((image.width() * image.height() * RGB_CHANNELS_PER_PIXEL) as usize);
        for pixel in image.pixels() {
            rgb.extend_from_slice(&[pixel[0], pixel[1], pixel[2]]);
        }
        rgb
    }

    pub(crate) fn assert_raw_markdown_is_not_embedded_as_payload_text(format: &str, bytes: &[u8]) {
        for needle in [
            b"[!WARNING]".as_slice(),
            b"```mermaid".as_slice(),
            b"graph TD".as_slice(),
        ] {
            assert!(
                !bytes.windows(needle.len()).any(|window| window == needle),
                "{format} payload still embeds raw marker"
            );
        }
    }

    pub(crate) fn assert_svg_source_is_not_embedded_as_payload_text(format: &str, bytes: &[u8]) {
        for needle in [
            b"#katana-mermaid-svg".as_slice(),
            b"@keyframes".as_slice(),
            b"stroke-dasharray".as_slice(),
            b"font-family".as_slice(),
        ] {
            assert!(
                !bytes.windows(needle.len()).any(|window| window == needle),
                "{format} payload still embeds SVG source"
            );
        }
    }

    pub(crate) fn pdf_page_count(bytes: &[u8]) -> usize {
        String::from_utf8_lossy(bytes)
            .matches("/Type /Page ")
            .count()
    }

    fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack
            .windows(needle.len())
            .position(|window| window == needle)
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
}

fn fallback_document() -> KmmDocument {
    KmmDocument {
        path: std::path::PathBuf::from("payload-contract.md"),
        fingerprint: TextFingerprint {
            algorithm: "manual".to_string(),
            value: "fallback".to_string(),
        },
        nodes: Vec::new(),
    }
}

const PDF_IMAGE_OBJECT_ID: &str = "5 0 obj";
const STREAM_START_MARKER: &str = "stream\n";
const STREAM_END_MARKER: &str = "\nendstream";
const RGB_CHANNELS_PER_PIXEL: u32 = 3;

#[cfg(test)]
#[path = "export_payload_contract_tests_support_tests.rs"]
mod tests;
