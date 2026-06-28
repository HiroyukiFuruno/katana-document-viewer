use katana_markdown_model::{KmmDocument, KmmError, KmmNode, KmmNodeId, KmmNodeKind, SourceSpan};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceUri(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceRevision(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceKind {
    Markdown,
    Html,
    Diagram,
    Image,
    Pdf,
    Binary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentKind {
    Markdown,
    Html,
    Diagram,
    Image,
    Pdf,
    Office,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentSource {
    pub uri: SourceUri,
    pub kind: SourceKind,
    pub revision: SourceRevision,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentMetadataView {
    pub unresolved_count: usize,
    pub diagnostic_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentOutline {
    pub items: Vec<DocumentOutlineItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentOutlineItem {
    pub node_id: KmmNodeId,
    pub level: u8,
    pub text: String,
    pub source: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentSnapshot {
    pub id: DocumentId,
    pub kind: DocumentKind,
    pub source_uri: SourceUri,
    pub revision: SourceRevision,
    pub source_path: PathBuf,
    pub document: KmmDocument,
    pub outline: DocumentOutline,
    pub metadata: DocumentMetadataView,
}

#[derive(Debug, Error)]
pub enum DocumentModelError {
    #[error("KMM parse failed: {0}")]
    KmmParse(#[from] KmmError),
}

pub struct DocumentSnapshotFactory;

impl DocumentSnapshotFactory {
    pub fn from_kmm(source: DocumentSource, document: KmmDocument) -> DocumentSnapshot {
        Self::from_kmm_with_kind(source, document, DocumentKind::Markdown)
    }

    pub fn from_kmm_with_kind(
        source: DocumentSource,
        document: KmmDocument,
        kind: DocumentKind,
    ) -> DocumentSnapshot {
        let outline = DocumentOutlineBuilder::build(&document);
        DocumentSnapshot {
            id: DocumentId(document.fingerprint.value.clone()),
            kind,
            source_uri: source.uri,
            revision: source.revision,
            source_path: document.path.clone(),
            document,
            outline,
            metadata: DocumentMetadataView {
                unresolved_count: 0,
                diagnostic_keys: Vec::new(),
            },
        }
    }

    pub fn from_parse_result(
        source: DocumentSource,
        result: Result<KmmDocument, KmmError>,
    ) -> Result<DocumentSnapshot, DocumentModelError> {
        Ok(Self::from_kmm(source, result?))
    }
}

struct DocumentOutlineBuilder;

impl DocumentOutlineBuilder {
    fn build(document: &KmmDocument) -> DocumentOutline {
        let mut items = Vec::new();
        for node in &document.nodes {
            Self::collect_node(node, &mut items);
        }
        DocumentOutline { items }
    }

    fn collect_node(node: &KmmNode, items: &mut Vec<DocumentOutlineItem>) {
        if let KmmNodeKind::Heading(heading) = &node.kind {
            items.push(DocumentOutlineItem {
                node_id: node.id.clone(),
                level: heading.level,
                text: heading.text.clone(),
                source: node.source.clone(),
            });
        }
        for child in &node.children {
            Self::collect_node(child, items);
        }
    }
}

#[cfg(test)]
#[path = "document_tests.rs"]
mod tests;
