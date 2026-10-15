use super::{PdfPageRenderRequest, PdfRenderedPage, PdfViewerSession};
use crate::multi_format::pdf_render_cache::PdfPageCache;

impl PdfViewerSession {
    pub(in crate::multi_format) fn clear_rendered_pages(&mut self) {
        self.cache = PdfPageCache::new();
    }

    pub(in crate::multi_format) fn cached_rendered_page(
        &mut self,
        request: PdfPageRenderRequest,
    ) -> Option<PdfRenderedPage> {
        self.cache
            .get((request.page_index, request.scale.to_bits()))
    }

    pub(in crate::multi_format) fn restore_rendered_page(&mut self, page: &PdfRenderedPage) {
        self.cache.insert(
            (page.page_index, page.scale.to_bits()),
            page.clone(),
            self.limits,
        );
    }
}
