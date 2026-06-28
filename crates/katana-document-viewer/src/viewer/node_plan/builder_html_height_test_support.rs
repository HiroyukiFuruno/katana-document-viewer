use crate::{
    DocumentSnapshotFactory, DocumentSource, KdvThemeSnapshot, SourceKind, SourceRevision,
    SourceUri, ViewerInput, ViewerInteractionConfig, ViewerMode, ViewerSearchState, ViewerViewport,
};
use katana_markdown_model::{
    ByteRange, KmmDocument, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange,
    RawSnippet, SourceSpan, TableCell, TableRow, TextFingerprint,
};
use std::path::PathBuf;

pub(super) fn input_with_nodes(nodes: Vec<KmmNode>) -> ViewerInput {
    input_with_font_size(nodes, 24)
}

pub(super) fn input_with_font_size(nodes: Vec<KmmNode>, preview_font_size: u16) -> ViewerInput {
    let document = KmmDocument {
        path: PathBuf::from("builder-html-height.md"),
        fingerprint: TextFingerprint {
            algorithm: "test".to_string(),
            value: "builder-html-height-revision".to_string(),
        },
        nodes,
    };
    let source = DocumentSource {
        uri: SourceUri("preview://builder-html-height.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("builder-html-height-revision".to_string()),
        content: String::new(),
    };
    ViewerInput {
        snapshot: DocumentSnapshotFactory::from_kmm(source, document),
        artifacts: Vec::new(),
        theme: KdvThemeSnapshot::default(),
        mode: ViewerMode::Document,
        interaction: ViewerInteractionConfig::default(),
        typography: crate::ViewerTypographyConfig { preview_font_size },
        viewport: ViewerViewport {
            width: 640.0,
            height: 320.0,
        },
        search: ViewerSearchState::default(),
    }
}

pub(super) fn table_row(cells: &[&str]) -> TableRow {
    TableRow {
        cells: cells
            .iter()
            .map(|cell| TableCell {
                text: (*cell).to_string(),
                source: source(cell),
            })
            .collect(),
    }
}

pub(super) fn node(kind: KmmNodeKind, raw: &str, children: Vec<KmmNode>) -> KmmNode {
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
