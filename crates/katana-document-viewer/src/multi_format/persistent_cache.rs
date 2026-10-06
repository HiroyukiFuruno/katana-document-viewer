use std::path::{Path, PathBuf};
use thiserror::Error;

#[path = "persistent_cache_key.rs"]
mod key;
#[path = "persistent_cache_payload_codec.rs"]
mod payload_codec;
#[path = "persistent_cache_sessions.rs"]
mod sessions;
#[path = "persistent_cache_store.rs"]
mod store;
pub use sessions::{PersistentOfficeViewerSession, PersistentPdfViewerSession};

#[derive(Debug, Error)]
pub enum PersistentCacheError {
    #[error("persistent artifact I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("persistent artifact is corrupt or incompatible")]
    Corrupt,
    #[error("persistent artifact exceeds the configured capacity")]
    Capacity,
    #[error("persistent cache environment revision must not be empty")]
    InvalidEnvironmentRevision,
    #[error("persistent cache directory must be private and contain regular files only")]
    UnsafeDirectory,
    #[error(transparent)]
    Office(#[from] super::OfficeWorkerError),
    #[error(transparent)]
    Pdf(#[from] super::PdfViewerError),
}

/// 保存先と削除・機密入力の保存可否はhostが所有する。専用directoryは他用途と共有しない。
#[derive(Debug, Clone)]
pub struct PersistentDocumentCache {
    root: PathBuf,
    max_bytes: u64,
    environment_revision: String,
    engine_revision: String,
}

#[cfg(test)]
#[path = "persistent_cache_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "persistent_cache_capacity_tests.rs"]
mod capacity_tests;

#[cfg(test)]
#[path = "persistent_cache_budget_tests.rs"]
mod budget_tests;

#[cfg(test)]
#[path = "persistent_cache_engine_tests.rs"]
mod engine_tests;

impl PersistentDocumentCache {
    /// 実行imageの指紋はprocess内で共有し、静的リンクされた描画依存の更新を識別する。
    /// environment_revisionはfont・設定・外部dynamic moduleの変更時にhostが更新する。
    pub fn new(
        root: PathBuf,
        max_bytes: u64,
        environment_revision: String,
    ) -> Result<Self, PersistentCacheError> {
        if max_bytes == 0 {
            return Err(PersistentCacheError::Capacity);
        }
        if environment_revision.is_empty() {
            return Err(PersistentCacheError::InvalidEnvironmentRevision);
        }
        let cache = Self {
            root,
            max_bytes,
            environment_revision,
            engine_revision: key::CacheKey::engine_revision()?,
        };
        cache.prepare()?;
        Ok(cache)
    }

    #[must_use]
    pub fn directory(&self) -> &Path {
        &self.root
    }
}
