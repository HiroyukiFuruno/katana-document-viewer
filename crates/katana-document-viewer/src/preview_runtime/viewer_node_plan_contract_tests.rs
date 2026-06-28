use crate::{
    DocumentSnapshotFactory, DocumentSource, SourceKind, SourceRevision, SourceUri, ViewerMode,
    ViewerNodeKind, ViewerNodePlanner, ViewerSearchState,
};
use crate::{PreviewConfig, PreviewOutputFactory};
use katana_markdown_model::{
    ByteRange, HeadingNode, InlineSpan, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, RawSnippet, SourceSpan, TextFingerprint, TextSpan,
};
use std::path::PathBuf;

#[test]
fn viewer_plan_contract_covers_commonmark_block_and_inline_edge_nodes() {
    let plan = ViewerNodePlanner::create(
        &PreviewOutputFactory::from_input(viewer_input(), 120.0).input,
        0.0,
    );

    assert_node_text_in_plan(&plan, "quoted", ViewerNodeKind::BlockQuote);
    assert_node_kind_in_plan(&plan, ViewerNodeKind::Rule);
    assert_node_style_text_in_plan(&plan, "gone", true);
    assert_node_contains_text_in_plan(&plan, "#######title");
}

fn assert_node_text_in_plan(plan: &crate::ViewerNodePlan, expected: &str, kind: ViewerNodeKind) {
    assert!(
        plan.nodes.iter().any(|node| {
            node.kind == kind && node.spans.iter().any(|span| span.text == expected)
        })
    );
}

fn assert_node_kind_in_plan(plan: &crate::ViewerNodePlan, kind: ViewerNodeKind) {
    assert!(plan.nodes.iter().any(|node| node.kind == kind));
}

fn assert_node_style_text_in_plan(
    plan: &crate::ViewerNodePlan,
    text: &str,
    style_strikethrough: bool,
) {
    assert!(plan.nodes.iter().any(|node| {
        node.spans
            .iter()
            .any(|span| span.style.strikethrough == style_strikethrough && span.text == text)
    }));
}

fn assert_node_contains_text_in_plan(plan: &crate::ViewerNodePlan, expected: &str) {
    assert!(
        plan.nodes
            .iter()
            .any(|node| { node.spans.iter().any(|span| span.text == expected) })
    );
}

fn viewer_input() -> crate::ViewerInput {
    let config = PreviewConfig::default();
    crate::ViewerInput {
        snapshot: DocumentSnapshotFactory::from_kmm(document_source(), document()),
        artifacts: Vec::new(),
        theme: crate::KdvThemeSnapshot::katana_light(),
        mode: ViewerMode::Document,
        interaction: config.interaction,
        typography: crate::ViewerTypographyConfig::default(),
        viewport: config.viewport,
        search: ViewerSearchState::default(),
    }
}

fn document_source() -> DocumentSource {
    DocumentSource {
        uri: SourceUri("preview://viewer-node-plan-contract.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("contract".to_string()),
        content: "#######title\n\n> quoted\n\n---\n\n~~gone~~".to_string(),
    }
}

fn document() -> KmmDocument {
    KmmDocument {
        path: PathBuf::from("viewer-node-plan-contract.md"),
        fingerprint: TextFingerprint {
            algorithm: "test".to_string(),
            value: "viewer-node-plan-contract".to_string(),
        },
        nodes: contract_nodes(),
    }
}

fn contract_nodes() -> Vec<KmmNode> {
    vec![
        heading_node(),
        quoted_block_node(),
        rule_node(),
        strike_node(),
    ]
}

fn heading_node() -> KmmNode {
    node(
        KmmNodeKind::Heading(HeadingNode {
            level: 2,
            text: "#######title".to_string(),
        }),
        "#######title",
        Vec::new(),
    )
}

fn quoted_block_node() -> KmmNode {
    node(
        KmmNodeKind::BlockQuote,
        "> quoted",
        vec![node(
            KmmNodeKind::Paragraph,
            "quoted",
            vec![text_node("quoted")],
        )],
    )
}

fn rule_node() -> KmmNode {
    node(KmmNodeKind::ThematicBreak, "---", Vec::new())
}

fn strike_node() -> KmmNode {
    node(
        KmmNodeKind::Paragraph,
        "~~gone~~",
        vec![node(
            KmmNodeKind::Strikethrough(InlineSpan {
                text: "~~gone~~".to_string(),
            }),
            "~~gone~~",
            vec![text_node("gone")],
        )],
    )
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
