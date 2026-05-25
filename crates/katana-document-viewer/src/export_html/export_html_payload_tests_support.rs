use crate::{BuildGraph, BuildProfile, BuildRequest, DocumentSnapshotFactory, DocumentSource};
use crate::{SourceKind, SourceRevision, SourceUri};
use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};

use crate::export_html_payload::HtmlExportPayloadFactory;

pub(super) fn export_html(markdown: &str) -> Result<String, Box<dyn std::error::Error>> {
    let graph = build_graph(markdown)?;
    let bytes = HtmlExportPayloadFactory::create(&graph, &crate::KdvThemeSnapshot::katana_light());
    Ok(String::from_utf8(bytes)?)
}

pub(super) fn build_graph(markdown: &str) -> Result<BuildGraph, Box<dyn std::error::Error>> {
    build_graph_with_uri(markdown, SourceUri("file:///test.md".to_string()))
}

pub(super) fn build_graph_with_uri(
    markdown: &str,
    uri: SourceUri,
) -> Result<BuildGraph, Box<dyn std::error::Error>> {
    let source = DocumentSource {
        uri,
        kind: SourceKind::Markdown,
        revision: SourceRevision("test".to_string()),
        content: markdown.to_string(),
    };
    let document =
        KatanaMarkdownModel::parse(MarkdownInput::from_content("test.md", markdown.to_string()))?;
    let snapshot = DocumentSnapshotFactory::from_kmm(source, document);
    Ok(BuildGraph::from_request(&BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme: crate::KdvThemeSnapshot::katana_light(),
    }))
}
