use crate::document::{
    DocumentModelError, DocumentSnapshot, DocumentSnapshotFactory, DocumentSource, SourceKind,
    SourceRevision, SourceUri,
};
use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};

pub(crate) struct SampleSnapshotFactory;

impl SampleSnapshotFactory {
    pub(crate) fn create() -> Result<DocumentSnapshot, DocumentModelError> {
        let source = DocumentSource {
            uri: SourceUri("file:///sample.md".to_string()),
            kind: SourceKind::Markdown,
            revision: SourceRevision("rev-1".to_string()),
            content: "# Title\n\nBody".to_string(),
        };
        let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
            "sample.md",
            source.content.clone(),
        ))?;
        Ok(DocumentSnapshotFactory::from_kmm(source, document))
    }
}
