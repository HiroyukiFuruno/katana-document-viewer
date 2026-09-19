use katana_document_viewer::{
    BinaryDocumentSource, DocumentSession, DocumentSessionCommand, DocumentSessionConfig,
    DocumentSessionError, DocumentSurfaceCommand, DocumentViewport, OfficeDocumentFormat,
    OfficeDocumentSource, OfficePackagePreflight, OfficePreflightLimits, OfficeWorkerError,
    SpreadsheetFilterCommand, ViewerSource, ViewerSourceIdentity,
};
use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;

#[test]
fn binary_linkage_preserves_closed_document_session_failures()
-> Result<(), Box<dyn std::error::Error>> {
    let mut session = closed_session()?;
    assert_closed_session_error(session.apply(DocumentSessionCommand::Surface(
        DocumentSurfaceCommand::Resize(DocumentViewport::new(1, 1)),
    )));
    assert_closed_session_error(session.frame());
    assert_closed_session_error(session.apply_spreadsheet_filter(
        SpreadsheetFilterCommand::Clear {
            sheet_index: 0,
            column: None,
        },
    ));
    Ok(())
}

fn closed_session() -> Result<DocumentSession, Box<dyn std::error::Error>> {
    let source = ViewerSource::Pdf(BinaryDocumentSource::new(
        ViewerSourceIdentity::new("file:///sample.pdf", "sha256:sample"),
        "application/pdf",
        include_bytes!("../../../../../assets/reference/katana/pdf/sample.pdf").to_vec(),
    ));
    let mut session = DocumentSession::open(
        source,
        DocumentSessionConfig::new(DocumentViewport::new(320, 240)),
    )?;
    session.close_in_place();
    Ok(session)
}

fn assert_closed_session_error<T>(result: Result<T, DocumentSessionError>) {
    assert!(matches!(
        result,
        Err(DocumentSessionError::Office(OfficeWorkerError::Protocol { ref message }))
            if message == "document session is closed"
    ));
}

#[test]
fn binary_linkage_preserves_corrupt_office_entry_rejection()
-> Result<(), Box<dyn std::error::Error>> {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    writer.start_file(
        "word/document.xml",
        SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored),
    )?;
    writer.write_all(b"<w:document/>")?;
    let mut bytes = writer.finish()?.into_inner();
    bytes[47] ^= 0xff;
    let source = OfficeDocumentSource::new(
        ViewerSourceIdentity::new("file:///corrupt.docx", "sha256:corrupt"),
        OfficeDocumentFormat::Docx,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        bytes,
    );
    assert!(OfficePackagePreflight::inspect(&source, OfficePreflightLimits::strict()).is_err());
    Ok(())
}
