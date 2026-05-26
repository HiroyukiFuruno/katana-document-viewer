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
        let parse_result = KatanaMarkdownModel::parse(MarkdownInput::from_content(
            "sample.md",
            source.content.clone(),
        ));
        DocumentSnapshotFactory::from_parse_result(source, parse_result)
    }
}

#[cfg(test)]
mod tests {
    use super::SampleSnapshotFactory;

    #[test]
    fn creates_parseable_sample_snapshot() {
        let result = SampleSnapshotFactory::create()
            .map(|snapshot| (snapshot.source_uri.0, !snapshot.document.nodes.is_empty()));
        assert!(result.is_ok());

        let actual = result.unwrap_or((String::new(), false));
        assert_eq!(actual, ("file:///sample.md".to_string(), true));
    }
}
