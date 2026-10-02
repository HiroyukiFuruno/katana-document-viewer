use super::{complete_response, read_response, read_rgba, rendered_page};
use crate::multi_format::pdf_raster_worker_protocol::{
    MAX_REQUEST_BYTES, PdfRasterRequest, PdfRasterResponse,
};
use crate::multi_format::{
    BinaryDocumentSource, OfficeWorkerError, PdfPageRenderRequest, PdfViewerLimits,
    PdfViewerSession, ViewerSourceIdentity,
};

#[test]
fn raster_rgba_requires_the_actual_file_length_to_match_the_response()
-> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("frame.rgba");
    std::fs::write(&path, [1, 2, 3, 4])?;
    assert_eq!(vec![1, 2, 3, 4], read_rgba(&path, 4)?);
    assert!(matches!(
        read_rgba(&path, 5),
        Err(OfficeWorkerError::Protocol { .. })
    ));
    assert!(matches!(
        read_rgba(&path, 3),
        Err(OfficeWorkerError::OutputLimitExceeded { .. })
    ));
    Ok(())
}

fn completed() -> PdfRasterResponse {
    PdfRasterResponse::Completed {
        page_index: 0,
        scale_bits: 1.0_f32.to_bits(),
        width: 2,
        height: 3,
        rgba_bytes: 24,
    }
}

#[test]
fn raster_request_encoding_is_bounded_and_rejects_unknown_fields()
-> Result<(), Box<dyn std::error::Error>> {
    for request in [
        PdfRasterRequest {
            page_index: 0,
            scale_bits: 0,
            max_rgba_bytes: 0,
        },
        PdfRasterRequest {
            page_index: usize::MAX,
            scale_bits: u32::MAX,
            max_rgba_bytes: u64::MAX,
        },
    ] {
        let bytes = serde_json::to_vec(&request)?;
        assert!(bytes.len() as u64 <= MAX_REQUEST_BYTES);
        assert_eq!(request, serde_json::from_slice::<PdfRasterRequest>(&bytes)?);
    }
    assert!(
        serde_json::from_str::<PdfRasterRequest>(
            r#"{"page_index":0,"scale_bits":1065353216,"max_rgba_bytes":67108864,"extra":true}"#
        )
        .is_err()
    );
    Ok(())
}

#[test]
fn completed_raster_requires_the_request_identity_and_success_status()
-> Result<(), Box<dyn std::error::Error>> {
    let request = PdfPageRenderRequest::new(0, 1.0);
    let limits = PdfViewerLimits::strict();
    assert_eq!(
        (2, 3, 24),
        complete_response(completed(), Some(0), request, limits)?
    );
    for status in [None, Some(1)] {
        assert_eq!(
            Err(OfficeWorkerError::WorkerCrashed { status }),
            complete_response(completed(), status, request, limits)
        );
    }
    for request in [
        PdfPageRenderRequest::new(1, 1.0),
        PdfPageRenderRequest::new(0, 2.0),
    ] {
        assert!(matches!(
            complete_response(completed(), Some(0), request, limits),
            Err(OfficeWorkerError::Protocol { .. })
        ));
    }
    Ok(())
}

#[test]
fn failed_raster_is_not_hidden_as_a_pdf_decode_failure() {
    let response = PdfRasterResponse::Failed {
        stage: "sandbox".to_owned(),
        message: "denied".to_owned(),
    };
    assert_eq!(
        Err(OfficeWorkerError::EngineFailure {
            stage: "sandbox".to_owned(),
            message: "denied".to_owned()
        }),
        complete_response(
            response,
            Some(70),
            PdfPageRenderRequest::new(0, 1.0),
            PdfViewerLimits::strict()
        )
    );
}

#[test]
fn raster_response_distinguishes_crash_from_success_with_malformed_output()
-> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("response.json");
    assert_eq!(
        Err(OfficeWorkerError::WorkerCrashed { status: Some(64) }),
        read_response(dir.path(), Some(64))
    );
    assert!(matches!(
        read_response(dir.path(), Some(0)),
        Err(OfficeWorkerError::Protocol { .. })
    ));
    std::fs::write(&path, b"not JSON")?;
    assert!(matches!(
        read_response(dir.path(), Some(0)),
        Err(OfficeWorkerError::Protocol { .. })
    ));
    std::fs::write(&path, serde_json::to_vec(&completed())?)?;
    assert_eq!(completed(), read_response(dir.path(), Some(0))?);
    Ok(())
}

#[test]
fn raster_surface_retains_the_parent_revision_fingerprint_and_display_contract()
-> Result<(), Box<dyn std::error::Error>> {
    let session = PdfViewerSession::open(BinaryDocumentSource::new(
        ViewerSourceIdentity::new("fixture:sample", "immutable"),
        "application/pdf",
        include_bytes!("../../../../assets/reference/katana/pdf/sample.pdf").to_vec(),
    ))?;
    let request = PdfPageRenderRequest::new(0, 1.0);
    let rgba = vec![255; 24];
    let page = rendered_page(session.artifact(), request, 2, 3, rgba.clone());
    assert_eq!("pdf:immutable:0:1065353216", page.surface.fingerprint);
    assert_eq!(2.0, page.surface.display_width);
    assert_eq!(3.0, page.surface.display_height);
    assert_eq!(100, page.surface.content_scale);
    assert_eq!(rgba, page.surface.rgba);
    assert_eq!(request.scale, page.scale);
    assert_eq!(request.page_index, page.page_index);
    Ok(())
}
