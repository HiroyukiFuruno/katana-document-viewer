use super::*;
use crate::theme::KdvThemeSnapshot;
use crate::{
    BuildProfile, BuildRequest, DocumentSnapshotFactory, DocumentSource, SourceKind, SourceRevision,
};
use katana_markdown_model::{
    ByteRange, KatanaMarkdownModel, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange,
    MarkdownInput, RawSnippet, SourceSpan, TextSpan,
};

#[test]
fn append_uses_start_attribute_when_ordered() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();
    let item = list_item("item", Some(3), None);
    ListHtmlWriter::append(&mut html, &graph, &theme, true, &[item], "fallback");
    assert!(html.starts_with("<ol start=\"3\">"));
}

#[test]
fn append_falls_back_for_empty_list() {
    let mut html = String::new();
    let graph = graph();
    let theme = KdvThemeSnapshot::katana_light();
    ListHtmlWriter::append(&mut html, &graph, &theme, false, &[], "fallback");
    assert_eq!(html, "<ul><li>fallback</li></ul>\n");
}

#[test]
fn append_task_marker_state_all_variants() {
    let mut html = String::new();
    ListHtmlWriter::append_task_marker(&mut html, "[x]");
    ListHtmlWriter::append_task_marker(&mut html, "[-]");
    ListHtmlWriter::append_task_marker(&mut html, "[/]");
    ListHtmlWriter::append_task_marker(&mut html, "[?]");
    assert!(html.contains("done"));
    assert!(html.contains("in-progress"));
    assert!(html.contains("todo"));
}

fn graph() -> BuildGraph {
    let source = DocumentSource {
        uri: crate::SourceUri("file:///test.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("r".to_string()),
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
        theme: KdvThemeSnapshot::katana_light(),
    })
}

fn list_item(text: &str, ordered_number: Option<usize>, task_marker: Option<&str>) -> ListItemNode {
    ListItemNode {
        marker: "-".to_string(),
        ordered_number,
        task_marker: task_marker.map(str::to_string),
        body: vec![KmmNode {
            id: KmmNodeId("item-node".to_string()),
            kind: KmmNodeKind::Text(TextSpan {
                text: text.to_string(),
            }),
            source: source_span(text),
            children: Vec::new(),
        }],
        children: Vec::new(),
        source: source_span(text),
    }
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
