use crate::document::{DocumentId, SourceRevision};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtifactId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtifactUri(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    Svg,
    OfficePlaceholder,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactBytes {
    pub bytes: Vec<u8>,
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
}

pub struct ArtifactFactory;

impl ArtifactFactory {
    pub fn preview(
        document_id: DocumentId,
        source_revision: SourceRevision,
        bytes: ArtifactBytes,
    ) -> Artifact {
        Self::artifact(
            ArtifactKind::Preview,
            ArtifactFormat::RenderTree,
            document_id,
            source_revision,
            bytes,
            "katana-document-viewer",
            ArtifactDiagnostics {
                entries: Vec::new(),
            },
        )
    }

    pub fn export(
        format: ArtifactFormat,
        document_id: DocumentId,
        source_revision: SourceRevision,
        bytes: ArtifactBytes,
    ) -> Artifact {
        Self::export_with_backend(
            format,
            document_id,
            source_revision,
            bytes,
            "katana-document-viewer",
            ArtifactDiagnostics {
                entries: Vec::new(),
            },
        )
    }

    pub fn export_with_backend(
        format: ArtifactFormat,
        document_id: DocumentId,
        source_revision: SourceRevision,
        bytes: ArtifactBytes,
        backend: &str,
        diagnostics: ArtifactDiagnostics,
    ) -> Artifact {
        Self::artifact(
            ArtifactKind::Export,
            format,
            document_id,
            source_revision,
            bytes,
            backend,
            diagnostics,
        )
    }

    pub fn image_with_backend(
        format: ArtifactFormat,
        document_id: DocumentId,
        source_revision: SourceRevision,
        bytes: ArtifactBytes,
        backend: &str,
        diagnostics: ArtifactDiagnostics,
    ) -> Artifact {
        Self::artifact(
            ArtifactKind::Image,
            format,
            document_id,
            source_revision,
            bytes,
            backend,
            diagnostics,
        )
    }

    fn artifact(
        kind: ArtifactKind,
        format: ArtifactFormat,
        document_id: DocumentId,
        source_revision: SourceRevision,
        bytes: ArtifactBytes,
        backend: &str,
        diagnostics: ArtifactDiagnostics,
    ) -> Artifact {
        let id = ArtifactId(format!("{}:{:?}", document_id.0, format));
        Artifact {
            uri: ArtifactUri(format!("kdv://artifact/{}", id.0)),
            manifest: ArtifactManifest {
                id,
                kind,
                format,
                document_id,
                source_revision,
                backend: backend.to_string(),
                diagnostics,
            },
            bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_manifest_serializes() -> Result<(), toml::ser::Error> {
        let artifact = ArtifactFactory::export(
            ArtifactFormat::Html,
            DocumentId("doc".to_string()),
            SourceRevision("rev".to_string()),
            ArtifactBytes {
                bytes: b"<h1>Title</h1>".to_vec(),
            },
        );

        let serialized = toml::to_string(&artifact.manifest)?;

        assert!(serialized.contains("format = \"Html\""));
        assert!(serialized.contains("source_revision = \"rev\""));
        Ok(())
    }
}
