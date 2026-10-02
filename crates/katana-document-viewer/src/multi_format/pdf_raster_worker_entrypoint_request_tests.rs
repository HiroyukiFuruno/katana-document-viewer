use super::super::super::pdf_raster_worker_protocol::{MAX_REQUEST_BYTES, REQUEST_NAME};
use super::super::{PdfRasterRequest, read_request};

#[test]
fn request_reader_accepts_128_bytes_and_rejects_129_unknown_fields_and_symlinks()
-> Result<(), Box<dyn std::error::Error>> {
    let workspace = tempfile::tempdir()?;
    let request = PdfRasterRequest {
        page_index: 3,
        scale_bits: 1.25_f32.to_bits(),
        max_rgba_bytes: 67108864,
    };
    let request_path = workspace.path().join(REQUEST_NAME);
    assert_request_is_rejected(workspace.path())?;
    std::fs::create_dir(&request_path)?;
    assert_request_is_rejected(workspace.path())?;
    std::fs::remove_dir(&request_path)?;
    assert_exact_limit_is_accepted(workspace.path(), &request)?;
    assert_limit_plus_one_is_rejected(workspace.path(), &request)?;
    assert_unknown_fields_are_rejected(workspace.path())?;
    assert_symlinks_are_rejected(workspace.path(), &request)?;
    Ok(())
}

fn assert_request_is_rejected(workspace: &std::path::Path) -> Result<(), std::io::Error> {
    assert!(matches!(
        read_request(workspace),
        Err((stage, _)) if stage == "request"
    ));
    Ok(())
}

fn assert_exact_limit_is_accepted(
    workspace: &std::path::Path,
    request: &PdfRasterRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut encoded = serde_json::to_vec(request)?;
    encoded.resize(MAX_REQUEST_BYTES as usize, b' ');
    std::fs::write(workspace.join(REQUEST_NAME), encoded)?;
    assert_eq!(
        *request,
        read_request(workspace).map_err(super::failure_as_io)?
    );
    Ok(())
}

fn assert_limit_plus_one_is_rejected(
    workspace: &std::path::Path,
    request: &PdfRasterRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut encoded = serde_json::to_vec(request)?;
    encoded.resize(MAX_REQUEST_BYTES as usize, b' ');
    encoded.push(b' ');
    std::fs::write(workspace.join(REQUEST_NAME), encoded)?;
    assert!(matches!(
        read_request(workspace),
        Err((stage, _)) if stage == "request_limit"
    ));
    Ok(())
}

fn assert_unknown_fields_are_rejected(workspace: &std::path::Path) -> Result<(), std::io::Error> {
    std::fs::write(
        workspace.join(REQUEST_NAME),
        br#"{"page_index":3,"scale_bits":1067450368,"max_rgba_bytes":67108864,"extra":true}"#,
    )?;
    assert!(matches!(
        read_request(workspace),
        Err((stage, _)) if stage == "request"
    ));
    Ok(())
}

fn assert_symlinks_are_rejected(
    workspace: &std::path::Path,
    request: &PdfRasterRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        let target = workspace.join("request-target.json");
        std::fs::write(&target, serde_json::to_vec(request)?)?;
        let request_path = workspace.join(REQUEST_NAME);
        std::fs::remove_file(&request_path)?;
        std::os::unix::fs::symlink(target, request_path)?;
        assert!(matches!(
            read_request(workspace),
            Err((stage, _)) if stage == "request"
        ));
    }
    #[cfg(not(unix))]
    let _ = (workspace, request);
    Ok(())
}
