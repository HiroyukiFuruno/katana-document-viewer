use crate::export_quality::surface_equivalence::SurfaceEquivalenceArtifacts;
use crate::forge::ExportFormat;

pub(super) const FORMAT_SCORE_MAX: u16 = 100;
pub(super) const TOTAL_SCORE_MAX: u16 = FORMAT_SCORE_MAX * 4;
pub(super) const SURFACE_PASS_SCORE: u16 = 95;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportQualityArtifacts<'a> {
    pub html: &'a [u8],
    pub pdf: &'a [u8],
    pub png: &'a [u8],
    pub jpeg: &'a [u8],
    pub source_markdown: &'a str,
    pub surface_equivalence: Option<SurfaceEquivalenceArtifacts<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportQualityReport {
    pub total_score: u16,
    pub max_score: u16,
    pub format_scores: Vec<ExportFormatQualityScore>,
    pub fatal_failures: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportFormatQualityScore {
    pub format: ExportFormat,
    pub score: u16,
    pub max_score: u16,
    pub checks: Vec<ExportQualityCheck>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportQualityCheck {
    pub name: String,
    pub passed: bool,
    pub fatal: bool,
    pub points: u16,
}

impl ExportQualityReport {
    pub fn is_pass(&self) -> bool {
        self.total_score == self.max_score && self.fatal_failures.is_empty()
    }
}

impl ExportFormatQualityScore {
    pub(super) fn new(format: ExportFormat, checks: Vec<ExportQualityCheck>) -> Self {
        let score = checks
            .iter()
            .filter(|check| check.passed)
            .map(|check| check.points)
            .sum();
        Self {
            format,
            score,
            max_score: FORMAT_SCORE_MAX,
            checks,
        }
    }

    pub(super) fn is_pass(&self) -> bool {
        match self.format {
            ExportFormat::Html => self.score == FORMAT_SCORE_MAX,
            ExportFormat::Pdf | ExportFormat::Png | ExportFormat::Jpeg => {
                self.score >= SURFACE_PASS_SCORE
            }
        }
    }

    pub(super) fn fatal_failures(&self) -> Vec<String> {
        self.checks
            .iter()
            .filter(|check| check.fatal && !check.passed)
            .map(|check| format!("{:?}: {}", self.format, check.name))
            .collect()
    }
}

pub(super) fn check(name: &str, passed: bool, fatal: bool, points: u16) -> ExportQualityCheck {
    ExportQualityCheck {
        name: name.to_string(),
        passed,
        fatal,
        points,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fatal_failures_list_failed_fatal_checks() {
        let score = ExportFormatQualityScore::new(
            ExportFormat::Html,
            vec![check("required semantic", false, true, 20)],
        );

        assert_eq!(
            score.fatal_failures(),
            vec!["Html: required semantic".to_string()]
        );
    }
}
