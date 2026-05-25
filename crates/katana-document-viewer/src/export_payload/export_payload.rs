use crate::export_html_payload::HtmlExportPayloadFactory;
use crate::export_image_payload::ImageExportPayloadFactory;
use crate::export_pdf_payload::PdfExportPayloadFactory;
use crate::export_surface::DocumentSurfaceFactory;
use crate::forge::{BuildGraph, ExportFormat, ForgeError};
use crate::theme::KdvThemeSnapshot;

pub(crate) struct ExportPayloadFactory;

impl ExportPayloadFactory {
    pub(crate) fn create(
        graph: &BuildGraph,
        format: ExportFormat,
        theme: &KdvThemeSnapshot,
    ) -> Result<Vec<u8>, ForgeError> {
        let bytes = match format {
            ExportFormat::Html => Ok(HtmlExportPayloadFactory::create(graph, theme)),
            ExportFormat::Pdf => {
                let surface = DocumentSurfaceFactory::create(graph, theme);
                PdfExportPayloadFactory::create(&surface)
            }
            ExportFormat::Png => {
                let surface = DocumentSurfaceFactory::create(graph, theme);
                ImageExportPayloadFactory::create_png(&surface)
            }
            ExportFormat::Jpeg => {
                let surface = DocumentSurfaceFactory::create(graph, theme);
                ImageExportPayloadFactory::create_jpeg(&surface)
            }
        };
        bytes.map_err(ForgeError::Export)
    }
}

#[cfg(test)]
#[path = "export_payload_contract_tests.rs"]
mod contract_tests;
