use crate::export_surface::{DocumentSurface, SurfaceLinkAnnotation};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use image::RgbaImage;
use std::io::Write;

#[path = "document_helpers.rs"]
mod document_helpers;

use document_helpers::{PdfDestination, PdfDocumentHelpers};

pub(crate) struct PdfImageDocument<'a> {
    surface: &'a DocumentSurface,
}

impl<'a> PdfImageDocument<'a> {
    pub(crate) fn new(surface: &'a DocumentSurface) -> Self {
        Self { surface }
    }

    pub(crate) fn into_bytes(self) -> Result<Vec<u8>, String> {
        let objects = self.objects()?;
        let mut output = b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n".to_vec();
        let mut offsets = Vec::with_capacity(objects.len());
        for object in objects {
            offsets.push(output.len());
            output.extend_from_slice(&object);
        }
        PdfDocumentHelpers::append_xref(&mut output, &offsets);
        Ok(output)
    }

    fn objects(&self) -> Result<Vec<Vec<u8>>, String> {
        let pages = self.pages();
        let page_annotations = self.annotations_by_page(pages.len());
        let page_objects = PdfDocumentHelpers::allocate_page_objects(&page_annotations);
        let mut objects = Vec::with_capacity(
            PDF_ROOT_OBJECT_COUNT
                + pages.len() * PDF_OBJECTS_PER_PAGE
                + page_annotations.iter().map(Vec::len).sum::<usize>(),
        );
        objects.push(PdfDocumentHelpers::ascii_object(
            PDF_CATALOG_OBJECT,
            "<< /Type /Catalog /Pages 2 0 R >>",
        ));
        objects.push(self.pages_object(&page_objects));
        self.append_page_objects(&mut objects, pages, &page_annotations, &page_objects)?;
        Ok(objects)
    }

    fn append_page_objects(
        &self,
        objects: &mut Vec<Vec<u8>>,
        pages: &[RgbaImage],
        page_annotations: &[Vec<&SurfaceLinkAnnotation>],
        page_objects: &[PdfPageObjects],
    ) -> Result<(), String> {
        let destinations =
            PdfDocumentHelpers::pdf_destinations(&self.surface.link_anchors, page_objects, pages);
        for (index, page) in pages.iter().enumerate() {
            self.append_single_page_objects(
                objects,
                page,
                &page_annotations[index],
                &page_objects[index],
                &destinations,
            )?;
        }
        Ok(())
    }

    fn append_single_page_objects(
        &self,
        objects: &mut Vec<Vec<u8>>,
        page: &RgbaImage,
        annotations: &[&SurfaceLinkAnnotation],
        numbers: &PdfPageObjects,
        destinations: &[(String, PdfDestination)],
    ) -> Result<(), String> {
        let content_stream = PdfDocumentHelpers::content_stream(page, numbers.image);
        let image_stream = compress(PdfDocumentHelpers::rgb_bytes(page).as_slice())?;
        objects.push(PdfDocumentHelpers::page_dictionary(
            page,
            numbers.page,
            numbers.content,
            numbers.image,
            &numbers.annotations,
        ));
        objects.push(PdfDocumentHelpers::stream_object(
            numbers.content,
            "<<",
            &content_stream,
        ));
        objects.push(PdfDocumentHelpers::image_dictionary(
            page,
            numbers.image,
            &image_stream,
        ));
        append_annotation_objects(objects, page, annotations, numbers, destinations);
        Ok(())
    }

    fn pages(&self) -> &[RgbaImage] {
        if self.surface.pages.is_empty() {
            std::slice::from_ref(&self.surface.image)
        } else {
            &self.surface.pages
        }
    }

    fn pages_object(&self, pages: &[PdfPageObjects]) -> Vec<u8> {
        let kids = pages
            .iter()
            .map(|page| format!("{} 0 R", page.page))
            .collect::<Vec<_>>()
            .join(" ");
        PdfDocumentHelpers::ascii_object(
            PDF_PAGES_OBJECT,
            &format!("<< /Type /Pages /Kids [{kids}] /Count {} >>", pages.len()),
        )
    }

    fn annotations_by_page(&self, page_count: usize) -> Vec<Vec<&SurfaceLinkAnnotation>> {
        let mut annotations = (0..page_count).map(|_| Vec::new()).collect::<Vec<_>>();
        for annotation in &self.surface.link_annotations {
            if let Some(page) = annotations.get_mut(annotation.page_index) {
                page.push(annotation);
            }
        }
        annotations
    }
}

fn append_annotation_objects(
    objects: &mut Vec<Vec<u8>>,
    page: &RgbaImage,
    annotations: &[&SurfaceLinkAnnotation],
    numbers: &PdfPageObjects,
    destinations: &[(String, PdfDestination)],
) {
    for (annotation, object_number) in annotations.iter().zip(numbers.annotations.iter()) {
        let destination = PdfDocumentHelpers::pdf_link_destination(annotation, destinations);
        objects.push(PdfDocumentHelpers::link_annotation_object(
            *object_number,
            numbers.page,
            annotation,
            page.height(),
            destination,
        ));
    }
}

const PDF_CATALOG_OBJECT: usize = 1;
const PDF_PAGES_OBJECT: usize = 2;
const PDF_ROOT_OBJECT_COUNT: usize = 2;
const PDF_OBJECTS_PER_PAGE: usize = 3;

struct PdfPageObjects {
    page: usize,
    content: usize,
    image: usize,
    annotations: Vec<usize>,
}

fn compress(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(bytes)
        .map_err(|error| format!("PDF image stream compression failed: {error}"))?;
    let compressed = encoder
        .finish()
        .map_err(|error| format!("PDF image stream compression finalization failed: {error}"))?;
    Ok(compressed)
}
