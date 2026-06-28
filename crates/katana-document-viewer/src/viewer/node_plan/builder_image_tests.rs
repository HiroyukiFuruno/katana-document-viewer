use super::{ViewerNodeKind, ViewerNodePlanner};
use crate::{
    DocumentSnapshotFactory, DocumentSource, KdvThemeSnapshot, SourceKind, SourceRevision,
    SourceUri, ViewerInput, ViewerInteractionConfig, ViewerMode, ViewerSearchState, ViewerViewport,
};
use katana_markdown_model::{
    ByteRange, ImageNode, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn,
    LineColumnRange, RawSnippet, SourceSpan, TextFingerprint, TextSpan,
};
use std::path::PathBuf;

const VIEWPORT_WIDTH: f32 = 640.0;
const VIEWPORT_HEIGHT: f32 = 320.0;

#[test]
fn planner_keeps_image_touching_next_block_as_paragraph() {
    let input = input_with_nodes(vec![
        image_paragraph_at_line(1),
        text_paragraph_at_line("Text", 2),
    ]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(2, plan.nodes.len());
    assert_eq!(ViewerNodeKind::Paragraph, plan.nodes[0].kind);
    assert!(plan.asset_requests.is_empty());
}

#[test]
fn planner_keeps_image_touching_previous_block_as_paragraph() {
    let input = input_with_nodes(vec![
        text_paragraph_at_line("Before", 1),
        image_paragraph_at_line(2),
    ]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(2, plan.nodes.len());
    assert_eq!(ViewerNodeKind::Paragraph, plan.nodes[1].kind);
    assert!(plan.asset_requests.is_empty());
}

#[test]
fn planner_loads_image_separated_by_blank_lines_as_media_node() {
    let input = input_with_nodes(vec![
        text_paragraph_at_line("Before", 1),
        image_paragraph_at_line(3),
        text_paragraph_at_line("After", 5),
    ]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(3, plan.nodes.len());
    assert_eq!(ViewerNodeKind::Image, plan.nodes[1].kind);
    assert_eq!(1, plan.asset_requests.len());
}

#[test]
fn node_plan_image_format_accepts_query_string_image_sources() {
    let input = input_with_nodes(vec![
        text_paragraph_at_line("Before", 1),
        image_paragraph_with_source_at_line("screen.webp?cache=1#frag", 3),
        text_paragraph_at_line("After", 5),
    ]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(ViewerNodeKind::Image, plan.nodes[1].kind);
    assert_eq!(1, plan.asset_requests.len());
    assert_eq!(crate::ArtifactFormat::Webp, plan.asset_requests[0].format);
}

#[test]
fn node_plan_image_request_uri_uses_kmm_image_source_not_raw_markdown() {
    let input = input_with_nodes(vec![
        text_paragraph_at_line("Before", 1),
        image_paragraph_with_raw_and_child_at_line(
            "![screen](assets/screen.png \"Screenshot\")",
            "assets/screen.png",
            Some("Screenshot"),
            3,
        ),
        text_paragraph_at_line("After", 5),
    ]);

    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert_eq!(ViewerNodeKind::Image, plan.nodes[1].kind);
    assert_eq!(1, plan.asset_requests.len());
    assert_eq!("assets/screen.png", plan.asset_requests[0].uri.0);
}

fn input_with_nodes(nodes: Vec<KmmNode>) -> ViewerInput {
    let document = KmmDocument {
        path: PathBuf::from("builder.md"),
        fingerprint: TextFingerprint {
            algorithm: "test".to_string(),
            value: "builder-image-revision".to_string(),
        },
        nodes,
    };
    let source = DocumentSource {
        uri: SourceUri("preview://builder.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("builder-image-revision".to_string()),
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
            width: VIEWPORT_WIDTH,
            height: VIEWPORT_HEIGHT,
        },
        search: ViewerSearchState::default(),
    }
}

fn image_paragraph_at_line(line: usize) -> KmmNode {
    image_paragraph_with_source_at_line("screen.webp", line)
}

fn image_paragraph_with_source_at_line(source: &str, line: usize) -> KmmNode {
    let raw = format!("![screen]({source})");
    image_paragraph_with_raw_and_child_at_line(&raw, source, None, line)
}

fn image_paragraph_with_raw_and_child_at_line(
    raw: &str,
    source: &str,
    title: Option<&str>,
    line: usize,
) -> KmmNode {
    node_at_line(
        KmmNodeKind::Paragraph,
        raw,
        line,
        vec![node_at_line(
            KmmNodeKind::Image(ImageNode {
                alt: "screen".to_string(),
                src: source.to_string(),
                title: title.map(str::to_string),
            }),
            raw,
            line,
            Vec::new(),
        )],
    )
}

fn text_paragraph_at_line(text: &str, line: usize) -> KmmNode {
    node_at_line(
        KmmNodeKind::Paragraph,
        text,
        line,
        vec![node_at_line(
            KmmNodeKind::Text(TextSpan {
                text: text.to_string(),
            }),
            text,
            line,
            Vec::new(),
        )],
    )
}

fn node_at_line(kind: KmmNodeKind, raw: &str, line: usize, children: Vec<KmmNode>) -> KmmNode {
    KmmNode {
        id: KmmNodeId(format!("node-{line}-{raw}")),
        kind,
        source: source_at_line(raw, line),
        children,
    }
}

fn source_at_line(raw: &str, line: usize) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: raw.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line, column: 1 },
            end: LineColumn {
                line,
                column: raw.len() + 1,
            },
        },
        raw: RawSnippet {
            text: raw.to_string(),
        },
    }
}
