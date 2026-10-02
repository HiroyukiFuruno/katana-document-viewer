use super::PdfViewerSession;
use crate::multi_format::debug_trace::DebugTrace;
use crate::multi_format::pdf_raster_worker_parent::PdfRasterWorkerRunner;
use crate::multi_format::{
    OfficeWorkerConfig, OfficeWorkerError, PdfPageRenderRequest, PdfRenderedPage,
};

impl PdfViewerSession {
    pub(in crate::multi_format) fn render_page_with_worker(
        &mut self,
        request: PdfPageRenderRequest,
        config: &OfficeWorkerConfig,
    ) -> Result<PdfRenderedPage, OfficeWorkerError> {
        self.validate_request(request)?;
        let key = (request.page_index, request.scale.to_bits());
        if let Some(rendered) = self.cache.get(key) {
            DebugTrace::event("pdf.cache", "hit=true");
            return Ok(rendered);
        }
        DebugTrace::event("pdf.cache", "hit=false");
        let _render = DebugTrace::start("pdf.render");
        let rendered = PdfRasterWorkerRunner::render(
            self.pdf.data().as_ref(),
            &self.artifact,
            request,
            self.limits,
            config,
        )?;
        self.cache.insert(key, rendered.clone(), self.limits);
        Ok(rendered)
    }
}
