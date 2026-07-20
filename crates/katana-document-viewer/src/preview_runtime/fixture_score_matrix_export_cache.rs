use super::{ExportBytes, PreviewOutput};
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

#[derive(Clone, Hash, PartialEq, Eq)]
pub(super) struct ExportCacheKey {
    pub(super) document_id: String,
    pub(super) revision: String,
    pub(super) kind: String,
    pub(super) source_path: String,
}

impl ExportCacheKey {
    pub(super) fn from_output(output: &PreviewOutput) -> Self {
        let snapshot = &output.input.snapshot;
        Self {
            document_id: snapshot.id.0.clone(),
            revision: snapshot.revision.0.clone(),
            kind: format!("{:?}", snapshot.kind),
            source_path: snapshot.source_path.to_string_lossy().to_string(),
        }
    }
}

pub(super) fn export_cache() -> &'static Mutex<HashMap<ExportCacheKey, ExportBytes>> {
    static CACHE: OnceLock<Mutex<HashMap<ExportCacheKey, ExportBytes>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(super) fn export_cache_lock() -> MutexGuard<'static, HashMap<ExportCacheKey, ExportBytes>> {
    match export_cache().lock() {
        Ok(cache) => cache,
        Err(poisoned) => poisoned.into_inner(),
    }
}
