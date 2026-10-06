use super::{OfficeStaticViewerSession, PdfPageRenderRequest, PdfRenderedPage};

impl OfficeStaticViewerSession {
    pub(in crate::multi_format) fn cached_rendered_item(
        &mut self,
        request: PdfPageRenderRequest,
    ) -> Option<PdfRenderedPage> {
        self.pdf.cached_rendered_page(request)
    }

    pub(in crate::multi_format) fn restore_rendered_item(&mut self, page: &PdfRenderedPage) {
        self.pdf.restore_rendered_page(page);
    }
}
