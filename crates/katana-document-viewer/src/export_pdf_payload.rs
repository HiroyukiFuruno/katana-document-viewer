use crate::export_surface::{DocumentSurface, SurfaceLinkAnchor, SurfaceLinkAnnotation};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use image::RgbaImage;
use std::io::Write;

pub(crate) struct PdfExportPayloadFactory;

impl PdfExportPayloadFactory {
    pub(crate) fn create(surface: &DocumentSurface) -> Result<Vec<u8>, String> {
        PdfImageDocument::new(surface).into_bytes()
    }
}

struct PdfImageDocument<'a> {
    surface: &'a DocumentSurface,
}

impl<'a> PdfImageDocument<'a> {
    fn new(surface: &'a DocumentSurface) -> Self {
        Self { surface }
    }

    fn into_bytes(self) -> Result<Vec<u8>, String> {
        let objects = self.objects()?;
        let mut output = b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n".to_vec();
        let mut offsets = Vec::with_capacity(objects.len());
        for object in objects {
            offsets.push(output.len());
            output.extend_from_slice(&object);
        }
        append_xref(&mut output, &offsets);
        Ok(output)
    }

    fn objects(&self) -> Result<Vec<Vec<u8>>, String> {
        let pages = self.pages();
        let page_annotations = self.annotations_by_page(pages.len());
        let page_objects = allocate_page_objects(&page_annotations);
        let destinations = pdf_destinations(&self.surface.link_anchors, &page_objects, pages);
        let mut objects = Vec::with_capacity(
            2 + pages.len() * 3 + page_annotations.iter().map(Vec::len).sum::<usize>(),
        );
        objects.push(ascii_object(1, "<< /Type /Catalog /Pages 2 0 R >>"));
        objects.push(self.pages_object(&page_objects));
        for (index, page) in pages.iter().enumerate() {
            let numbers = &page_objects[index];
            let content_stream = content_stream(page, numbers.image);
            let image_stream = compress(&rgb_bytes(page))?;
            objects.push(page_dictionary(
                page,
                numbers.page,
                numbers.content,
                numbers.image,
                &numbers.annotations,
            ));
            objects.push(stream_object(numbers.content, "<<", &content_stream));
            objects.push(image_dictionary(page, numbers.image, &image_stream));
            for (annotation, object_number) in page_annotations[index]
                .iter()
                .zip(numbers.annotations.iter())
            {
                let destination = pdf_link_destination(annotation, &destinations);
                objects.push(link_annotation_object(
                    *object_number,
                    annotation,
                    page.height(),
                    destination,
                ));
            }
        }
        Ok(objects)
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
        ascii_object(
            2,
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

struct PdfPageObjects {
    page: usize,
    content: usize,
    image: usize,
    annotations: Vec<usize>,
}

fn rgb_bytes(image: &RgbaImage) -> Vec<u8> {
    let mut rgb = Vec::with_capacity((image.width() * image.height() * 3) as usize);
    for pixel in image.pixels() {
        rgb.extend_from_slice(&[pixel[0], pixel[1], pixel[2]]);
    }
    rgb
}

fn content_stream(image: &RgbaImage, image_object: usize) -> Vec<u8> {
    let width = image.width();
    let height = image.height();
    format!("q\n{width} 0 0 {height} 0 0 cm\n/Im{image_object} Do\nQ\n").into_bytes()
}

fn page_dictionary(
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
    ascii_object(
        page_object,
        &format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {width} {height}] /Resources << /XObject << /Im{image_object} {image_object} 0 R >> >> /Contents {content_object} 0 R{annotations} >>"
        ),
    )
}

fn image_dictionary(image: &RgbaImage, image_object: usize, image_stream: &[u8]) -> Vec<u8> {
    let width = image.width();
    let height = image.height();
    let dictionary = format!(
        "<< /Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /FlateDecode"
    );
    stream_object(image_object, &dictionary, image_stream)
}

fn link_annotation_object(
    number: usize,
    annotation: &SurfaceLinkAnnotation,
    page_height: u32,
    destination: Option<PdfDestination>,
) -> Vec<u8> {
    let x1 = annotation.x;
    let x2 = annotation.x + annotation.width;
    let y1 = page_height.saturating_sub(annotation.y + annotation.height);
    let y2 = page_height.saturating_sub(annotation.y);
    let action = pdf_link_action(annotation, destination);
    ascii_object(
        number,
        &format!(
            "<< /Type /Annot /Subtype /Link /Rect [{x1} {y1} {x2} {y2}] /Border [0 0 0] /A {action} >>"
        ),
    )
}

#[derive(Clone, Copy)]
struct PdfDestination {
    page_object: usize,
    x: u32,
    y: u32,
}

fn pdf_link_action(
    annotation: &SurfaceLinkAnnotation,
    destination: Option<PdfDestination>,
) -> String {
    if annotation.target.starts_with('#') {
        let destination = destination.unwrap_or(PdfDestination {
            page_object: 3,
            x: 0,
            y: 0,
        });
        return format!(
            "<< /S /GoTo /D [{} 0 R /XYZ {} {} null] >>",
            destination.page_object, destination.x, destination.y
        );
    }
    format!(
        "<< /S /URI /URI ({}) >>",
        escape_pdf_string(&annotation.target)
    )
}

fn pdf_link_destination(
    annotation: &SurfaceLinkAnnotation,
    destinations: &[(String, PdfDestination)],
) -> Option<PdfDestination> {
    let target = annotation.target.strip_prefix('#')?;
    destinations
        .iter()
        .find_map(|(id, destination)| (id == target).then_some(*destination))
}

fn pdf_destinations(
    anchors: &[SurfaceLinkAnchor],
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

fn allocate_page_objects(page_annotations: &[Vec<&SurfaceLinkAnnotation>]) -> Vec<PdfPageObjects> {
    let mut next_object = 3usize;
    let mut pages = Vec::with_capacity(page_annotations.len());
    for annotations in page_annotations {
        let page = next_object;
        let content = next_object + 1;
        let image = next_object + 2;
        next_object += 3;
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

fn escape_pdf_string(value: &str) -> String {
    value
        .replace('\\', r"\\")
        .replace('(', r"\(")
        .replace(')', r"\)")
}

fn ascii_object(number: usize, body: &str) -> Vec<u8> {
    format!("{number} 0 obj\n{body}\nendobj\n").into_bytes()
}

fn stream_object(number: usize, dictionary: &str, stream: &[u8]) -> Vec<u8> {
    let mut object = format!(
        "{number} 0 obj\n{dictionary} /Length {} >>\nstream\n",
        stream.len()
    )
    .into_bytes();
    object.extend_from_slice(stream);
    object.extend_from_slice(b"\nendstream\nendobj\n");
    object
}

fn append_xref(output: &mut Vec<u8>, offsets: &[usize]) {
    let xref_start = output.len();
    output.extend_from_slice(format!("xref\n0 {}\n", offsets.len() + 1).as_bytes());
    output.extend_from_slice(b"0000000000 65535 f \n");
    for offset in offsets {
        output.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    output.extend_from_slice(
        format!(
            "trailer << /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
            offsets.len() + 1,
            xref_start
        )
        .as_bytes(),
    );
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
