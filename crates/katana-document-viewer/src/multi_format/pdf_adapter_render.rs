use super::super::pdf_surface::PdfSurfaceDecoder;
use super::{PdfPageRenderRequest, PdfRenderedPage, PdfViewerError, PdfViewerSession};
use hayro::hayro_interpret::InterpreterSettings;
use hayro::{RenderCache, RenderSettings, render};

macro_rules! traced {
    ($stage:literal, $expression:expr) => {{
        let _trace = super::super::debug_trace::DebugTrace::start($stage);
        $expression
    }};
}

impl PdfViewerSession {
    pub(super) fn render_uncached(
        &self,
        request: PdfPageRenderRequest,
    ) -> Result<PdfRenderedPage, PdfViewerError> {
        let surface = self.render_surface(request)?;
        Ok(PdfRenderedPage {
            page_index: request.page_index,
            scale: request.scale,
            surface,
        })
    }

    fn render_surface(
        &self,
        request: PdfPageRenderRequest,
    ) -> Result<crate::ViewerImageSurface, PdfViewerError> {
        let page = &self.pdf.pages()[request.page_index];
        let pixmap = traced!(
            "pdf.rasterize",
            render(
                page,
                &RenderCache::new(),
                &InterpreterSettings::default(),
                &render_settings(request.scale),
            )
        );
        let png = traced!(
            "pdf.image_encode",
            pixmap
                .into_png()
                .map_err(|_| PdfViewerError::RenderDecode)?
        );
        let surface = traced!(
            "pdf.frame_decode",
            PdfSurfaceDecoder::decode(&self.artifact, request, &png)?
        );
        Ok(surface)
    }
}

fn render_settings(scale: f32) -> RenderSettings {
    RenderSettings {
        x_scale: scale,
        y_scale: scale,
        ..RenderSettings::default()
    }
}
