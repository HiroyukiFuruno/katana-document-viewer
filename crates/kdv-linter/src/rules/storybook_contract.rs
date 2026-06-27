use crate::diagnostics::{KdvLintError, Violation};
use crate::workspace::{SourceFile, WorkspaceModel};
use std::path::PathBuf;

#[path = "storybook_contract_adapter.rs"]
mod adapter;
#[path = "storybook_contract_patterns.rs"]
mod patterns;
#[path = "storybook_contract_window_presentation.rs"]
mod window_presentation;

use patterns::StorybookForbiddenPattern;

pub struct StorybookContractRule;

impl StorybookContractRule {
    pub fn check(workspace: &WorkspaceModel) -> Result<Vec<Violation>, KdvLintError> {
        let mut violations = Vec::new();
        for file in workspace.storybook_files() {
            violations.extend(StorybookSourceChecker::new(file).violations());
            violations
                .extend(window_presentation::WindowPresentationChecker::new(file).violations());
        }
        for file in workspace.rust_files() {
            violations.extend(adapter::AdapterContractChecker::new(file).violations());
        }
        Ok(violations)
    }
}

struct StorybookSourceChecker<'a> {
    file: &'a SourceFile,
}

impl<'a> StorybookSourceChecker<'a> {
    fn new(file: &'a SourceFile) -> Self {
        Self { file }
    }

    fn violations(&self) -> Vec<Violation> {
        let mut violations = Vec::new();
        for pattern in StorybookForbiddenPattern::all() {
            violations.extend(self.find_pattern(*pattern));
        }
        violations
    }

    fn find_pattern(&self, pattern: StorybookForbiddenPattern) -> Vec<Violation> {
        self.file
            .source()
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                line.find(pattern.needle()).map(|column| {
                    Violation::new(
                        PathBuf::from(self.file.path()),
                        index + 1,
                        column + 1,
                        pattern.rule(),
                        pattern.message(),
                    )
                })
            })
            .collect()
    }
}

#[cfg(test)]
#[path = "storybook_contract_tests.rs"]
mod tests;
