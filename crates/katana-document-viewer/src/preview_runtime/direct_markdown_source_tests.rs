use super::{MarkdownSource, PreviewConfig, PreviewError, PreviewOutputFactory};
use crate::{DocumentKind, ViewerNodeKind, ViewerNodePlanner};

const CONTENT_HEIGHT: f32 = 120.0;

#[test]
fn direct_md_source_stays_markdown_and_builds_viewer_nodes() -> Result<(), PreviewError> {
    assert_markdown_document("fixture.md")
}

#[test]
fn direct_markdown_source_stays_markdown_and_builds_viewer_nodes() -> Result<(), PreviewError> {
    assert_markdown_document("fixture.markdown")
}

#[test]
fn direct_txt_source_stays_markdown_and_builds_viewer_nodes() -> Result<(), PreviewError> {
    assert_markdown_document("fixture.txt")
}

fn assert_markdown_document(document_id: &str) -> Result<(), PreviewError> {
    let output = PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: "# Direct Markdown\n\nBody text".to_string(),
            document_id: Some(document_id.to_string()),
        },
        &PreviewConfig::default(),
        CONTENT_HEIGHT,
    )?;
    let plan = ViewerNodePlanner::create(&output.input, 0.0);

    assert_eq!(DocumentKind::Markdown, output.input.snapshot.kind);
    assert!(matches!(plan.nodes[0].kind, ViewerNodeKind::Heading { .. }));
    assert!(matches!(plan.nodes[1].kind, ViewerNodeKind::Paragraph));
    Ok(())
}
