#![deny(
    warnings,
    clippy::all,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::dbg_macro
)]

pub mod diagnostics;
pub mod rules;
pub mod span;
pub mod syntax;
pub mod workspace;

pub use diagnostics::{KdpLintError, Violation, ViolationReport};

use std::path::Path;

pub struct KdpLinter;

impl KdpLinter {
    pub fn lint_workspace(root: &Path) -> Result<Vec<Violation>, KdpLintError> {
        let workspace = workspace::WorkspaceModel::load(root)?;
        rules::RuleRunner::check(&workspace)
    }
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
