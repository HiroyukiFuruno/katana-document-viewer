use super::{BinaryDocumentSource, Failure, PdfRasterRequest, PdfRasterResponse, WorkerArguments};
use crate::multi_format::{PdfPageRenderRequest, PdfViewerLimits, PdfViewerSession};

pub(super) fn render_and_write(
    arguments: &WorkerArguments,
    source: BinaryDocumentSource,
    request: PdfRasterRequest,
) -> Result<PdfRasterResponse, Failure> {
    let rendered = render_page(source, &request)?;
    let rgba_bytes = rendered.surface.rgba.len() as u64;
    let output_limit = request.max_rgba_bytes.min(
        PdfViewerLimits::strict()
            .max_render_pixels
            .saturating_mul(4),
    );
    ensure_output_limit(rgba_bytes, output_limit)?;
    write_rgba(&arguments.workspace, &rendered.surface.rgba)?;
    Ok(PdfRasterResponse::Completed {
        page_index: rendered.page_index,
        scale_bits: request.scale_bits,
        width: rendered.surface.width,
        height: rendered.surface.height,
        rgba_bytes,
    })
}

fn render_page(
    source: BinaryDocumentSource,
    request: &PdfRasterRequest,
) -> Result<crate::multi_format::PdfRenderedPage, Failure> {
    let scale = f32::from_bits(request.scale_bits);
    let mut session = PdfViewerSession::open(source).map_err(|error| failure("open", error))?;
    session
        .render_page(PdfPageRenderRequest::new(request.page_index, scale))
        .map_err(|error| failure("render", error))
}

fn ensure_output_limit(rgba_bytes: u64, max_output_bytes: u64) -> Result<(), Failure> {
    if rgba_bytes <= max_output_bytes {
        return Ok(());
    }
    Err((
        "output_limit".to_owned(),
        format!("rendered RGBA is {rgba_bytes} bytes and exceeds {max_output_bytes} bytes"),
    ))
}

fn write_rgba(workspace: &std::path::Path, rgba: &[u8]) -> Result<(), Failure> {
    std::fs::write(
        workspace.join(super::super::pdf_raster_worker_protocol::RGBA_NAME),
        rgba,
    )
    .map_err(|error| failure("rgba_write", error))
}

fn failure(stage: &str, error: impl std::fmt::Display) -> Failure {
    (stage.to_owned(), error.to_string())
}
