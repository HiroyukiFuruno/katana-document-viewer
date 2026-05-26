use super::*;
use katana_markdown_model::{
    ByteRange, HeadingNode, LineColumn, LineColumnRange, MarkdownInput, RawSnippet, TextFingerprint,
};

#[test]
fn snapshot_keeps_kmm_outline_and_revision() {
    let source = DocumentSource {
        uri: SourceUri("file:///sample.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("rev-1".to_string()),
        content: "# Title".to_string(),
    };
    let snapshot = DocumentSnapshotFactory::from_kmm(source, sample_document());

    assert_eq!(snapshot.revision, SourceRevision("rev-1".to_string()));
    assert_eq!(snapshot.outline.items.len(), 1);
    assert_eq!(snapshot.outline.items[0].text, "Title");
}

fn sample_document() -> KmmDocument {
    KmmDocument {
        path: PathBuf::from("sample.md"),
        fingerprint: TextFingerprint {
            algorithm: "test".to_string(),
            value: "doc-1".to_string(),
        },
        nodes: vec![KmmNode {
            id: KmmNodeId("node-1".to_string()),
            kind: KmmNodeKind::Heading(HeadingNode {
                level: 1,
                text: "Title".to_string(),
            }),
            source: sample_span("# Title"),
            children: Vec::new(),
        }],
    }
}

fn sample_span(text: &str) -> SourceSpan {
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

#[test]
fn from_parse_result_returns_snapshot_for_success() -> Result<(), Box<dyn std::error::Error>> {
    let source = DocumentSource {
        uri: SourceUri("file:///success.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("rev".to_string()),
        content: "# Success".to_string(),
    };
    let parsed = katana_markdown_model::KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "success.md",
        source.content.clone(),
    ))?;
    let snapshot = DocumentSnapshotFactory::from_parse_result(source, Ok(parsed))?;

    assert_eq!(snapshot.revision, SourceRevision("rev".to_string()));
    assert_eq!(snapshot.kind, DocumentKind::Markdown);
    assert_eq!(
        snapshot.source_uri,
        SourceUri("file:///success.md".to_string())
    );
    Ok(())
}

#[test]
fn from_parse_result_propagates_parse_error() {
    let source = DocumentSource {
        uri: SourceUri("file:///error.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("rev".to_string()),
        content: String::new(),
    };

    let snapshot = DocumentSnapshotFactory::from_parse_result(source, Err(KmmError::EmptySource));

    assert!(matches!(
        snapshot,
        Err(DocumentModelError::KmmParse(KmmError::EmptySource))
    ));
}
