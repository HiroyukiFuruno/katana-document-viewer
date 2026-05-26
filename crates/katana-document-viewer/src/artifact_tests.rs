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

#[test]
fn artifact_preview_uses_preview_kind_and_render_tree_format() {
    let artifact = ArtifactFactory::preview(
        DocumentId("document-1".to_string()),
        SourceRevision("r1".to_string()),
        ArtifactBytes {
            bytes: b"preview".to_vec(),
        },
    );

    assert_eq!(artifact.manifest.kind, ArtifactKind::Preview);
    assert_eq!(artifact.manifest.format, ArtifactFormat::RenderTree);
    assert_eq!(artifact.manifest.backend, "katana-document-viewer");
    assert_eq!(artifact.manifest.id.0, "document-1:RenderTree");
    assert_eq!(artifact.uri.0, "kdv://artifact/document-1:RenderTree");
}

#[test]
fn artifact_export_with_backend_preserves_metadata_and_diagnostics() {
    let diagnostics = ArtifactDiagnostics {
        entries: vec![ArtifactDiagnostic {
            severity: DiagnosticSeverity::Warning,
            code: "w-1".to_string(),
            message: "warning".to_string(),
        }],
    };
    let artifact = ArtifactFactory::export_with_backend(
        ArtifactFormat::Pdf,
        DocumentId("document-2".to_string()),
        SourceRevision("r2".to_string()),
        ArtifactBytes {
            bytes: b"%PDF-1.4".to_vec(),
        },
        "custom-backend",
        diagnostics.clone(),
    );

    assert_eq!(artifact.manifest.kind, ArtifactKind::Export);
    assert_eq!(artifact.manifest.format, ArtifactFormat::Pdf);
    assert_eq!(artifact.manifest.backend, "custom-backend");
    assert_eq!(artifact.manifest.diagnostics, diagnostics);
}

#[test]
fn artifact_image_with_backend_marks_image_kind() {
    let artifact = ArtifactFactory::image_with_backend(
        ArtifactFormat::Png,
        DocumentId("document-3".to_string()),
        SourceRevision("r3".to_string()),
        ArtifactBytes {
            bytes: vec![0x89, 0x50, 0x4e, 0x47],
        },
        "viewer",
        ArtifactDiagnostics {
            entries: Vec::new(),
        },
    );

    assert_eq!(artifact.manifest.kind, ArtifactKind::Image);
    assert_eq!(artifact.manifest.format, ArtifactFormat::Png);
    assert_eq!(artifact.manifest.backend, "viewer");
}
