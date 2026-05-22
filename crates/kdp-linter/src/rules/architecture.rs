use crate::diagnostics::{KdpLintError, Violation};
use crate::workspace::WorkspaceModel;

use super::egui_duplication::EguiDuplicationRule;
use super::manifest_boundary::ManifestBoundaryRule;

pub const VIEWER_CRATE: &str = "crates/katana-document-viewer";
pub const LIB_CRATE: &str = "crates/katana-document-preview";
pub const EGUI_CRATE: &str = "crates/katana-document-preview-egui";

pub struct ArchitectureRule;

impl ArchitectureRule {
    pub fn check(workspace: &WorkspaceModel) -> Result<Vec<Violation>, KdpLintError> {
        let mut violations = Vec::new();
        violations.extend(ManifestBoundaryRule::check(workspace.root())?);
        violations.extend(EguiDuplicationRule::check(workspace));
        Ok(violations)
    }
}
