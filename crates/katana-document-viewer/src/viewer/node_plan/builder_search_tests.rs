use super::{ViewerNodeKind, ViewerNodePlanner};
use crate::{
    DocumentSnapshotFactory, DocumentSource, KdvThemeSnapshot, SourceKind, SourceRevision,
    SourceUri, ViewerInput, ViewerInteractionConfig, ViewerMode, ViewerRect, ViewerSearchEngine,
    ViewerSearchMatch, ViewerSearchMatchId, ViewerSearchState, ViewerSearchTarget, ViewerTextRange,
    ViewerViewport,
};
use katana_markdown_model::{
    ByteRange, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange,
    RawSnippet, SourceSpan, TextFingerprint, TextSpan,
};
use std::path::PathBuf;

#[test]
fn planner_marks_matching_search_text_as_highlighted_spans() {
    let mut input = input_with_nodes(vec![paragraph("alpha beta alpha")]);
    input.search = ViewerSearchEngine::state("alpha", Vec::new(), Some(0));

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(ViewerNodeKind::Paragraph, plan.nodes[0].kind);
    assert!(plan.nodes[0].spans.iter().any(|span| span.style.highlight));
    assert_eq!("alpha", plan.nodes[0].spans[0].text);
}

#[test]
fn planner_marks_current_search_text_with_separate_style() {
    let mut input = input_with_nodes(vec![paragraph("alpha beta alpha")]);
    let node_id = input.snapshot.document.nodes[0].id.clone();
    input.search = ViewerSearchEngine::state(
        "alpha",
        vec![search_target(node_id, source("alpha beta alpha"), 11, 16)],
        Some(0),
    );

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert!(plan.nodes[0].spans[0].style.highlight);
    assert!(!plan.nodes[0].spans[0].style.current_highlight);
    assert!(plan.nodes[0].spans[2].style.current_highlight);
}

fn input_with_nodes(nodes: Vec<KmmNode>) -> ViewerInput {
    let document = KmmDocument {
        path: PathBuf::from("builder.md"),
        fingerprint: TextFingerprint {
            algorithm: "test".to_string(),
            value: "builder-search-revision".to_string(),
        },
        nodes,
    };
    let source = DocumentSource {
        uri: SourceUri("preview://builder.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("builder-search-revision".to_string()),
        content: String::new(),
    };
    ViewerInput {
        snapshot: DocumentSnapshotFactory::from_kmm(source, document),
        artifacts: Vec::new(),
        theme: KdvThemeSnapshot::default(),
        mode: ViewerMode::Document,
        interaction: ViewerInteractionConfig::default(),
        typography: crate::ViewerTypographyConfig::default(),
        viewport: ViewerViewport {
            width: 640.0,
            height: 320.0,
        },
        search: ViewerSearchState::default(),
    }
}

fn paragraph(text: &str) -> KmmNode {
    node(KmmNodeKind::Paragraph, text, vec![text_node(text)])
}

fn text_node(text: &str) -> KmmNode {
    node(
        KmmNodeKind::Text(TextSpan {
            text: text.to_string(),
        }),
        text,
        Vec::new(),
    )
}

fn node(kind: KmmNodeKind, raw: &str, children: Vec<KmmNode>) -> KmmNode {
    KmmNode {
        id: KmmNodeId(format!("node-{raw}")),
        kind,
        source: source(raw),
        children,
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

fn search_target(
    node_id: KmmNodeId,
    source: SourceSpan,
    start: usize,
    end: usize,
) -> ViewerSearchTarget {
    ViewerSearchTarget {
        index: 0,
        matched: ViewerSearchMatch {
            id: ViewerSearchMatchId("current".to_string()),
            node_id,
            source,
            range: ViewerTextRange { start, end },
            text: "alpha".to_string(),
            artifact_id: None,
        },
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 20.0,
        },
    }
}
