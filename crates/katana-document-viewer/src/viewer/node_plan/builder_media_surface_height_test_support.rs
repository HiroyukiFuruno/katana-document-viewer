use super::{super::super::planned_node::PlannedNode, super::super::types::ViewerNodeKind};
use crate::forge::BuildGraph;
use katana_markdown_model::{
    ByteRange, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange, RawSnippet, TableCell,
    TableNode, TableRow,
};

pub(super) fn paragraph_kind_node() -> KmmNode {
    KmmNode {
        id: KmmNodeId("node-paragraph".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source("paragraph"),
        children: Vec::new(),
    }
}

pub(super) fn paragraph_with_image() -> KmmNode {
    KmmNode {
        id: KmmNodeId("node-image".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source("![image](img.svg)"),
        children: Vec::new(),
    }
}

pub(super) fn table_node() -> KmmNode {
    KmmNode {
        id: KmmNodeId("node-table".to_string()),
        kind: KmmNodeKind::Table(TableNode {
            alignments: Vec::new(),
            rows: vec![TableRow {
                cells: vec![
                    TableCell {
                        text: "A".to_string(),
                        source: source("A"),
                    },
                    TableCell {
                        text: "B".to_string(),
                        source: source("B"),
                    },
                ],
            }],
        }),
        source: source("A|B"),
        children: Vec::new(),
    }
}

pub(super) fn planned_node(kind: ViewerNodeKind, text: &str, node: &KmmNode) -> PlannedNode {
    PlannedNode {
        node_id: KmmNodeId(format!("planned-{}", node.id.0)),
        kind,
        source: source(""),
        text: text.to_string(),
        spans: Vec::new(),
        reference: None,
    }
}

pub(super) fn fake_graph() -> BuildGraph {
    use crate::document::DocumentSnapshotFactory;
    use crate::document::{SourceKind, SourceRevision as Revision, SourceUri};
    use crate::forge::BuildProfile;
    use crate::forge::BuildRequest;
    use katana_markdown_model::{KmmDocument, TextFingerprint};
    use std::path::PathBuf;
    let source_document = crate::document::DocumentSource {
        uri: SourceUri("preview://builder.md".to_string()),
        kind: SourceKind::Markdown,
        revision: Revision("rev".to_string()),
        content: String::new(),
    };
    let snapshot = DocumentSnapshotFactory::from_kmm(
        source_document,
        KmmDocument {
            path: PathBuf::from("builder.md"),
            fingerprint: TextFingerprint {
                algorithm: "md".to_string(),
                value: "id".to_string(),
            },
            nodes: Vec::new(),
        },
    );
    BuildGraph::from_request(&BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme: crate::theme::KdvThemeSnapshot::default(),
    })
}

fn source(text: &str) -> katana_markdown_model::SourceSpan {
    katana_markdown_model::SourceSpan {
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
