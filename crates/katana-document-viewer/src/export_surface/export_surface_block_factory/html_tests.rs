use super::*;
use crate::document::DocumentId;
use crate::theme::KdvThemeSnapshot;
use crate::{
    BuildGraph, BuildProfile, BuildRequest, DocumentMetadataView, DocumentOutline,
    DocumentSnapshot, SourceRevision, SourceUri,
};
use katana_markdown_model::{
    ByteRange, HtmlBlockRole, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, RawSnippet, SourceSpan, TextFingerprint,
};
use std::path::PathBuf;

const EMPTY_ID: &str = "html-node";

#[test]
fn generic_html_is_treated_as_wrapped_text() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<div>hello</div>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Generic,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert!(!blocks[0].text_for_tests().is_empty());
}

#[test]
fn centered_html_is_rendered_into_centered_line() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<center>Center</center>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Centered,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].text_for_tests(), "Center");
}

#[test]
fn long_centered_html_falls_back_to_wrapped_centered_lines() {
    let graph = graph();
    let mut blocks = Vec::new();
    let long_text = "a".repeat(BODY_MAX_CHARS + 1);
    let source = format!("<center>{long_text}</center>");
    let node = node(&source);

    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Centered,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].text_for_tests(), "a".repeat(BODY_MAX_CHARS));
    assert_eq!(blocks[1].text_for_tests(), "a");
}

#[test]
fn badge_row_html_creates_badge_block() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<div class=\"badge-row\">A</div>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::BadgeRow,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 1);
    assert!(!blocks[0].text_for_tests().is_empty());
}

#[test]
fn details_html_is_split_into_summary_and_body_lines() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<details><summary>Summary</summary><div>Body</div></details>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Generic,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].text_for_tests(), "Summary");
    assert_eq!(blocks[1].text_for_tests(), "Body");
}

#[test]
fn details_html_falls_back_to_raw_body_when_markdown_nodes_are_empty() {
    let graph = graph();
    let mut blocks = Vec::new();
    let node = node("<details><summary>Summary</summary> </details>");
    SurfaceBlockFactory::append_html(
        &mut blocks,
        &graph,
        &node,
        &HtmlBlockRole::Generic,
        0,
        0,
        &KdvThemeSnapshot::katana_light(),
    );

    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].text_for_tests(), "Summary");
    assert_eq!(blocks[1].text_for_tests(), "");
}

fn graph() -> BuildGraph {
    let document = KmmDocument {
        path: PathBuf::from("/tmp/html.md"),
        fingerprint: TextFingerprint {
            algorithm: "alg".to_string(),
            value: "value".to_string(),
        },
        nodes: Vec::new(),
    };
    let snapshot = DocumentSnapshot {
        id: DocumentId("html".to_string()),
        kind: crate::DocumentKind::Markdown,
        source_uri: SourceUri("file:///html.md".to_string()),
        revision: SourceRevision("r1".to_string()),
        source_path: PathBuf::from("/tmp/html.md"),
        document,
        outline: DocumentOutline { items: Vec::new() },
        metadata: DocumentMetadataView {
            unresolved_count: 0,
            diagnostic_keys: Vec::new(),
        },
    };
    let request = BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    };
    BuildGraph::from_request(&request)
}

fn node(source_text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind: KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic),
        source: source_span(source_text),
        children: Vec::new(),
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
