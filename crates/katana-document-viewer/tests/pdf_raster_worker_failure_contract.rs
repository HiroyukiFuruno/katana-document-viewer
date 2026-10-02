#![cfg(unix)]

#[path = "support/pdf_raster_failure.rs"]
mod support;

use support::{TestResult, assert_cpu_limit_failure, assert_exit, assert_failure, pdf, workspace};

const OUTPUT_LIMIT: u64 = 128 * 1024 * 1024;

#[test]
fn real_raster_worker_rejects_invalid_limits_with_usage_exit_before_sandbox() -> TestResult {
    let workspace = workspace()?;
    assert_exit(workspace.path(), 0, 64)?;
    assert!(!workspace.path().join("response.json").exists());
    Ok(())
}

#[test]
fn real_raster_worker_preserves_a_rejected_os_resource_limit() -> TestResult {
    let workspace = workspace()?;
    assert_cpu_limit_failure(workspace.path(), OUTPUT_LIMIT)
}

#[test]
fn real_raster_worker_rejects_invalid_pdf_and_render_requests() -> TestResult {
    let invalid_pdf = workspace()?;
    std::fs::write(invalid_pdf.path().join("input.office"), b"not a PDF")?;
    assert_failure(invalid_pdf.path(), OUTPUT_LIMIT, "open")?;
    for request in [
        br#"{"page_index":999999,"scale_bits":1065353216,"max_rgba_bytes":67108864}"#.as_slice(),
        br#"{"page_index":0,"scale_bits":2143289344,"max_rgba_bytes":67108864}"#.as_slice(),
    ] {
        let invalid_request = workspace()?;
        std::fs::write(invalid_request.path().join("request.json"), request)?;
        assert_failure(invalid_request.path(), OUTPUT_LIMIT, "render")?;
    }
    Ok(())
}

#[test]
fn real_raster_worker_enforces_source_request_and_rgba_limits() -> TestResult {
    let input_limit = workspace()?;
    assert_failure(input_limit.path(), 1, "input_limit")?;
    let output_limit = workspace()?;
    let mut request = br#"{"page_index":0,"scale_bits":1065353216,"max_rgba_bytes":1}"#.to_vec();
    request.resize(128, b' ');
    std::fs::write(output_limit.path().join("request.json"), request)?;
    assert_failure(output_limit.path(), pdf().len() as u64, "output_limit")?;
    let request_limit = workspace()?;
    std::fs::write(request_limit.path().join("request.json"), [b' '; 129])?;
    assert_failure(request_limit.path(), OUTPUT_LIMIT, "request_limit")?;
    let unknown_fields = workspace()?;
    std::fs::write(
        unknown_fields.path().join("request.json"),
        br#"{"page_index":0,"scale_bits":1065353216,"max_rgba_bytes":67108864,"unknown":true}"#,
    )?;
    assert_failure(unknown_fields.path(), OUTPUT_LIMIT, "request")?;
    Ok(())
}

#[test]
fn real_raster_worker_does_not_hide_missing_files_or_output_failures() -> TestResult {
    let missing = workspace()?;
    std::fs::remove_file(missing.path().join("input.office"))?;
    assert_failure(missing.path(), OUTPUT_LIMIT, "input")?;
    let rgba_failure = workspace()?;
    std::fs::create_dir(rgba_failure.path().join("frame.rgba"))?;
    assert_failure(rgba_failure.path(), OUTPUT_LIMIT, "rgba_write")?;
    let response_failure = workspace()?;
    std::fs::create_dir(response_failure.path().join("response.json"))?;
    assert_exit(response_failure.path(), OUTPUT_LIMIT, 70)?;
    assert!(response_failure.path().join("response.json").is_dir());
    Ok(())
}
