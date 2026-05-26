use super::*;
use image::Rgba;

#[test]
fn pdf_pages_uses_image_when_pages_list_is_empty() {
    let surface = crate::export_surface::DocumentSurface {
        image: RgbaImage::from_pixel(2, 2, Rgba([1, 2, 3, 255])),
        pages: Vec::new(),
        link_annotations: Vec::new(),
        link_anchors: Vec::new(),
    };
    let document = PdfImageDocument::new(&surface);

    assert_eq!(
        document.pages(),
        &[RgbaImage::from_pixel(2, 2, Rgba([1, 2, 3, 255]))]
    );
}

#[test]
fn pdf_pages_prefers_multi_page_surface_pages() {
    let first = RgbaImage::from_pixel(2, 2, Rgba([10, 20, 30, 255]));
    let second = RgbaImage::from_pixel(2, 2, Rgba([40, 50, 60, 255]));
    let surface = crate::export_surface::DocumentSurface {
        image: RgbaImage::from_pixel(2, 2, Rgba([255, 255, 255, 255])),
        pages: vec![first.clone(), second.clone()],
        link_annotations: Vec::new(),
        link_anchors: Vec::new(),
    };
    let document = PdfImageDocument::new(&surface);

    assert_eq!(document.pages(), &[first, second]);
}

#[test]
fn annotations_by_page_groups_valid_pages_and_ignores_out_of_range() {
    let surface = crate::export_surface::DocumentSurface {
        image: RgbaImage::from_pixel(2, 2, Rgba([1, 2, 3, 255])),
        pages: vec![RgbaImage::from_pixel(2, 2, Rgba([4, 5, 6, 255]))],
        link_annotations: vec![
            link_annotation(0, "https://example.com"),
            link_annotation(3, "ignored"),
        ],
        link_anchors: Vec::new(),
    };
    let document = PdfImageDocument::new(&surface);

    let annotations = document.annotations_by_page(1);

    assert_eq!(annotations.len(), 1);
    assert_eq!(annotations[0].len(), 1);
    assert_eq!(annotations[0][0].target, "https://example.com");
}

const LINK_ANNOTATION_WIDTH: u32 = 3;
const LINK_ANNOTATION_HEIGHT: u32 = 4;

fn link_annotation(
    page_index: usize,
    target: &str,
) -> crate::export_surface::SurfaceLinkAnnotation {
    crate::export_surface::SurfaceLinkAnnotation {
        page_index,
        x: 1,
        y: 2,
        width: LINK_ANNOTATION_WIDTH,
        height: LINK_ANNOTATION_HEIGHT,
        target: target.to_string(),
    }
}

#[test]
fn pdf_compression_error_messages_include_operation() {
    assert!(pdf_compression_error(std::io::Error::other("write")).contains("compression failed"));
    assert!(
        pdf_compression_finish_error(std::io::Error::other("finish"))
            .contains("finalization failed")
    );
}
