#[cfg(test)]
use crate::{diagnostics::KdpLintError, workspace::WorkspaceModel};
#[cfg(test)]
use std::{
    fs,
    path::Path,
    path::PathBuf,
    process::id,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(test)]
const CRATE_DIRS: [&str; 5] = [
    "crates/kdp-linter/src",
    "crates/kdp-linter/tests",
    "crates/katana-document-viewer/src",
    "crates/katana-document-preview/src",
    "crates/katana-document-preview-egui/src",
];

#[cfg(test)]
pub(crate) fn unique_fixture_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);
    let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let process_id = id();
    std::env::temp_dir().join(format!(
        "kdp-linter-rule-fixture-{process_id}-{nanos}-{sequence}"
    ))
}

#[cfg(test)]
fn ensure_directory(path: &Path) -> Result<(), KdpLintError> {
    fs::create_dir_all(path).map_err(|source| KdpLintError::Read {
        path: path.to_path_buf(),
        source,
    })
}

#[cfg(test)]
fn write_file(path: &Path, source: &str) -> Result<(), KdpLintError> {
    if let Some(parent) = path.parent() {
        ensure_directory(parent)?;
    }
    fs::write(path, source).map_err(|source| KdpLintError::Read {
        path: path.to_path_buf(),
        source,
    })
}

#[cfg(test)]
pub(crate) struct FixtureWorkspace {
    pub(crate) root: PathBuf,
}

#[cfg(test)]
impl FixtureWorkspace {
    pub(crate) fn new() -> Self {
        Self {
            root: unique_fixture_root(),
        }
    }

    pub(crate) fn with_default_manifests(self) -> Result<Self, KdpLintError> {
        ensure_directory(&self.root)?;
        for dir in &CRATE_DIRS {
            ensure_directory(&self.root.join(dir))?;
        }
        self.write_default_manifest("crates/katana-document-viewer/Cargo.toml")?;
        self.write_default_manifest("crates/katana-document-preview/Cargo.toml")?;
        self.write_default_manifest("crates/katana-document-preview-egui/Cargo.toml")?;
        Ok(self)
    }

    pub(crate) fn write_default_manifest(&self, relative_path: &str) -> Result<(), KdpLintError> {
        let path = self.root.join(relative_path);
        write_file(
            &path,
            r#"
[package]
name = "fixture"
version = "0.1.0"
edition = "2021"
"#,
        )
    }

    pub(crate) fn write_manifest(
        &self,
        relative_path: &str,
        content: &str,
    ) -> Result<(), KdpLintError> {
        write_file(&self.root.join(relative_path), content)
    }

    pub(crate) fn write_rust_file(
        &self,
        relative_path: &str,
        source: &str,
    ) -> Result<(), KdpLintError> {
        write_file(&self.root.join(relative_path), source)
    }

    pub(crate) fn workspace(&self) -> Result<WorkspaceModel, KdpLintError> {
        WorkspaceModel::load(&self.root)
    }
}

#[cfg(test)]
impl Drop for FixtureWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[cfg(test)]
#[cfg(test)]
#[path = "test_helpers_tests.rs"]
mod tests;
