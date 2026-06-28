use super::contract::PdfPostprocessContract;
use super::{
    ExportPostprocessDiagnostic, ExportPostprocessEvaluationReport,
    ExportPostprocessEvaluationRequest, ExportPostprocessMetrics, ExportPostprocessMode,
    ExportPostprocessPolicy, ExportPostprocessStatus, PdfPostprocessAdapter, PdfPostprocessError,
    PdfPostprocessInput, PdfPostprocessOutput,
};
use crate::{ExportQualityArtifacts, ExportQualityGate, ExportQualityReport};

pub struct ExportPostprocessEvaluationService<A> {
    adapter: A,
    mode: ExportPostprocessMode,
    policy: ExportPostprocessPolicy,
}

impl<A: PdfPostprocessAdapter> ExportPostprocessEvaluationService<A> {
    pub fn new(adapter: A, mode: ExportPostprocessMode, policy: ExportPostprocessPolicy) -> Self {
        Self {
            adapter,
            mode,
            policy,
        }
    }

    pub fn evaluate(
        &self,
        request: &ExportPostprocessEvaluationRequest<'_>,
    ) -> ExportPostprocessEvaluationReport {
        let baseline_quality = ExportQualityGate::evaluate(&request.artifacts);
        if self.mode == ExportPostprocessMode::Disabled {
            return self.skipped_report(request, baseline_quality);
        }
        let input = PdfPostprocessInput {
            pdf: request.artifacts.pdf,
        };
        match self.adapter.postprocess_pdf(&input) {
            Ok(output) => self.output_report(request, baseline_quality, output),
            Err(error) => self.failure_report(request, baseline_quality, error),
        }
    }

    fn skipped_report(
        &self,
        request: &ExportPostprocessEvaluationRequest<'_>,
        baseline_quality: ExportQualityReport,
    ) -> ExportPostprocessEvaluationReport {
        let metrics = self.metrics(request, request.artifacts.pdf, 0);
        self.report(
            ExportPostprocessStatus::Skipped,
            request.artifacts.pdf.to_vec(),
            baseline_quality.clone(),
            baseline_quality,
            metrics,
            vec![ExportPostprocessDiagnostic::new(
                "postprocess-disabled",
                "PDF postprocess is disabled by default",
            )],
        )
    }

    fn failure_report(
        &self,
        request: &ExportPostprocessEvaluationRequest<'_>,
        baseline_quality: ExportQualityReport,
        error: PdfPostprocessError,
    ) -> ExportPostprocessEvaluationReport {
        let metrics = self.metrics(request, request.artifacts.pdf, 0);
        self.report(
            ExportPostprocessStatus::Rejected,
            request.artifacts.pdf.to_vec(),
            baseline_quality.clone(),
            baseline_quality,
            metrics,
            vec![ExportPostprocessDiagnostic::new(
                "postprocess-failed",
                &error.message,
            )],
        )
    }

    fn output_report(
        &self,
        request: &ExportPostprocessEvaluationRequest<'_>,
        baseline_quality: ExportQualityReport,
        output: PdfPostprocessOutput,
    ) -> ExportPostprocessEvaluationReport {
        let optimized_quality = self.optimized_quality(request, &output.pdf);
        let metrics = self.metrics(request, &output.pdf, output.elapsed_millis);
        let diagnostics =
            self.output_diagnostics(request, &output.pdf, &optimized_quality, &metrics);
        let accepted = diagnostics.is_empty();
        let selected_pdf = if accepted {
            output.pdf
        } else {
            request.artifacts.pdf.to_vec()
        };
        self.report(
            status_for(accepted),
            selected_pdf,
            baseline_quality,
            optimized_quality,
            metrics,
            diagnostics,
        )
    }

    fn optimized_quality(
        &self,
        request: &ExportPostprocessEvaluationRequest<'_>,
        optimized_pdf: &[u8],
    ) -> ExportQualityReport {
        ExportQualityGate::evaluate(&ExportQualityArtifacts {
            html: request.artifacts.html,
            pdf: optimized_pdf,
            png: request.artifacts.png,
            jpeg: request.artifacts.jpeg,
            source_markdown: request.artifacts.source_markdown,
            surface_equivalence: request.artifacts.surface_equivalence,
        })
    }

    fn output_diagnostics(
        &self,
        request: &ExportPostprocessEvaluationRequest<'_>,
        optimized_pdf: &[u8],
        optimized_quality: &ExportQualityReport,
        metrics: &ExportPostprocessMetrics,
    ) -> Vec<ExportPostprocessDiagnostic> {
        let mut diagnostics = Vec::new();
        if !optimized_quality.is_pass() {
            diagnostics.push(ExportPostprocessDiagnostic::new(
                "postprocess-quality-regressed",
                "optimized PDF did not pass ExportQualityGate",
            ));
        }
        diagnostics.extend(PdfPostprocessContract::regression_diagnostics(
            request.artifacts.pdf,
            optimized_pdf,
        ));
        diagnostics.extend(self.policy.diagnostics(metrics));
        diagnostics
    }

    fn metrics(
        &self,
        request: &ExportPostprocessEvaluationRequest<'_>,
        optimized_pdf: &[u8],
        postprocess_millis: u128,
    ) -> ExportPostprocessMetrics {
        ExportPostprocessMetrics::new(
            request.artifacts.pdf.len(),
            optimized_pdf.len(),
            request.baseline_pdf_generation_millis,
            postprocess_millis,
        )
    }

    fn report(
        &self,
        status: ExportPostprocessStatus,
        selected_pdf_bytes: Vec<u8>,
        baseline_quality: ExportQualityReport,
        optimized_quality: ExportQualityReport,
        metrics: ExportPostprocessMetrics,
        diagnostics: Vec<ExportPostprocessDiagnostic>,
    ) -> ExportPostprocessEvaluationReport {
        ExportPostprocessEvaluationReport {
            adapter_name: self.adapter.name().to_string(),
            status,
            selected_pdf_bytes,
            baseline_quality,
            optimized_quality,
            metrics,
            diagnostics,
        }
    }
}

fn status_for(accepted: bool) -> ExportPostprocessStatus {
    if accepted {
        ExportPostprocessStatus::Accepted
    } else {
        ExportPostprocessStatus::Rejected
    }
}
