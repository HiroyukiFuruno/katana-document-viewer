use super::office_worker_process::OfficeWorkerProcess;
use super::office_worker_protocol::{MAX_RESPONSE_BYTES, RESPONSE_NAME};
use super::office_worker_workspace::OfficeWorkerWorkspace;
use super::pdf_raster_worker_protocol::{
    PdfRasterRequest, PdfRasterResponse, REQUEST_NAME, RGBA_NAME,
};
use super::pdf_raster_worker_reader::read_regular_bounded;
use super::{
    OfficeWorkerConfig, OfficeWorkerError, PdfDocumentArtifact, PdfPageRenderRequest,
    PdfRenderedPage, PdfResourceLimitKind, PdfViewerError, PdfViewerLimits,
};
use crate::ViewerImageSurface;
use std::path::Path;

pub(super) struct PdfRasterWorkerRunner;

impl PdfRasterWorkerRunner {
    pub(super) fn render(
        bytes: &[u8],
        artifact: &PdfDocumentArtifact,
        request: PdfPageRenderRequest,
        limits: PdfViewerLimits,
        config: &OfficeWorkerConfig,
    ) -> Result<PdfRenderedPage, OfficeWorkerError> {
        let workspace = OfficeWorkerWorkspace::prepare("kdv-pdf-raster-worker-", bytes, config)?;
        let payload = PdfRasterRequest {
            page_index: request.page_index,
            scale_bits: request.scale.to_bits(),
            max_rgba_bytes: limits.max_render_pixels.saturating_mul(4),
        };
        let encoded = serde_json::to_vec(&payload).map_err(OfficeWorkerError::protocol_json)?;
        std::fs::write(workspace.path().join(REQUEST_NAME), encoded)
            .map_err(OfficeWorkerError::protocol_io)?;
        let status = OfficeWorkerProcess::run_pdf_raster(workspace.path(), config)?;
        let response = read_response(workspace.path(), status)?;
        let (width, height, length) = complete_response(response, status, request, limits)?;
        let rgba = read_rgba(&workspace.path().join(RGBA_NAME), length)?;
        Ok(rendered_page(artifact, request, width, height, rgba))
    }
}

fn read_rgba(path: &Path, length: u64) -> Result<Vec<u8>, OfficeWorkerError> {
    let rgba = read_regular_bounded(path, length)?;
    if rgba.len() as u64 != length {
        return Err(OfficeWorkerError::protocol(
            "raster RGBA length differs from response".to_owned(),
        ));
    }
    Ok(rgba)
}

fn read_response(
    workspace: &Path,
    status: Option<i64>,
) -> Result<PdfRasterResponse, OfficeWorkerError> {
    let result = read_regular_bounded(&workspace.join(RESPONSE_NAME), MAX_RESPONSE_BYTES)
        .and_then(|bytes| serde_json::from_slice(&bytes).map_err(OfficeWorkerError::protocol_json));
    match result {
        Ok(response) => Ok(response),
        Err(error) if status == Some(0) => Err(error),
        Err(_) => Err(OfficeWorkerError::WorkerCrashed { status }),
    }
}

fn complete_response(
    response: PdfRasterResponse,
    status: Option<i64>,
    request: PdfPageRenderRequest,
    limits: PdfViewerLimits,
) -> Result<(u32, u32, u64), OfficeWorkerError> {
    let (page_index, scale_bits, width, height, rgba_bytes) = match response {
        PdfRasterResponse::Completed {
            page_index,
            scale_bits,
            width,
            height,
            rgba_bytes,
        } => (page_index, scale_bits, width, height, rgba_bytes),
        PdfRasterResponse::Failed { stage, message } => {
            return Err(OfficeWorkerError::EngineFailure { stage, message });
        }
    };
    validate_response_request(status, page_index, scale_bits, request)?;
    validate_dimensions(
        width,
        height,
        rgba_bytes,
        limits,
        limits.max_render_pixels.saturating_mul(4),
    )?;
    Ok((width, height, rgba_bytes))
}

fn validate_response_request(
    status: Option<i64>,
    page_index: usize,
    scale_bits: u32,
    request: PdfPageRenderRequest,
) -> Result<(), OfficeWorkerError> {
    if status != Some(0) {
        return Err(OfficeWorkerError::WorkerCrashed { status });
    }
    if page_index != request.page_index || scale_bits != request.scale.to_bits() {
        return Err(OfficeWorkerError::protocol(
            "raster response does not match page/scale request".to_owned(),
        ));
    }
    Ok(())
}

fn validate_dimensions(
    width: u32,
    height: u32,
    bytes: u64,
    limits: PdfViewerLimits,
    output_limit: u64,
) -> Result<(), OfficeWorkerError> {
    let pixels = u64::from(width) * u64::from(height);
    if width == 0 || height == 0 || pixels.checked_mul(4) != Some(bytes) {
        return Err(OfficeWorkerError::protocol(
            "raster response has invalid RGBA dimensions".to_owned(),
        ));
    }
    check_pdf_limit(
        PdfResourceLimitKind::RenderDimension,
        u64::from(width.max(height)),
        u64::from(limits.max_render_dimension),
    )?;
    check_pdf_limit(
        PdfResourceLimitKind::RenderPixels,
        pixels,
        limits.max_render_pixels,
    )?;
    check_output_limit(bytes, output_limit)
}

fn check_output_limit(bytes: u64, output_limit: u64) -> Result<(), OfficeWorkerError> {
    if bytes > output_limit {
        return Err(OfficeWorkerError::OutputLimitExceeded {
            actual: bytes,
            limit: output_limit,
        });
    }
    Ok(())
}

fn check_pdf_limit(
    kind: PdfResourceLimitKind,
    actual: u64,
    limit: u64,
) -> Result<(), OfficeWorkerError> {
    if actual > limit {
        return Err(PdfViewerError::ResourceLimitExceeded {
            kind,
            actual,
            limit,
        }
        .into());
    }
    Ok(())
}

fn rendered_page(
    artifact: &PdfDocumentArtifact,
    request: PdfPageRenderRequest,
    width: u32,
    height: u32,
    rgba: Vec<u8>,
) -> PdfRenderedPage {
    PdfRenderedPage {
        page_index: request.page_index,
        scale: request.scale,
        surface: ViewerImageSurface {
            fingerprint: format!(
                "pdf:{}:{}:{}",
                artifact.identity.revision,
                request.page_index,
                request.scale.to_bits()
            ),
            width,
            height,
            display_width: width as f32,
            display_height: height as f32,
            content_scale: 100,
            rgba,
        },
    }
}

#[cfg(test)]
#[path = "pdf_raster_worker_parent_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "pdf_raster_worker_dimension_tests.rs"]
mod dimension_tests;
