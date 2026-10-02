use super::{
    EXIT_FAILURE, EXIT_USAGE, PDF_RASTER_MODE, PdfRasterResponse, PdfRasterWorkerEntrypoint,
    WorkerArguments, parse_arguments, parse_positive_u64, read_pdf_source, write_response,
};
use crate::multi_format::office_worker_input::INPUT_NAME;
use crate::multi_format::office_worker_protocol::{MAX_RESPONSE_BYTES, RESPONSE_NAME};
use std::ffi::OsString;

#[path = "pdf_raster_worker_entrypoint_request_tests.rs"]
mod request_tests;

fn arguments(workspace: &std::path::Path) -> Vec<OsString> {
    [
        OsString::from("worker"),
        workspace.as_os_str().to_owned(),
        OsString::from(PDF_RASTER_MODE),
        OsString::from("1048576"),
        OsString::from("2"),
        OsString::from("4194304"),
    ]
    .into()
}

#[test]
fn arguments_accept_the_six_value_worker_contract() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = tempfile::tempdir()?;
    assert_eq!(
        WorkerArguments {
            workspace: workspace.path().to_path_buf(),
            max_memory_bytes: 1_048_576,
            max_cpu_seconds: 2,
            max_output_bytes: 4_194_304,
        },
        parse_arguments(arguments(workspace.path())).map_err(std::io::Error::other)?
    );
    Ok(())
}

#[test]
fn arguments_reject_bad_mode_arity_path_and_nonpositive_limits()
-> Result<(), Box<dyn std::error::Error>> {
    let workspace = tempfile::tempdir()?;
    let mut wrong_mode = arguments(workspace.path());
    wrong_mode[2] = OsString::from("office");
    assert!(parse_arguments(wrong_mode).is_err());
    let mut extra = arguments(workspace.path());
    extra.push(OsString::from("extra"));
    assert!(parse_arguments(extra).is_err());
    let mut relative = arguments(workspace.path());
    relative[1] = OsString::from("relative");
    assert!(parse_arguments(relative).is_err());
    let mut zero = arguments(workspace.path());
    zero[3] = OsString::from("0");
    assert!(parse_arguments(zero).is_err());
    Ok(())
}

#[test]
fn malformed_entrypoint_arguments_return_usage_exit_code() {
    assert_eq!(
        EXIT_USAGE,
        PdfRasterWorkerEntrypoint::run(vec![OsString::from("worker")])
    );
}

#[test]
fn arguments_reject_each_missing_resource_limit() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = tempfile::tempdir()?;
    for (length, name) in [
        (3, "max memory"),
        (4, "max CPU seconds"),
        (5, "max output bytes"),
    ] {
        let mut missing = arguments(workspace.path());
        missing.truncate(length);
        let error = parse_arguments(missing)
            .err()
            .ok_or("missing limit was accepted")?;
        assert!(error.contains(name));
    }
    Ok(())
}

#[test]
fn positive_u64_rejects_malformed_and_zero_values() {
    assert_eq!(
        Ok(1),
        parse_positive_u64(Some(OsString::from("1")), "limit")
    );
    assert!(parse_positive_u64(Some(OsString::from("0")), "limit").is_err());
    assert!(parse_positive_u64(Some(OsString::from("-1")), "limit").is_err());
    assert!(parse_positive_u64(None, "limit").is_err());
}

#[cfg(unix)]
#[test]
fn positive_u64_rejects_non_utf8_os_argument() {
    use std::os::unix::ffi::OsStringExt;
    assert!(parse_positive_u64(Some(OsString::from_vec(vec![0xff])), "limit").is_err());
}

#[test]
fn pdf_source_reads_regular_file_and_preserves_fixed_pdf_identity()
-> Result<(), Box<dyn std::error::Error>> {
    let workspace = tempfile::tempdir()?;
    let bytes = b"bounded pdf bytes";
    std::fs::write(workspace.path().join(INPUT_NAME), bytes)?;
    let source = read_pdf_source(workspace.path(), bytes.len() as u64).map_err(failure_as_io)?;
    assert_eq!("application/pdf", source.mime);
    assert_eq!(bytes.as_slice(), source.bytes.as_slice());
    assert_eq!("kdv:private-pdf-raster/input.office", source.identity.uri);
    Ok(())
}

#[test]
fn pdf_source_rejects_oversized_missing_and_nonregular_input()
-> Result<(), Box<dyn std::error::Error>> {
    let workspace = tempfile::tempdir()?;
    let path = workspace.path().join(INPUT_NAME);
    std::fs::write(&path, b"12345")?;
    assert!(matches!(
        read_pdf_source(workspace.path(), 4),
        Err((stage, _)) if stage == "input_limit"
    ));
    std::fs::remove_file(&path)?;
    assert!(matches!(
        read_pdf_source(workspace.path(), 8),
        Err((stage, _)) if stage == "input"
    ));
    std::fs::create_dir(&path)?;
    assert!(matches!(
        read_pdf_source(workspace.path(), 8),
        Err((stage, _)) if stage == "input"
    ));
    Ok(())
}

#[test]
fn response_writer_serializes_normal_response_and_rejects_oversize_message()
-> Result<(), Box<dyn std::error::Error>> {
    let workspace = tempfile::tempdir()?;
    let response = PdfRasterResponse::Completed {
        page_index: 3,
        scale_bits: 1.25_f32.to_bits(),
        width: 2,
        height: 4,
        rgba_bytes: 32,
    };
    assert_eq!(0, write_response(workspace.path(), &response, 0));
    let bytes = std::fs::read(workspace.path().join(RESPONSE_NAME))?;
    assert!(bytes.len() as u64 <= MAX_RESPONSE_BYTES);
    assert_eq!(response, serde_json::from_slice(&bytes)?);
    let oversized_workspace = tempfile::tempdir()?;
    let failed = PdfRasterResponse::Failed {
        stage: "render".to_owned(),
        message: "x".repeat(MAX_RESPONSE_BYTES as usize),
    };
    assert_eq!(
        EXIT_FAILURE,
        write_response(oversized_workspace.path(), &failed, EXIT_FAILURE)
    );
    assert!(!oversized_workspace.path().join(RESPONSE_NAME).exists());
    Ok(())
}

#[test]
fn response_writer_reports_directory_write_failure() -> Result<(), Box<dyn std::error::Error>> {
    let file = tempfile::NamedTempFile::new()?;
    let response = PdfRasterResponse::Failed {
        stage: "input".to_owned(),
        message: "missing".to_owned(),
    };
    assert_eq!(
        EXIT_FAILURE,
        write_response(file.path(), &response, EXIT_FAILURE)
    );
    Ok(())
}

pub(super) fn failure_as_io(failure: (String, String)) -> std::io::Error {
    std::io::Error::other(format!("{}: {}", failure.0, failure.1))
}

#[test]
fn bounded_file_reader_maps_limit_and_regular_file_errors_to_stages()
-> Result<(), Box<dyn std::error::Error>> {
    let workspace = tempfile::tempdir()?;
    let path = workspace.path().join("input");
    std::fs::write(&path, [1_u8, 2, 3, 4])?;
    let oversized = super::read_regular_file(&path, 3, "input");
    assert!(matches!(
        oversized,
        Err((stage, message)) if stage == "input_limit"
            && message.contains("exceeds 3 bytes")
    ));
    Ok(())
}
