use crate::export_surface::SurfaceLinkAnnotation;
use image::RgbaImage;

use super::super::PdfPageObjects;
use super::{PdfDestination, PdfDocumentHelpers};

impl PdfDocumentHelpers {
    pub(crate) fn rgb_bytes(image: &RgbaImage) -> Vec<u8> {
        let mut rgb =
            Vec::with_capacity((image.width() * image.height() * PDF_COLOR_CHANNEL_COUNT) as usize);
        for pixel in image.pixels() {
            rgb.extend_from_slice(&[pixel[0], pixel[1], pixel[2]]);
        }
        rgb
    }

    pub(crate) fn content_stream(image: &RgbaImage, image_object: usize) -> Vec<u8> {
        let width = image.width();
        let height = image.height();
        format!("q\n{width} 0 0 {height} 0 0 cm\n/Im{image_object} Do\nQ\n").into_bytes()
    }

    pub(crate) fn page_dictionary(
        image: &RgbaImage,
        page_object: usize,
        content_object: usize,
        image_object: usize,
        annotation_objects: &[usize],
    ) -> Vec<u8> {
        let width = image.width();
        let height = image.height();
        let annotations = if annotation_objects.is_empty() {
            String::new()
        } else {
            let refs = annotation_objects
                .iter()
                .map(|object| format!("{object} 0 R"))
                .collect::<Vec<_>>()
                .join(" ");
            format!(" /Annots [{refs}]")
        };
        Self::ascii_object(
            page_object,
            &format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {width} {height}] /Resources << /XObject << /Im{image_object} {image_object} 0 R >> >> /Contents {content_object} 0 R{annotations} >>"
            ),
        )
    }

    pub(crate) fn image_dictionary(
        image: &RgbaImage,
        image_object: usize,
        image_stream: &[u8],
    ) -> Vec<u8> {
        let width = image.width();
        let height = image.height();
        let dictionary = format!(
            "<< /Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /FlateDecode"
        );
        Self::stream_object(image_object, &dictionary, image_stream)
    }

    pub(crate) fn link_annotation_object(
        number: usize,
        page_object: usize,
        annotation: &SurfaceLinkAnnotation,
        page_height: u32,
        destination: Option<PdfDestination>,
    ) -> Vec<u8> {
        let x1 = annotation.x;
        let x2 = annotation.x + annotation.width;
        let y1 = page_height.saturating_sub(annotation.y + annotation.height);
        let y2 = page_height.saturating_sub(annotation.y);
        let target = Self::pdf_link_target(annotation, destination);
        Self::ascii_object(
            number,
            &format!(
                "<< /Type /Annot /Subtype /Link /Rect [{x1} {y1} {x2} {y2}] /Border [0 0 0] /F 4 /H /I /P {page_object} 0 R {target} >>"
            ),
        )
    }

    pub(crate) fn pdf_link_target(
        annotation: &SurfaceLinkAnnotation,
        destination: Option<PdfDestination>,
    ) -> String {
        if annotation.target.starts_with('#') {
            let destination = destination.unwrap_or(PdfDestination {
                page_object: PDF_DEFAULT_PAGE_OBJECT,
                x: 0,
                y: 0,
            });
            return format!(
                "/Dest [{} 0 R /XYZ {} {} null]",
                destination.page_object, destination.x, destination.y
            );
        }
        format!(
            "/A << /Type /Action /S /URI /URI ({}) >>",
            Self::escape_pdf_string(&annotation.target)
        )
    }

    pub(crate) fn pdf_link_destination(
        annotation: &SurfaceLinkAnnotation,
        destinations: &[(String, PdfDestination)],
    ) -> Option<PdfDestination> {
        let target = annotation.target.strip_prefix('#')?;
        destinations
            .iter()
            .find_map(|(id, destination)| (id == target).then_some(*destination))
    }

    pub(crate) fn pdf_destinations(
        anchors: &[crate::export_surface::SurfaceLinkAnchor],
        page_objects: &[PdfPageObjects],
        pages: &[RgbaImage],
    ) -> Vec<(String, PdfDestination)> {
        anchors
            .iter()
            .filter_map(|anchor| {
                let page_object = page_objects.get(anchor.page_index)?.page;
                let page_height = pages.get(anchor.page_index)?.height();
                Some((
                    anchor.id.clone(),
                    PdfDestination {
                        page_object,
                        x: anchor.x,
                        y: page_height.saturating_sub(anchor.y),
                    },
                ))
            })
            .collect()
    }

    pub(crate) fn allocate_page_objects(
        page_annotations: &[Vec<&SurfaceLinkAnnotation>],
    ) -> Vec<PdfPageObjects> {
        let mut next_object = PDF_FIRST_PAGE_OBJECT;
        let mut pages = Vec::with_capacity(page_annotations.len());
        for annotations in page_annotations {
            let page = next_object;
            let content = next_object + 1;
            let image = next_object + 2;
            next_object += PDF_OBJECTS_PER_PAGE;
            let annotation_objects = (0..annotations.len())
                .map(|_| {
                    let object = next_object;
                    next_object += 1;
                    object
                })
                .collect();
            pages.push(PdfPageObjects {
                page,
                content,
                image,
                annotations: annotation_objects,
            });
        }
        pages
    }
}

const PDF_COLOR_CHANNEL_COUNT: u32 = 3;
const PDF_DEFAULT_PAGE_OBJECT: usize = 3;
const PDF_FIRST_PAGE_OBJECT: usize = 3;
const PDF_OBJECTS_PER_PAGE: usize = 3;
