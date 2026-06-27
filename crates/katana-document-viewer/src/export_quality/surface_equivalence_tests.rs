use super::test_support::{combined_rgba, encode_jpeg, encode_png, fake_pdf, repeated_rgba};
use super::*;

const TEST_WIDTH: u32 = 2;
const TEST_HEIGHT: u32 = 2;
const PAGE_HEIGHT: u32 = 1;
const GAP_HEIGHT: u32 = 2;
const FIRST_PAGE_WITH_GAP_HEIGHT: u32 = PAGE_HEIGHT + GAP_HEIGHT;
const TEST_RGB_MIN: u8 = 0;
const TEST_RGB_MAX: u8 = 255;
const TEST_RGB_MID: u8 = 128;
const TEST_ALPHA_OPAQUE: u8 = 255;
const BLACK_PIXEL: [u8; RGBA_CHANNELS] =
    [TEST_RGB_MIN, TEST_RGB_MIN, TEST_RGB_MIN, TEST_ALPHA_OPAQUE];
const WHITE_PIXEL: [u8; RGBA_CHANNELS] =
    [TEST_RGB_MAX, TEST_RGB_MAX, TEST_RGB_MAX, TEST_ALPHA_OPAQUE];
const MID_PIXEL: [u8; RGBA_CHANNELS] =
    [TEST_RGB_MID, TEST_RGB_MIN, TEST_RGB_MIN, TEST_ALPHA_OPAQUE];

#[test]
fn identical_png_pdf_and_jpeg_pass() -> Result<(), Box<dyn std::error::Error>> {
    let rgba = repeated_rgba(BLACK_PIXEL, TEST_WIDTH, TEST_HEIGHT);
    let artifacts = artifacts(&rgba, TEST_WIDTH, TEST_HEIGHT, &rgba)?;

    let report = SurfaceEquivalenceGate::evaluate(&artifacts);

    assert!(report.is_pass(), "{report:#?}");
    assert_eq!(report.png_score, PERFECT_SCORE);
    assert_eq!(report.pdf_score, PERFECT_SCORE);
    assert!(report.jpeg_score >= SURFACE_EQUIVALENCE_THRESHOLD);
    Ok(())
}

#[test]
fn different_surface_below_threshold_fails() -> Result<(), Box<dyn std::error::Error>> {
    let viewer = repeated_rgba(BLACK_PIXEL, TEST_WIDTH, TEST_HEIGHT);
    let candidate = repeated_rgba(WHITE_PIXEL, TEST_WIDTH, TEST_HEIGHT);
    let artifacts = artifacts(&viewer, TEST_WIDTH, TEST_HEIGHT, &candidate)?;

    let report = SurfaceEquivalenceGate::evaluate(&artifacts);

    assert!(!report.is_pass());
    assert_eq!(report.minimum_score, ZERO_SCORE);
    Ok(())
}

#[test]
fn pdf_pages_are_compared_as_vertical_surface() -> Result<(), Box<dyn std::error::Error>> {
    let top = repeated_rgba(BLACK_PIXEL, TEST_WIDTH, PAGE_HEIGHT);
    let bottom = repeated_rgba(WHITE_PIXEL, TEST_WIDTH, PAGE_HEIGHT);
    let rgba = combined_rgba(&top, &bottom);
    let artifacts = page_stack_artifacts(&rgba, &top, &bottom)?;

    let report = SurfaceEquivalenceGate::evaluate(&artifacts);

    assert!(report.is_pass(), "{report:#?}");
    assert_eq!(report.pdf_score, PERFECT_SCORE);
    Ok(())
}

#[test]
fn pdf_page_gap_rows_are_normalized() -> Result<(), Box<dyn std::error::Error>> {
    let first = repeated_rgba(BLACK_PIXEL, TEST_WIDTH, PAGE_HEIGHT);
    let gap = repeated_rgba(WHITE_PIXEL, TEST_WIDTH, GAP_HEIGHT);
    let second = repeated_rgba(MID_PIXEL, TEST_WIDTH, PAGE_HEIGHT);
    let viewer = combined_rgba(&first, &second);
    let first_page = combined_rgba(&first, &gap);
    let artifacts = page_gap_artifacts(&viewer, &first_page, &second)?;

    let report = SurfaceEquivalenceGate::evaluate(&artifacts);

    assert!(report.is_pass(), "{report:#?}");
    assert_eq!(report.pdf_score, PERFECT_SCORE);
    Ok(())
}

fn page_stack_artifacts<'a>(
    viewer: &'a [u8],
    top: &[u8],
    bottom: &[u8],
) -> Result<SurfaceEquivalenceArtifacts<'a>, Box<dyn std::error::Error>> {
    let pdf = fake_pdf(&[
        (top, TEST_WIDTH, PAGE_HEIGHT),
        (bottom, TEST_WIDTH, PAGE_HEIGHT),
    ])?;
    comparable_artifacts(viewer, pdf)
}

fn page_gap_artifacts<'a>(
    viewer: &'a [u8],
    first_page: &[u8],
    second: &[u8],
) -> Result<SurfaceEquivalenceArtifacts<'a>, Box<dyn std::error::Error>> {
    let pdf = fake_pdf(&[
        (first_page, TEST_WIDTH, FIRST_PAGE_WITH_GAP_HEIGHT),
        (second, TEST_WIDTH, PAGE_HEIGHT),
    ])?;
    comparable_artifacts(viewer, pdf)
}

fn comparable_artifacts<'a>(
    viewer: &'a [u8],
    pdf: Vec<u8>,
) -> Result<SurfaceEquivalenceArtifacts<'a>, Box<dyn std::error::Error>> {
    let png = encode_png(viewer, TEST_WIDTH, TEST_HEIGHT)?;
    let jpeg = encode_jpeg(viewer, TEST_WIDTH, TEST_HEIGHT)?;
    Ok(SurfaceEquivalenceArtifacts {
        raster_reference: comparable_image(viewer),
        pdf_reference: comparable_image(viewer),
        pdf: Box::leak(pdf.into_boxed_slice()),
        png: Box::leak(png.into_boxed_slice()),
        jpeg: Box::leak(jpeg.into_boxed_slice()),
    })
}

fn comparable_image(rgba: &[u8]) -> SurfaceEquivalenceImage<'_> {
    SurfaceEquivalenceImage {
        width: TEST_WIDTH,
        height: TEST_HEIGHT,
        rgba,
    }
}

fn artifacts<'a>(
    viewer: &'a [u8],
    width: u32,
    height: u32,
    candidate: &[u8],
) -> Result<SurfaceEquivalenceArtifacts<'a>, Box<dyn std::error::Error>> {
    let png = encode_png(candidate, width, height)?;
    let jpeg = encode_jpeg(candidate, width, height)?;
    let pdf = fake_pdf(&[(candidate, width, height)])?;
    Ok(SurfaceEquivalenceArtifacts {
        raster_reference: SurfaceEquivalenceImage {
            width,
            height,
            rgba: viewer,
        },
        pdf_reference: SurfaceEquivalenceImage {
            width,
            height,
            rgba: viewer,
        },
        pdf: Box::leak(pdf.into_boxed_slice()),
        png: Box::leak(png.into_boxed_slice()),
        jpeg: Box::leak(jpeg.into_boxed_slice()),
    })
}
