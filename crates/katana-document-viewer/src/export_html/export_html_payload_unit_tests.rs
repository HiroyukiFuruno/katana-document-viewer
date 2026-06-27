use super::*;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{
    ByteRange, FootnoteDefinitionNode, KatanaMarkdownModel, KmmNode, KmmNodeId, KmmNodeKind,
    LineColumn, LineColumnRange, MarkdownInput, RawSnippet, SourceSpan, TextSpan,
};

#[test]
fn append_node_uses_footnote_definition_path() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();
    let node = KmmNode {
        id: KmmNodeId("node-footnote".to_string()),
        kind: KmmNodeKind::FootnoteDefinition(FootnoteDefinitionNode {
            label: "1".to_string(),
            text: "note".to_string(),
        }),
        source: source_span(""),
        children: Vec::new(),
    };

    HtmlExportPayloadFactory::append_node(&mut html, &graph, &theme, &node);

    assert!(html.contains("<section id=\"fn-1\""));
    assert!(html.contains("note"));
}

#[test]
fn append_paragraph_uses_node_children_when_present() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let child = KmmNode {
        id: KmmNodeId("child".to_string()),
        kind: KmmNodeKind::Text(TextSpan {
            text: "inline".to_string(),
        }),
        source: source_span("inline"),
        children: Vec::new(),
    };
    let node = KmmNode {
        id: KmmNodeId("paragraph".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span("inline"),
        children: vec![child],
    };
    append_paragraph(&mut html, &node, &theme);
    assert!(html.starts_with("<p>"));
    assert!(html.contains("inline"));
}

#[test]
fn append_paragraph_uses_raw_text_when_children_missing() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let node = KmmNode {
        id: KmmNodeId("paragraph-empty".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span("&lt;"),
        children: Vec::new(),
    };

    append_paragraph(&mut html, &node, &theme);
    assert!(html.contains("&lt;"));
}

#[test]
fn diagram_error_for_node_returns_matching_diagnostic_message() {
    let mut graph = graph();
    graph
        .diagnostics
        .messages
        .push("diagram node-diagram failed".to_string());

    assert_eq!(
        "diagram node-diagram failed",
        diagram_error_for_node(&graph, "node-diagram")
    );
}

fn graph() -> BuildGraph {
    use crate::{BuildProfile, BuildRequest, DocumentSnapshotFactory, DocumentSource, SourceKind};
    let source = DocumentSource {
        uri: crate::SourceUri("file:///test.md".to_string()),
        kind: SourceKind::Markdown,
        revision: crate::SourceRevision("r".to_string()),
        content: "x".to_string(),
    };
    let snapshot = DocumentSnapshotFactory::from_kmm(
        source.clone(),
        match KatanaMarkdownModel::parse(MarkdownInput::from_content(
            "test.md",
            source.content.clone(),
        )) {
            Ok(model) => model,
            Err(error) => {
                std::panic::resume_unwind(Box::new(format!("parse test markdown: {error}")))
            }
        },
    );
    BuildGraph::from_request(&BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme: crate::KdvThemeSnapshot::katana_light(),
    })
}

fn source_span(text: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: text.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: text.len() + 1,
            },
        },
        raw: RawSnippet {
            text: text.to_string(),
        },
    }
}
