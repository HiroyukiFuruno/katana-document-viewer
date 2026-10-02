use super::{
    BinaryDocumentSource, OfficeDocumentFormat, OfficeDocumentSource, OfficeStaticDocumentArtifact,
    OfficeStaticViewerSession, OfficeWorkerConfig, OfficeWorkerError, PdfViewerError,
    PdfViewerSession, ViewerQualityProfile, decode_static_pdf, engine_warning, static_artifact,
    static_profile,
};
use crate::multi_format::office_conversion_key::OfficeConversionKey;
use crate::multi_format::{ViewerSourceIdentity, debug_trace::DebugTrace};
use std::path::PathBuf;

type TestResult = Result<(), Box<dyn std::error::Error>>;
type SessionFixture = (
    ViewerSourceIdentity,
    OfficeConversionKey,
    OfficeStaticViewerSession,
);

#[test]
fn artifact_getter_returns_the_session_artifact() -> TestResult {
    let (identity, conversion_key, session) = test_session()?;

    assert_eq!(&identity, &session.artifact().identity);
    assert_eq!(&conversion_key, session.conversion_key());
    Ok(())
}

#[test]
fn trace_scope_restores_the_stored_office_correlation() -> TestResult {
    let (_, _, mut session) = test_session()?;
    session.trace_session = Some((42, 0x0123_4567_89ab_cdef));

    let _trace_scope = session.trace_scope();

    assert_eq!(
        Some((42, 0x0123_4567_89ab_cdef)),
        DebugTrace::current_session()
    );
    Ok(())
}

fn test_session() -> Result<SessionFixture, Box<dyn std::error::Error>> {
    let identity = ViewerSourceIdentity::new("file:///representative.pdf", "sha256:test");
    let pdf = representative_pdf(&identity)?;
    let artifact = representative_artifact(&identity);
    let source = representative_source(&identity);
    let conversion_key =
        OfficeConversionKey::new(&source, &OfficeWorkerConfig::new(PathBuf::from("worker")));
    Ok((
        identity,
        conversion_key.clone(),
        OfficeStaticViewerSession {
            artifact,
            pdf,
            conversion_key: conversion_key.clone(),
            trace_session: None,
            worker_config: OfficeWorkerConfig::new(PathBuf::from("worker")),
        },
    ))
}

fn representative_pdf(identity: &ViewerSourceIdentity) -> Result<PdfViewerSession, PdfViewerError> {
    PdfViewerSession::open(BinaryDocumentSource::new(
        identity.clone(),
        "application/pdf",
        include_bytes!("../../../../assets/reference/katana/pdf/sample.pdf").to_vec(),
    ))
}

fn representative_artifact(identity: &ViewerSourceIdentity) -> OfficeStaticDocumentArtifact {
    OfficeStaticDocumentArtifact {
        identity: identity.clone(),
        format: OfficeDocumentFormat::Docx,
        mime: "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_owned(),
        item_count: 0,
        items: Vec::new(),
        capabilities: ViewerQualityProfile::static_page().capabilities,
        diagnostics: Vec::new(),
    }
}

fn representative_source(identity: &ViewerSourceIdentity) -> OfficeDocumentSource {
    OfficeDocumentSource::new(
        identity.clone(),
        OfficeDocumentFormat::Docx,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        b"test".to_vec(),
    )
}

#[test]
fn static_profiles_cover_every_office_format() {
    assert_eq!(
        Ok(ViewerQualityProfile::static_page()),
        static_profile(OfficeDocumentFormat::Docx)
    );
    assert_eq!(
        Ok(ViewerQualityProfile::static_slide_with_chart_fallback()),
        static_profile(OfficeDocumentFormat::Pptx)
    );
    assert_eq!(
        Err(OfficeWorkerError::UnsupportedFormat(
            OfficeDocumentFormat::Xlsx
        )),
        static_profile(OfficeDocumentFormat::Xlsx)
    );
}

#[test]
fn source_metadata_and_diagnostics_survive_items_attached_after_pdf_open() -> TestResult {
    let identity = ViewerSourceIdentity::new("file:///representative.docx", "sha256:test");
    let source = representative_source(&identity);
    let mime = source.mime.clone();
    let profile = static_profile(source.format)?;
    let capabilities = profile.capabilities.clone();
    let diagnostics = vec![engine_warning("retained warning".to_owned())];
    let artifact = static_artifact(source, profile, diagnostics.clone(), Vec::new());
    assert_eq!(0, artifact.item_count);
    assert!(artifact.items.is_empty());

    let bytes = include_bytes!("../../../../assets/reference/katana/pdf/sample.pdf").to_vec();
    let (artifact, pdf) = decode_static_pdf(artifact, bytes)?;

    assert_eq!(identity, artifact.identity);
    assert_eq!(mime, artifact.mime);
    assert_eq!(OfficeDocumentFormat::Docx, artifact.format);
    assert_eq!(capabilities, artifact.capabilities);
    assert_eq!(diagnostics, artifact.diagnostics);
    assert_eq!(pdf.artifact().page_count, artifact.item_count);
    assert!(!artifact.items.is_empty());
    Ok(())
}

#[test]
fn failed_pdf_decode_keeps_the_typed_error() {
    let identity = ViewerSourceIdentity::new("fixture:invalid.pdf", "immutable");
    let artifact = representative_artifact(&identity);
    assert!(decode_static_pdf(artifact, b"not a PDF".to_vec()).is_err());
}
