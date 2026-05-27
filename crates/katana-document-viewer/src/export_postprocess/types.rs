use crate::{ExportQualityArtifacts, ExportQualityReport};

const PERCENT_X100_DENOMINATOR: i128 = 10_000;
const NON_ZERO_DENOMINATOR_FLOOR: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportPostprocessMode {
    Disabled,
    Enabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportPostprocessStatus {
    Skipped,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportPostprocessDiagnostic {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportPostprocessMetrics {
    pub baseline_pdf_size_bytes: usize,
    pub optimized_pdf_size_bytes: usize,
    pub baseline_pdf_generation_millis: u128,
    pub postprocess_millis: u128,
    pub size_reduction_percent_x100: i64,
}

pub struct ExportPostprocessEvaluationRequest<'a> {
    pub artifacts: ExportQualityArtifacts<'a>,
    pub baseline_pdf_generation_millis: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportPostprocessEvaluationReport {
    pub adapter_name: String,
    pub status: ExportPostprocessStatus,
    pub selected_pdf_bytes: Vec<u8>,
    pub baseline_quality: ExportQualityReport,
    pub optimized_quality: ExportQualityReport,
    pub metrics: ExportPostprocessMetrics,
    pub diagnostics: Vec<ExportPostprocessDiagnostic>,
}

impl ExportPostprocessDiagnostic {
    pub(crate) fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
        }
    }
}

impl ExportPostprocessMetrics {
    pub(crate) fn new(
        baseline_pdf_size_bytes: usize,
        optimized_pdf_size_bytes: usize,
        baseline_pdf_generation_millis: u128,
        postprocess_millis: u128,
    ) -> Self {
        Self {
            baseline_pdf_size_bytes,
            optimized_pdf_size_bytes,
            baseline_pdf_generation_millis,
            postprocess_millis,
            size_reduction_percent_x100: size_reduction_percent_x100(
                baseline_pdf_size_bytes,
                optimized_pdf_size_bytes,
            ),
        }
    }
}

fn size_reduction_percent_x100(baseline: usize, optimized: usize) -> i64 {
    let numerator = baseline as i128 - optimized as i128;
    ((numerator * PERCENT_X100_DENOMINATOR) / baseline.max(NON_ZERO_DENOMINATOR_FLOOR) as i128)
        as i64
}
