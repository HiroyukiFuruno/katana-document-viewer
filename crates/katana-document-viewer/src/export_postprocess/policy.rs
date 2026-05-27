use super::{ExportPostprocessDiagnostic, ExportPostprocessMetrics};

const MINIMUM_SIZE_REDUCTION_PERCENT_X100: i64 = 500;
const MAXIMUM_POSTPROCESS_MILLIS: u128 = 30_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExportPostprocessPolicy {
    pub minimum_size_reduction_percent_x100: i64,
    pub maximum_postprocess_millis: u128,
}

impl ExportPostprocessPolicy {
    pub fn v0_1_3() -> Self {
        Self {
            minimum_size_reduction_percent_x100: MINIMUM_SIZE_REDUCTION_PERCENT_X100,
            maximum_postprocess_millis: MAXIMUM_POSTPROCESS_MILLIS,
        }
    }

    pub(crate) fn diagnostics(
        &self,
        metrics: &ExportPostprocessMetrics,
    ) -> Vec<ExportPostprocessDiagnostic> {
        let mut diagnostics = Vec::new();
        if metrics.size_reduction_percent_x100 < self.minimum_size_reduction_percent_x100 {
            diagnostics.push(ExportPostprocessDiagnostic::new(
                "postprocess-size-reduction-too-small",
                "optimized PDF size reduction is below the v0.1.3 adoption threshold",
            ));
        }
        if metrics.postprocess_millis > self.maximum_postprocess_millis {
            diagnostics.push(ExportPostprocessDiagnostic::new(
                "postprocess-too-slow",
                "PDF postprocess time is above the v0.1.3 adoption threshold",
            ));
        }
        diagnostics
    }
}
