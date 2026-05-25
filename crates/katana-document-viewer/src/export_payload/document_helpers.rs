#[derive(Clone, Copy)]
pub(crate) struct PdfDestination {
    page_object: usize,
    x: u32,
    y: u32,
}

pub(crate) struct PdfDocumentHelpers;

mod document_helpers_content;
mod document_helpers_xref;
