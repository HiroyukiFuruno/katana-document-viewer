use crate::diagnostics::{KdvLintError, Violation};
use crate::workspace::{SourceFile, WorkspaceModel};
use std::path::{Path, PathBuf};

pub struct KdvUiAdapterOwnershipRule;

impl KdvUiAdapterOwnershipRule {
    pub fn check(workspace: &WorkspaceModel) -> Result<Vec<Violation>, KdvLintError> {
        let mut violations = Vec::new();
        violations.extend(KdvOwnedAdapterCrateChecker::new(workspace.root()).violations());
        violations.extend(StorybookOwnedBridgeChecker::new(workspace.root()).violations());
        for file in workspace.storybook_files() {
            violations.extend(StorybookAdapterChecker::new(file).violations());
        }
        Ok(violations)
    }
}

struct KdvOwnedAdapterCrateChecker<'a> {
    root: &'a Path,
}

impl<'a> KdvOwnedAdapterCrateChecker<'a> {
    fn new(root: &'a Path) -> Self {
        Self { root }
    }

    fn violations(&self) -> Vec<Violation> {
        let manifest = self.manifest_path();
        if !manifest.exists() {
            return Vec::new();
        }
        vec![Violation::new(
            manifest,
            1,
            1,
            "no_kdv_ui_adapter_ownership",
            "KDV repo must not own a KUC UI adapter crate; move viewer projection/host contract to KUC.",
        )]
    }

    fn manifest_path(&self) -> PathBuf {
        self.root
            .join("crates")
            .join("katana-document-viewer-kuc")
            .join("Cargo.toml")
    }
}

struct StorybookOwnedBridgeChecker<'a> {
    root: &'a Path,
}

impl<'a> StorybookOwnedBridgeChecker<'a> {
    fn new(root: &'a Path) -> Self {
        Self { root }
    }

    fn violations(&self) -> Vec<Violation> {
        let module_path = self.module_path();
        if !module_path.exists() {
            return Vec::new();
        }
        vec![Violation::new(
            module_path,
            1,
            1,
            "no_kdv_ui_adapter_ownership",
            "KDV Storybook must not own a KUC viewer bridge module; move viewer projection/host contract to KUC.",
        )]
    }

    fn module_path(&self) -> PathBuf {
        self.root
            .join("tools")
            .join("kdv-storybook")
            .join("src")
            .join("kuc_bridge")
            .join("mod.rs")
    }
}

struct StorybookAdapterChecker<'a> {
    file: &'a SourceFile,
}

impl<'a> StorybookAdapterChecker<'a> {
    fn new(file: &'a SourceFile) -> Self {
        Self { file }
    }

    fn violations(&self) -> Vec<Violation> {
        if self.is_test_only_file() {
            return Vec::new();
        }
        let mut violations = Vec::new();
        for pattern in StorybookAdapterPattern::all() {
            violations.extend(self.find_pattern(*pattern));
        }
        violations
    }

    fn is_test_only_file(&self) -> bool {
        let path = self.file.path().to_string_lossy();
        path.contains("/tests/") || path.ends_with("_tests.rs") || path.ends_with("test_support.rs")
    }

    fn find_pattern(&self, pattern: StorybookAdapterPattern) -> Vec<Violation> {
        self.file
            .source()
            .lines()
            .enumerate()
            .filter_map(|(index, line)| self.violation_for_line(pattern, index, line))
            .collect()
    }

    fn violation_for_line(
        &self,
        pattern: StorybookAdapterPattern,
        index: usize,
        line: &str,
    ) -> Option<Violation> {
        line.find(pattern.needle()).map(|column| {
            Violation::new(
                PathBuf::from(self.file.path()),
                index + 1,
                column + 1,
                "no_kdv_ui_adapter_ownership",
                pattern.message(),
            )
        })
    }
}

#[derive(Clone, Copy)]
enum StorybookAdapterPattern {
    KdvKucCrate,
    KucStorybookHost,
    KucRenderedInteractionSurface,
    KucHostActionHitQuery,
    KucCanvasRenderer,
    KucHostActionHitRects,
    StorybookKucRendererModule,
    StorybookKucRendererFunction,
    KucBridgeModule,
}

impl StorybookAdapterPattern {
    fn all() -> &'static [Self] {
        &[
            Self::KdvKucCrate,
            Self::KucStorybookHost,
            Self::KucRenderedInteractionSurface,
            Self::KucHostActionHitQuery,
            Self::KucCanvasRenderer,
            Self::KucHostActionHitRects,
            Self::StorybookKucRendererModule,
            Self::StorybookKucRendererFunction,
            Self::KucBridgeModule,
        ]
    }

    fn needle(self) -> &'static str {
        match self {
            Self::KdvKucCrate => "katana_document_viewer_kuc",
            Self::KucStorybookHost => "UiTreeStorybookHost",
            Self::KucRenderedInteractionSurface => "UiTreeInteractionSurface",
            Self::KucHostActionHitQuery => "UiTreeHostActionHitQuery",
            Self::KucCanvasRenderer => "UiTreeCanvasRenderer",
            Self::KucHostActionHitRects => "host_action_hit_rects",
            Self::StorybookKucRendererModule => "frame_kuc_renderer",
            Self::StorybookKucRendererFunction => "kuc_tree_host_action_hits_at",
            Self::KucBridgeModule => "kuc_bridge",
        }
    }

    fn message(self) -> &'static str {
        match self {
            Self::KdvKucCrate => "Storybook must not depend on a KDV-owned KUC adapter crate.",
            Self::KucStorybookHost
            | Self::KucRenderedInteractionSurface
            | Self::KucHostActionHitQuery
            | Self::KucCanvasRenderer
            | Self::KucHostActionHitRects
            | Self::StorybookKucRendererModule
            | Self::StorybookKucRendererFunction
            | Self::KucBridgeModule => {
                "Storybook must not wrap KUC renderer/hit-test internals; use a KUC-owned host contract."
            }
        }
    }
}

#[cfg(test)]
#[path = "kdv_ui_adapter_ownership_tests.rs"]
mod tests;
