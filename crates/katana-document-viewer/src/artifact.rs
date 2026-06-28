use crate::document::{DocumentId, SourceRevision};
use serde::{Deserialize, Serialize};

#[path = "artifact/factory.rs"]
mod factory;
pub use factory::ArtifactFactory;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtifactId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtifactUri(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtifactKind {
    Preview,
    Export,
    Image,
    Pdf,
    OfficePlaceholder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtifactFormat {
    RenderTree,
    Html,
    Pdf,
    Png,
    Jpeg,
    Gif,
    Webp,
    Bmp,
    Svg,
    OfficePlaceholder,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactBytes {
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactTextExtraction {
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactDiagnostic {
    pub severity: DiagnosticSeverity,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactDiagnostics {
    pub entries: Vec<ArtifactDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactManifest {
    pub id: ArtifactId,
    pub kind: ArtifactKind,
    pub format: ArtifactFormat,
    pub document_id: DocumentId,
    pub source_revision: SourceRevision,
    pub backend: String,
    pub diagnostics: ArtifactDiagnostics,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Artifact {
    pub manifest: ArtifactManifest,
    pub uri: ArtifactUri,
    pub bytes: ArtifactBytes,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_extraction: Option<ArtifactTextExtraction>,
}

impl Artifact {
    #[must_use]
    pub fn with_text_extraction(mut self, text: impl Into<String>) -> Self {
        self.text_extraction = Some(ArtifactTextExtraction { text: text.into() });
        self
    }
}

#[cfg(test)]
#[path = "artifact_tests.rs"]
mod tests;
