use super::test_support::{
    combined_rgba, encode_jpeg, encode_png, fake_pdf, repeated_rgba, surface_image,
};
use super::*;

const TEST_WIDTH: u32 = 2;
const TEST_HEIGHT: u32 = 2;
const PAGE_HEIGHT: u32 = 1;
const BLACK_PIXEL: [u8; RGBA_CHANNELS] = [0, 0, 0, 255];
const MID_PIXEL: [u8; RGBA_CHANNELS] = [128, 0, 0, 255];

#[test]
fn decode_failures_score_zero_and_report_failures() {
    let rgba = repeated_rgba(BLACK_PIXEL, TEST_WIDTH, TEST_HEIGHT);
    let artifacts = SurfaceEquivalenceArtifacts {
        raster_reference: comparable_image(&rgba),
        pdf_reference: comparable_image(&rgba),
        pdf: b"%PDF-1.4",
        png: b"not png",
        jpeg: b"not jpeg",
    };

    let report = SurfaceEquivalenceGate::evaluate(&artifacts);

    assert_eq!(ZERO_SCORE, report.minimum_score);
    assert_failure_contains(&report, "png decode failed");
    assert_failure_contains(&report, "jpeg decode failed");
    assert_failure_contains(&report, "pdf decode failed");
}

#[test]
fn invalid_viewer_rgba_scores_zero() -> Result<(), Box<dyn std::error::Error>> {
    let viewer = vec![0, 0, 0];
    let candidate = repeated_rgba(BLACK_PIXEL, TEST_WIDTH, TEST_HEIGHT);
    let artifacts = artifacts(&viewer, TEST_WIDTH, TEST_HEIGHT, &candidate)?;

    let report = SurfaceEquivalenceGate::evaluate(&artifacts);

    assert_eq!(ZERO_SCORE, report.minimum_score);
    assert!(
        report
            .failures
            .iter()
            .any(|failure| failure.contains("rgba byte length"))
    );
    Ok(())
}

#[test]
fn dimension_mismatch_scores_zero() -> Result<(), Box<dyn std::error::Error>> {
    let viewer = repeated_rgba(BLACK_PIXEL, TEST_WIDTH, TEST_HEIGHT);
    let candidate = repeated_rgba(BLACK_PIXEL, TEST_WIDTH + 1, TEST_HEIGHT);
    let png = encode_png(&candidate, TEST_WIDTH + 1, TEST_HEIGHT)?;
    let jpeg = encode_jpeg(&candidate, TEST_WIDTH + 1, TEST_HEIGHT)?;
    let pdf = fake_pdf(&[(&candidate, TEST_WIDTH + 1, TEST_HEIGHT)])?;
    let artifacts = SurfaceEquivalenceArtifacts {
        raster_reference: comparable_image(&viewer),
        pdf_reference: comparable_image(&viewer),
        pdf: Box::leak(pdf.into_boxed_slice()),
        png: Box::leak(png.into_boxed_slice()),
        jpeg: Box::leak(jpeg.into_boxed_slice()),
    };

    let report = SurfaceEquivalenceGate::evaluate(&artifacts);

    assert_eq!(ZERO_SCORE, report.minimum_score);
    assert!(
        report
            .failures
            .iter()
            .any(|failure| failure == "surface dimensions differ")
    );
    Ok(())
}

#[test]
fn non_blank_extra_pdf_rows_are_not_normalized() -> Result<(), Box<dyn std::error::Error>> {
    let viewer = combined_rgba(&BLACK_PIXEL, &MID_PIXEL);
    let first = viewer.clone();
    let extra = combined_rgba(&MID_PIXEL, &BLACK_PIXEL);
    let pdf_page = combined_rgba(&first, &extra);
    let pdf = fake_pdf(&[(&pdf_page, TEST_WIDTH, PAGE_HEIGHT + 1)])?;
    let png = encode_png(&viewer, TEST_WIDTH, PAGE_HEIGHT)?;
    let jpeg = encode_jpeg(&viewer, TEST_WIDTH, PAGE_HEIGHT)?;
    let artifacts = SurfaceEquivalenceArtifacts {
        raster_reference: surface_image(&viewer, TEST_WIDTH, PAGE_HEIGHT),
        pdf_reference: surface_image(&viewer, TEST_WIDTH, PAGE_HEIGHT),
        pdf: Box::leak(pdf.into_boxed_slice()),
        png: Box::leak(png.into_boxed_slice()),
        jpeg: Box::leak(jpeg.into_boxed_slice()),
    };

    let report = SurfaceEquivalenceGate::evaluate(&artifacts);

    assert_eq!(ZERO_SCORE, report.pdf_score);
    Ok(())
}

fn assert_failure_contains(report: &SurfaceEquivalenceReport, message: &str) {
    assert!(
        report
            .failures
            .iter()
            .any(|failure| failure.contains(message))
    );
}

fn comparable_image(rgba: &[u8]) -> SurfaceEquivalenceImage<'_> {
    surface_image(rgba, TEST_WIDTH, TEST_HEIGHT)
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
