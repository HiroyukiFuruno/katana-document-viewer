use super::{ViewerNodeKind, ViewerNodePlanner};
use crate::{
    DocumentSnapshotFactory, DocumentSource, KdvThemeSnapshot, SourceKind, SourceRevision,
    SourceUri, ViewerInput, ViewerInteractionConfig, ViewerMode, ViewerSearchState, ViewerViewport,
};
use katana_markdown_model::{
    ByteRange, FootnoteDefinitionNode, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, RawSnippet, SourceSpan, TextFingerprint, TextSpan,
};
use std::path::PathBuf;

const VIEWPORT_WIDTH: f32 = 640.0;
const VIEWPORT_HEIGHT: f32 = 320.0;

#[test]
fn planner_preserves_footnote_definition_as_footnote_node() {
    let input = input_with_footnote();

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(ViewerNodeKind::Rule, plan.nodes[0].kind);
    assert_eq!(
        ViewerNodeKind::FootnoteDefinition {
            label: "note".to_string(),
        },
        plan.nodes[1].kind
    );
    assert_eq!("note. 注釈本文", plan.nodes[1].text);
}

#[test]
fn planner_moves_footnote_definitions_to_document_end() {
    let input = input_with_nodes(vec![
        paragraph_node("before"),
        footnote_node(),
        paragraph_node("after"),
    ]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!("before", plan.nodes[0].text);
    assert_eq!("after", plan.nodes[1].text);
    assert_eq!(ViewerNodeKind::Rule, plan.nodes[2].kind);
    assert_eq!(
        ViewerNodeKind::FootnoteDefinition {
            label: "note".to_string(),
        },
        plan.nodes[3].kind
    );
}

fn input_with_footnote() -> ViewerInput {
    input_with_nodes(vec![footnote_node()])
}

fn input_with_nodes(nodes: Vec<KmmNode>) -> ViewerInput {
    ViewerInput {
        snapshot: DocumentSnapshotFactory::from_kmm(document_source(), document_with_nodes(nodes)),
        artifacts: Vec::new(),
        theme: KdvThemeSnapshot::default(),
        mode: ViewerMode::Document,
        interaction: ViewerInteractionConfig::default(),
        typography: crate::ViewerTypographyConfig::default(),
        viewport: ViewerViewport {
            width: VIEWPORT_WIDTH,
            height: VIEWPORT_HEIGHT,
        },
        search: ViewerSearchState::default(),
    }
}

fn document_source() -> DocumentSource {
    DocumentSource {
        uri: SourceUri("preview://footnote.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("footnote-revision".to_string()),
        content: String::new(),
    }
}

fn document_with_nodes(nodes: Vec<KmmNode>) -> KmmDocument {
    KmmDocument {
        path: PathBuf::from("footnote.md"),
        fingerprint: TextFingerprint {
            algorithm: "test".to_string(),
            value: "footnote-revision".to_string(),
        },
        nodes,
    }
}

fn paragraph_node(text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(format!("paragraph-{text}")),
        kind: KmmNodeKind::Paragraph,
        source: source(text),
        children: vec![KmmNode {
            id: KmmNodeId(format!("text-{text}")),
            kind: KmmNodeKind::Text(TextSpan {
                text: text.to_string(),
            }),
            source: source(text),
            children: Vec::new(),
        }],
    }
}

fn footnote_node() -> KmmNode {
    let raw = "[^note]: 注釈本文";
    KmmNode {
        id: KmmNodeId("footnote-note".to_string()),
        kind: KmmNodeKind::FootnoteDefinition(FootnoteDefinitionNode {
            label: "note".to_string(),
            text: "注釈本文".to_string(),
        }),
        source: source(raw),
        children: Vec::new(),
    }
}

fn source(raw: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: raw.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: raw.len() + 1,
            },
        },
        raw: RawSnippet {
            text: raw.to_string(),
        },
    }
}
