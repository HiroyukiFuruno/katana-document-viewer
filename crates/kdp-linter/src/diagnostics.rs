use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Violation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub rule: &'static str,
    pub message: String,
}

impl Violation {
    pub fn new(
        file: PathBuf,
        line: usize,
        column: usize,
        rule: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            file,
            line,
            column,
            rule,
            message: message.into(),
        }
    }
}

pub struct ViolationReport;

impl ViolationReport {
    pub fn format(violations: &[Violation]) -> String {
        let mut report = String::from("\n[AST lint]\n");
        for violation in violations {
            report.push_str(&format!(
                "{}:{}:{} [{}] {}\n",
                violation.file.display(),
                violation.line,
                violation.column,
                violation.rule,
                violation.message
            ));
        }
        report
    }
}

#[derive(Debug, Error)]
pub enum KdpLintError {
    #[error("failed to read {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse Rust syntax in {path}:{line}:{column}: {message}")]
    RustParse {
        path: PathBuf,
        line: usize,
        column: usize,
        message: String,
    },
    #[error("failed to parse TOML in {path}: {source}")]
    TomlParse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("workspace root could not be resolved from {path}")]
    WorkspaceRoot { path: PathBuf },
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LINE: usize = 12;
    const TEST_COLUMN: usize = 4;

    #[test]
    fn violation_new_populates_fields() {
        let violation = Violation::new(
            PathBuf::from("src/lib.rs"),
            TEST_LINE,
            TEST_COLUMN,
            "rule-id",
            "message",
        );

        assert_eq!(violation.file, PathBuf::from("src/lib.rs"));
        assert_eq!(violation.line, TEST_LINE);
        assert_eq!(violation.column, TEST_COLUMN);
        assert_eq!(violation.rule, "rule-id");
        assert_eq!(violation.message, "message");
    }

    #[test]
    fn violation_report_formats_all_fields() {
        let violation = Violation::new(
            PathBuf::from("src/lib.rs"),
            TEST_LINE,
            TEST_COLUMN,
            "rule-id",
            "message",
        );

        let report = ViolationReport::format(&[violation]);

        assert!(report.contains("[AST lint]"));
        assert!(report.contains("src/lib.rs:12:4 [rule-id] message"));
    }

    #[test]
    fn workspace_root_error_includes_path() {
        let error = KdpLintError::WorkspaceRoot {
            path: PathBuf::from("missing"),
        };

        assert_eq!(
            error.to_string(),
            "workspace root could not be resolved from missing"
        );
    }
}
