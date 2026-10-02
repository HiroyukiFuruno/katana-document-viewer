use super::{complete_response, validate_dimensions};
use crate::multi_format::pdf_raster_worker_protocol::PdfRasterResponse;
use crate::multi_format::{
    OfficeWorkerError, PdfPageRenderRequest, PdfResourceLimitKind, PdfViewerError, PdfViewerLimits,
};

#[test]
fn completed_raster_preserves_invalid_dimension_errors() {
    let response = PdfRasterResponse::Completed {
        page_index: 0,
        scale_bits: 1.0_f32.to_bits(),
        width: 0,
        height: 3,
        rgba_bytes: 0,
    };
    assert!(matches!(
        complete_response(
            response,
            Some(0),
            PdfPageRenderRequest::new(0, 1.0),
            PdfViewerLimits::strict()
        ),
        Err(OfficeWorkerError::Protocol { .. })
    ));
}

#[test]
fn raster_dimensions_reject_invalid_rgba_and_preserve_output_error() {
    let limits = PdfViewerLimits::strict();
    for dimensions in [(0, 1, 0), (1, 0, 0), (2, 3, 23), (u32::MAX, u32::MAX, 0)] {
        assert!(matches!(
            validate_dimensions(dimensions.0, dimensions.1, dimensions.2, limits, 128),
            Err(OfficeWorkerError::Protocol { .. })
        ));
    }
    assert_eq!(
        Err(OfficeWorkerError::OutputLimitExceeded {
            actual: 24,
            limit: 23
        }),
        validate_dimensions(2, 3, 24, limits, 23)
    );
}

#[test]
fn raster_dimensions_preserve_pdf_resource_limit_errors() {
    let limits = PdfViewerLimits::strict();
    assert!(matches!(
        validate_dimensions(8193, 1, 32772, limits, 65536),
        Err(OfficeWorkerError::Pdf(
            PdfViewerError::ResourceLimitExceeded {
                kind: PdfResourceLimitKind::RenderDimension,
                ..
            }
        ))
    ));
    assert!(matches!(
        validate_dimensions(4097, 4097, 67141636, limits, 134217728),
        Err(OfficeWorkerError::Pdf(
            PdfViewerError::ResourceLimitExceeded {
                kind: PdfResourceLimitKind::RenderPixels,
                ..
            }
        ))
    ));
}
