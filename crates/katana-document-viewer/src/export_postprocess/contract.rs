use super::ExportPostprocessDiagnostic;

const PAGE_TREE_CODE: &str = "postprocess-page-tree-regressed";
const PAGE_IMAGE_CODE: &str = "postprocess-page-image-regressed";
const LINK_ANNOTATION_CODE: &str = "postprocess-link-annotation-regressed";
const FOOTNOTE_DESTINATION_CODE: &str = "postprocess-footnote-destination-regressed";
const PAGE_TREE_LABEL: &str = "PDF page tree";
const PAGE_IMAGE_LABEL: &str = "native surface page image";
const LINK_ANNOTATION_LABEL: &str = "link annotation";
const FOOTNOTE_DESTINATION_LABEL: &str = "footnote destination";

pub(crate) struct PdfPostprocessContract;

impl PdfPostprocessContract {
    pub(crate) fn regression_diagnostics(
        baseline_pdf: &[u8],
        optimized_pdf: &[u8],
    ) -> Vec<ExportPostprocessDiagnostic> {
        let baseline = PdfFeatureCounts::from_pdf(baseline_pdf);
        let optimized = PdfFeatureCounts::from_pdf(optimized_pdf);
        let mut diagnostics = Vec::new();
        for regression in feature_regressions(baseline, optimized) {
            Self::push_regression(&mut diagnostics, &regression);
        }
        diagnostics
    }

    fn push_regression(
        diagnostics: &mut Vec<ExportPostprocessDiagnostic>,
        regression: &FeatureRegression,
    ) {
        if regression.optimized_count < regression.baseline_count {
            diagnostics.push(ExportPostprocessDiagnostic::new(
                regression.code,
                &regression.message(),
            ));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PdfFeatureCounts {
    page_trees: usize,
    page_images: usize,
    link_annotations: usize,
    footnote_destinations: usize,
}

struct FeatureRegression {
    code: &'static str,
    label: &'static str,
    baseline_count: usize,
    optimized_count: usize,
}

impl FeatureRegression {
    fn new(
        code: &'static str,
        label: &'static str,
        baseline_count: usize,
        optimized_count: usize,
    ) -> Self {
        Self {
            code,
            label,
            baseline_count,
            optimized_count,
        }
    }

    fn message(&self) -> String {
        format!(
            "{} count regressed from {} to {}",
            self.label, self.baseline_count, self.optimized_count
        )
    }
}

impl PdfFeatureCounts {
    fn from_pdf(bytes: &[u8]) -> Self {
        let text = String::from_utf8_lossy(bytes);
        Self {
            page_trees: text.matches("/Type /Pages").count(),
            page_images: text.matches("/Subtype /Image").count(),
            link_annotations: text.matches("/Subtype /Link").count(),
            footnote_destinations: text.matches("/Dest [").count(),
        }
    }
}

fn feature_regressions(
    baseline: PdfFeatureCounts,
    optimized: PdfFeatureCounts,
) -> Vec<FeatureRegression> {
    vec![
        page_tree_regression(baseline, optimized),
        page_image_regression(baseline, optimized),
        link_annotation_regression(baseline, optimized),
        footnote_destination_regression(baseline, optimized),
    ]
}

fn page_tree_regression(
    baseline: PdfFeatureCounts,
    optimized: PdfFeatureCounts,
) -> FeatureRegression {
    FeatureRegression::new(
        PAGE_TREE_CODE,
        PAGE_TREE_LABEL,
        baseline.page_trees,
        optimized.page_trees,
    )
}

fn page_image_regression(
    baseline: PdfFeatureCounts,
    optimized: PdfFeatureCounts,
) -> FeatureRegression {
    FeatureRegression::new(
        PAGE_IMAGE_CODE,
        PAGE_IMAGE_LABEL,
        baseline.page_images,
        optimized.page_images,
    )
}

fn link_annotation_regression(
    baseline: PdfFeatureCounts,
    optimized: PdfFeatureCounts,
) -> FeatureRegression {
    FeatureRegression::new(
        LINK_ANNOTATION_CODE,
        LINK_ANNOTATION_LABEL,
        baseline.link_annotations,
        optimized.link_annotations,
    )
}

fn footnote_destination_regression(
    baseline: PdfFeatureCounts,
    optimized: PdfFeatureCounts,
) -> FeatureRegression {
    FeatureRegression::new(
        FOOTNOTE_DESTINATION_CODE,
        FOOTNOTE_DESTINATION_LABEL,
        baseline.footnote_destinations,
        optimized.footnote_destinations,
    )
}
