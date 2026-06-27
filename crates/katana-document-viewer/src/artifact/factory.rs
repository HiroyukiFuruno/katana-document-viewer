use super::{
    Artifact, ArtifactBytes, ArtifactDiagnostics, ArtifactFormat, ArtifactId, ArtifactKind,
    ArtifactManifest, ArtifactUri,
};
use crate::document::{DocumentId, SourceRevision};

pub struct ArtifactFactory;

struct ArtifactCreateRequest<'a> {
    id: ArtifactId,
    kind: ArtifactKind,
    format: ArtifactFormat,
    document_id: DocumentId,
    source_revision: SourceRevision,
    bytes: ArtifactBytes,
    backend: &'a str,
    diagnostics: ArtifactDiagnostics,
}

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

    pub fn image_asset_with_id(
        id: ArtifactId,
        format: ArtifactFormat,
        document_id: DocumentId,
        source_revision: SourceRevision,
        bytes: ArtifactBytes,
        backend: &str,
        diagnostics: ArtifactDiagnostics,
    ) -> Artifact {
        Self::artifact_with_id(ArtifactCreateRequest {
            id,
            kind: ArtifactKind::Image,
            format,
            document_id,
            source_revision,
            bytes,
            backend,
            diagnostics,
        })
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
        Self::artifact_with_id(ArtifactCreateRequest {
            id,
            kind,
            format,
            document_id,
            source_revision,
            bytes,
            backend,
            diagnostics,
        })
    }

    fn artifact_with_id(request: ArtifactCreateRequest<'_>) -> Artifact {
        Artifact {
            uri: ArtifactUri(format!("kdv://artifact/{}", request.id.0)),
            manifest: ArtifactManifest {
                id: request.id,
                kind: request.kind,
                format: request.format,
                document_id: request.document_id,
                source_revision: request.source_revision,
                backend: request.backend.to_string(),
                diagnostics: request.diagnostics,
            },
            bytes: request.bytes,
            text_extraction: None,
        }
    }
}
