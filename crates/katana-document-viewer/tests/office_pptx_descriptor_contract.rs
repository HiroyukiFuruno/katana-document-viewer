use katana_document_viewer::{
    OfficeDocumentFormat, OfficeDocumentSource, OfficePackagePreflight, OfficePreflightLimits,
    OfficeStaticViewerSession, OfficeWorkerConfig, PdfPageRenderRequest, ViewerSourceIdentity,
};
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn representative_pptx() -> TestResult<OfficeDocumentSource> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.pptx");
    Ok(OfficeDocumentSource::new(
        ViewerSourceIdentity::new("file:///fixtures/representative.pptx", "representative"),
        OfficeDocumentFormat::Pptx,
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        std::fs::read(path)?,
    ))
}

fn streaming_pptx(compression: zip::CompressionMethod) -> TestResult<OfficeDocumentSource> {
    let source = representative_pptx()?;
    let mut archive = zip::ZipArchive::new(Cursor::new(source.bytes.as_slice()))?;
    let mut writer = zip::ZipWriter::new_stream(Vec::new());
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        let name = entry.name().to_owned();
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes)?;
        writer.start_file(
            name,
            zip::write::SimpleFileOptions::default().compression_method(compression),
        )?;
        writer.write_all(&bytes)?;
    }
    let bytes = writer.finish()?.into_inner();
    let mut checked = zip::ZipArchive::new(Cursor::new(bytes.as_slice()))?;
    assert!(!checked.is_empty());
    for index in 0..checked.len() {
        let entry = checked.by_index(index)?;
        let header_start = usize::try_from(entry.header_start())?;
        let header = bytes
            .get(header_start..header_start + 30)
            .ok_or("truncated local header")?;
        assert_eq!(&header[..4], b"PK\x03\x04");
        assert_ne!(
            0,
            u16::from_le_bytes([header[6], header[7]]) & 0x0008,
            "entry must use a data descriptor"
        );
        assert_eq!(&header[14..26], &[0; 12]);
    }
    Ok(OfficeDocumentSource::new(
        ViewerSourceIdentity::new(
            "file:///fixtures/representative-descriptor.pptx",
            "descriptor",
        ),
        OfficeDocumentFormat::Pptx,
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        bytes,
    ))
}

fn worker_config() -> OfficeWorkerConfig {
    OfficeWorkerConfig::new(PathBuf::from(env!("CARGO_BIN_EXE_kdv-office-worker")))
}

fn render_first_frame(source: OfficeDocumentSource) -> TestResult<Vec<u8>> {
    OfficePackagePreflight::inspect(&source, OfficePreflightLimits::strict())?;
    let mut session = OfficeStaticViewerSession::open(source, worker_config())?;
    Ok(session
        .render_item(PdfPageRenderRequest::new(0, 1.0))?
        .surface
        .rgba)
}

#[test]
fn office2pdf_accepts_legal_stored_descriptor_from_central_directory() -> TestResult {
    let source = streaming_pptx(zip::CompressionMethod::Stored)?;
    let pdf = office2pdf::convert_bytes(
        &source.bytes,
        office2pdf::config::Format::Pptx,
        &office2pdf::config::ConvertOptions::default(),
    )?;
    assert!(pdf.pdf.starts_with(b"%PDF"));
    Ok(())
}

#[test]
fn representative_pptx_baseline_and_deflated_descriptor_generate_frames() -> TestResult {
    let baseline = render_first_frame(representative_pptx()?)?;
    let descriptor = render_first_frame(streaming_pptx(zip::CompressionMethod::Deflated)?)?;
    assert!(!baseline.is_empty());
    assert_eq!(baseline, descriptor);
    Ok(())
}

#[test]
fn stored_descriptor_pptx_generates_a_frame() -> TestResult {
    let frame = render_first_frame(streaming_pptx(zip::CompressionMethod::Stored)?)?;
    assert!(!frame.is_empty());
    Ok(())
}
