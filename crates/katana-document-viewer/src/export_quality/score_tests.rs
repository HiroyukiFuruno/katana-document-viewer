use super::*;
use crate::export_quality::surface_equivalence::test_support::{
    encode_jpeg, encode_png, fake_pdf, repeated_rgba,
};
use crate::export_quality::{SurfaceEquivalenceArtifacts, SurfaceEquivalenceImage};

const SURFACE_TEST_WIDTH: u32 = 2;
const SURFACE_TEST_HEIGHT: u32 = 2;
const BLACK_PIXEL: [u8; 4] = [0, 0, 0, 255];
const WHITE_PIXEL: [u8; 4] = [255, 255, 255, 255];

#[test]
fn evaluate_reports_warnings_for_failed_formats() {
    let artifacts = ExportQualityArtifacts {
        html: b"",
        pdf: b"",
        png: b"",
        jpeg: b"",
        source_markdown: "",
        surface_equivalence: None,
    };

    let report = ExportQualityGate::evaluate(&artifacts);

    assert!(!report.is_pass());
    assert!(
        report
            .warnings
            .iter()
            .any(|warning| warning.contains("Html"))
    );
    assert!(!report.fatal_failures.is_empty());
}

#[test]
fn html_anchor_without_closing_tag_does_not_require_pdf_link_annotation() {
    let artifacts = ExportQualityArtifacts {
        html: b"<main data-kdv-export><style data-kdv-export-style></style></main>",
        pdf: b"plain text",
        png: b"",
        jpeg: b"",
        source_markdown: r#"<a href="https://example.com""#,
        surface_equivalence: None,
    };

    let report = ExportQualityGate::evaluate(&artifacts);

    assert!(
        !report
            .fatal_failures
            .iter()
            .any(|failure| failure == "Pdf: pdf keeps link annotations"),
        "{report:#?}"
    );
}

#[test]
fn html_anchor_without_link_annotation_causes_pdf_failure() {
    let artifacts = ExportQualityArtifacts {
        html: b"<main data-kdv-export><style data-kdv-export-style></style></main>",
        pdf: b"%PDF-1.4",
        png: b"",
        jpeg: b"",
        source_markdown: "<a href=\"https://example.com\"></a>",
        surface_equivalence: None,
    };

    let report = ExportQualityGate::evaluate(&artifacts);

    assert!(
        report
            .fatal_failures
            .iter()
            .any(|failure| failure == "Pdf: pdf keeps link annotations")
    );
}

#[test]
fn evaluate_reports_surface_equivalence_failure_when_reference_differs()
-> Result<(), Box<dyn std::error::Error>> {
    let viewer = repeated_rgba(BLACK_PIXEL, SURFACE_TEST_WIDTH, SURFACE_TEST_HEIGHT);
    let candidate = repeated_rgba(WHITE_PIXEL, SURFACE_TEST_WIDTH, SURFACE_TEST_HEIGHT);
    let surface = surface_artifacts(&viewer, &candidate)?;
    let artifacts = artifacts_with_surface(Some(surface));

    let report = ExportQualityGate::evaluate(&artifacts);

    assert!(
        report
            .fatal_failures
            .iter()
            .any(|failure| failure.starts_with("Surface: ")),
        "{report:#?}"
    );
    assert!(
        report
            .warnings
            .iter()
            .any(|warning| warning.contains("surface equivalence score")),
        "{report:#?}"
    );
    Ok(())
}

#[test]
fn evaluate_keeps_surface_equivalence_clean_when_reference_matches()
-> Result<(), Box<dyn std::error::Error>> {
    let viewer = repeated_rgba(BLACK_PIXEL, SURFACE_TEST_WIDTH, SURFACE_TEST_HEIGHT);
    let surface = surface_artifacts(&viewer, &viewer)?;
    let artifacts = artifacts_with_surface(Some(surface));

    let report = ExportQualityGate::evaluate(&artifacts);

    assert!(
        !report
            .fatal_failures
            .iter()
            .any(|failure| failure.starts_with("Surface: ")),
        "{report:#?}"
    );
    assert!(
        !report
            .warnings
            .iter()
            .any(|warning| warning.contains("surface equivalence score")),
        "{report:#?}"
    );
    Ok(())
}

fn artifacts_with_surface<'a>(
    surface_equivalence: Option<SurfaceEquivalenceArtifacts<'a>>,
) -> ExportQualityArtifacts<'a> {
    ExportQualityArtifacts {
        html: b"",
        pdf: b"",
        png: b"",
        jpeg: b"",
        source_markdown: "",
        surface_equivalence,
    }
}

fn surface_artifacts<'a>(
    viewer: &'a [u8],
    candidate: &[u8],
) -> Result<SurfaceEquivalenceArtifacts<'a>, Box<dyn std::error::Error>> {
    let png = encode_png(candidate, SURFACE_TEST_WIDTH, SURFACE_TEST_HEIGHT)?;
    let jpeg = encode_jpeg(candidate, SURFACE_TEST_WIDTH, SURFACE_TEST_HEIGHT)?;
    let pdf = fake_pdf(&[(candidate, SURFACE_TEST_WIDTH, SURFACE_TEST_HEIGHT)])?;
    Ok(SurfaceEquivalenceArtifacts {
        raster_reference: surface_image(viewer),
        pdf_reference: surface_image(viewer),
        pdf: Box::leak(pdf.into_boxed_slice()),
        png: Box::leak(png.into_boxed_slice()),
        jpeg: Box::leak(jpeg.into_boxed_slice()),
    })
}

fn surface_image(rgba: &[u8]) -> SurfaceEquivalenceImage<'_> {
    SurfaceEquivalenceImage {
        width: SURFACE_TEST_WIDTH,
        height: SURFACE_TEST_HEIGHT,
        rgba,
    }
}
