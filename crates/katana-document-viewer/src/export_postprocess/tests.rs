use super::test_support::{FixtureArtifacts, StaticPostprocessAdapter};
use super::*;
use std::error::Error;

const MINIMUM_ACCEPTED_REDUCTION_PERCENT_X100: i64 = 500;
const SLOW_POSTPROCESS_MILLIS: u128 = 31_000;

#[test]
fn disabled_mode_keeps_pdf_bytes_and_records_diagnostic() -> Result<(), Box<dyn Error>> {
    let fixture = FixtureArtifacts::new()?;
    let adapter = StaticPostprocessAdapter::success("karui", fixture.optimized_pdf());
    let service = ExportPostprocessEvaluationService::new(
        adapter,
        ExportPostprocessMode::Disabled,
        ExportPostprocessPolicy::v0_1_3(),
    );

    let report = service.evaluate(&fixture.request());

    assert_eq!(report.status, ExportPostprocessStatus::Skipped);
    assert_eq!(report.selected_pdf_bytes, fixture.pdf);
    assert_diagnostic(&report, "postprocess-disabled");
    Ok(())
}

#[test]
fn adapter_failure_keeps_pdf_bytes_and_records_reason() -> Result<(), Box<dyn Error>> {
    let fixture = FixtureArtifacts::new()?;
    let service = ExportPostprocessEvaluationService::new(
        StaticPostprocessAdapter::failure("karui", "engine unavailable"),
        ExportPostprocessMode::Enabled,
        ExportPostprocessPolicy::v0_1_3(),
    );

    let report = service.evaluate(&fixture.request());

    assert_eq!(report.status, ExportPostprocessStatus::Rejected);
    assert_eq!(report.selected_pdf_bytes, fixture.pdf);
    assert_diagnostic(&report, "postprocess-failed");
    Ok(())
}

#[test]
fn quality_regression_rejects_optimized_pdf() -> Result<(), Box<dyn Error>> {
    let fixture = FixtureArtifacts::new()?;
    let degraded_pdf = fixture.pdf_without_link_annotation();
    let service = ExportPostprocessEvaluationService::new(
        StaticPostprocessAdapter::success("karui", degraded_pdf),
        ExportPostprocessMode::Enabled,
        ExportPostprocessPolicy::v0_1_3(),
    );

    let report = service.evaluate(&fixture.request());

    assert_eq!(report.status, ExportPostprocessStatus::Rejected);
    assert_eq!(report.selected_pdf_bytes, fixture.pdf);
    assert_diagnostic(&report, "postprocess-quality-regressed");
    assert_diagnostic(&report, "postprocess-link-annotation-regressed");
    Ok(())
}

#[test]
fn useful_optimized_pdf_is_accepted_with_comparison_metrics() -> Result<(), Box<dyn Error>> {
    let fixture = FixtureArtifacts::with_test_metadata()?;
    let service = ExportPostprocessEvaluationService::new(
        StaticPostprocessAdapter::success("karui", FixtureArtifacts::base_pdf()),
        ExportPostprocessMode::Enabled,
        ExportPostprocessPolicy::v0_1_3(),
    );

    let report = service.evaluate(&fixture.request());

    assert_eq!(report.status, ExportPostprocessStatus::Accepted);
    assert_eq!(report.selected_pdf_bytes, FixtureArtifacts::base_pdf());
    assert!(report.metrics.size_reduction_percent_x100 >= MINIMUM_ACCEPTED_REDUCTION_PERCENT_X100);
    assert!(report.optimized_quality.is_pass());
    Ok(())
}

#[test]
fn small_size_reduction_is_not_enough_for_adoption() -> Result<(), Box<dyn Error>> {
    let fixture = FixtureArtifacts::new()?;
    let service = ExportPostprocessEvaluationService::new(
        StaticPostprocessAdapter::success("karui", fixture.pdf_with_one_byte_removed()),
        ExportPostprocessMode::Enabled,
        ExportPostprocessPolicy::v0_1_3(),
    );

    let report = service.evaluate(&fixture.request());

    assert_eq!(report.status, ExportPostprocessStatus::Rejected);
    assert_diagnostic(&report, "postprocess-size-reduction-too-small");
    Ok(())
}

#[test]
fn slow_postprocess_is_not_enough_for_adoption() -> Result<(), Box<dyn Error>> {
    let fixture = FixtureArtifacts::with_test_metadata()?;
    let adapter = StaticPostprocessAdapter::success("karui", FixtureArtifacts::base_pdf())
        .with_elapsed_millis(SLOW_POSTPROCESS_MILLIS);
    let service = ExportPostprocessEvaluationService::new(
        adapter,
        ExportPostprocessMode::Enabled,
        ExportPostprocessPolicy::v0_1_3(),
    );

    let report = service.evaluate(&fixture.request());

    assert_eq!(report.status, ExportPostprocessStatus::Rejected);
    assert_diagnostic(&report, "postprocess-too-slow");
    Ok(())
}

#[test]
fn karui_adapter_reports_public_engine_is_unavailable() {
    let adapter = KaruiPdfPostprocessAdapter;
    let result = adapter.postprocess_pdf(&PdfPostprocessInput {
        pdf: FixtureArtifacts::base_pdf().as_slice(),
    });

    assert_eq!(adapter.name(), "karui");
    assert!(result.is_err());
}

fn assert_diagnostic(report: &ExportPostprocessEvaluationReport, code: &str) {
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == code),
        "missing diagnostic {code}: {report:#?}"
    );
}
