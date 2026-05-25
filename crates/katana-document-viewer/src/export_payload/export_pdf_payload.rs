use crate::export_surface::DocumentSurface;

mod document;

pub(crate) struct PdfExportPayloadFactory;

impl PdfExportPayloadFactory {
    pub(crate) fn create(surface: &DocumentSurface) -> Result<Vec<u8>, String> {
        document::PdfImageDocument::new(surface).into_bytes()
    }
}
