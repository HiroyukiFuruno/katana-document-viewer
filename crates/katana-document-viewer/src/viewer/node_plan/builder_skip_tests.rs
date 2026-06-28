use super::{PlannedNode, ViewerNodeKind, ViewerNodePlanBuilder};
use crate::artifact::{ArtifactFormat, ArtifactId, ArtifactUri};
use crate::viewer::asset::ViewerAssetReference;
use crate::{
    DocumentSnapshotFactory, DocumentSource, KdvThemeSnapshot, SourceKind, SourceRevision,
    SourceUri, ViewerInput, ViewerInteractionConfig, ViewerMode, ViewerSearchState, ViewerViewport,
};
use katana_markdown_model::{
    ByteRange, KmmDocument, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
    TextFingerprint,
};
use std::path::PathBuf;

#[test]
fn skips_empty_and_structural_html_paragraph_nodes() {
    let input = input();
    let builder = ViewerNodePlanBuilder::new(&input, 0.0);

    let empty_node = planned(1, ViewerNodeKind::Paragraph, "", None);
    let structural_node = planned(2, ViewerNodeKind::Paragraph, "<section>", None);
    let declaration_node = planned(3, ViewerNodeKind::Paragraph, "<!doctype html>", None);

    assert!(builder.should_skip_planned_node(&empty_node));
    assert!(builder.should_skip_planned_node(&structural_node));
    assert!(builder.should_skip_planned_node(&declaration_node));
}

#[test]
fn does_not_skip_empty_html_tag_name() {
    let input = input();
    let builder = ViewerNodePlanBuilder::new(&input, 0.0);

    let empty_tag = planned(2, ViewerNodeKind::Paragraph, "<>", None);

    assert!(!builder.should_skip_planned_node(&empty_tag));
}

#[test]
fn keeps_non_paragraph_and_referenced_nodes() {
    let input = input();
    let builder = ViewerNodePlanBuilder::new(&input, 0.0);
    let non_paragraph = planned(3, ViewerNodeKind::Heading { level: 1 }, "<section>", None);
    let with_reference = planned(4, ViewerNodeKind::Paragraph, "text", Some(reference()));

    assert!(!builder.should_skip_planned_node(&non_paragraph));
    assert!(!builder.should_skip_planned_node(&with_reference));
}

fn planned(
    id: usize,
    kind: ViewerNodeKind,
    text: &str,
    reference: Option<ViewerAssetReference>,
) -> PlannedNode {
    PlannedNode {
        node_id: KmmNodeId(format!("node-{id}")),
        kind,
        source: source(text),
        text: text.to_string(),
        spans: Vec::new(),
        reference,
    }
}

fn reference() -> ViewerAssetReference {
    ViewerAssetReference {
        node_id: KmmNodeId("referenced".to_string()),
        artifact_id: ArtifactId("artifact".to_string()),
        uri: ArtifactUri("kdv://test".to_string()),
        format: ArtifactFormat::Svg,
    }
}

fn input() -> ViewerInput {
    let document = KmmDocument {
        path: PathBuf::from("builder-skip.md"),
        fingerprint: TextFingerprint {
            algorithm: "test".to_string(),
            value: "builder-skip-revision".to_string(),
        },
        nodes: Vec::new(),
    };
    let source = DocumentSource {
        uri: SourceUri("preview://builder-skip.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("builder-skip-revision".to_string()),
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

fn source(text: &str) -> SourceSpan {
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
