use super::PersistentDocumentCache;
use crate::multi_format::{BinaryDocumentSource, OfficeDocumentSource, OfficeWorkerConfig};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static ENGINE_REVISION: OnceLock<String> = OnceLock::new();

pub(super) struct CacheKey;

#[cfg(test)]
#[path = "persistent_cache_key_tests.rs"]
mod tests;

impl CacheKey {
    pub(super) fn digest(bytes: &[u8]) -> String {
        Self::hex(&Sha256::digest(bytes))
    }

    pub(super) fn artifact_digest(key: &str, bytes: &[u8]) -> String {
        let mut digest = Sha256::new();
        digest.update(key.as_bytes());
        digest.update(bytes);
        Self::hex(&digest.finalize())
    }

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    pub(super) fn pdf(cache: &PersistentDocumentCache, source: &BinaryDocumentSource) -> String {
        Self::document(
            cache,
            &source.bytes,
            &format!("{:?}", (&source.identity, &source.mime)),
        )
    }

    pub(super) fn office(
        cache: &PersistentDocumentCache,
        source: &OfficeDocumentSource,
        config: &OfficeWorkerConfig,
    ) -> Result<String, std::io::Error> {
        let settings = format!(
            "{:?}:{config:?}:{}",
            (&source.identity, source.format, &source.mime),
            Self::executable_digest(&Self::resolve_executable(&config.executable)?)?
        );
        Ok(Self::document(cache, &source.bytes, &settings))
    }

    pub(super) fn resolve_executable(path: &Path) -> Result<PathBuf, std::io::Error> {
        if path.is_absolute() {
            return Ok(path.to_path_buf());
        }
        if path.components().count() > 1 {
            return std::env::current_dir().map(|directory| directory.join(path));
        }
        which::which(path).map_err(|error| std::io::Error::new(std::io::ErrorKind::NotFound, error))
    }

    pub(super) fn executable_digest(path: &Path) -> Result<String, std::io::Error> {
        let mut file = std::fs::File::open(path)?;
        let mut digest = Sha256::new();
        let mut buffer = [0_u8; 65536];
        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            digest.update(&buffer[..count]);
        }
        Ok(Self::hex(&digest.finalize()))
    }

    pub(super) fn engine_revision() -> Result<String, std::io::Error> {
        if let Some(revision) = ENGINE_REVISION.get() {
            return Ok(revision.clone());
        }
        // downstreamで再解決された描画依存も、実際にリンクされたimageから識別する。
        #[cfg(target_os = "linux")]
        let image = Path::new("/proc/self/exe").to_path_buf();
        #[cfg(not(target_os = "linux"))]
        let image = std::env::current_exe()?;
        let revision = Self::executable_digest(&image)?;
        let _ = ENGINE_REVISION.set(revision.clone());
        Ok(revision)
    }

    fn document(cache: &PersistentDocumentCache, bytes: &[u8], settings: &str) -> String {
        let metadata = format!(
            "kdv-artifact-v2:{}:{}:{}:{settings}",
            env!("CARGO_PKG_VERSION"),
            cache.engine_revision,
            cache.environment_revision,
        );
        let mut digest = Sha256::new();
        digest.update((metadata.len() as u64).to_le_bytes());
        digest.update(metadata.as_bytes());
        digest.update(bytes);
        Self::hex(&digest.finalize())
    }

    pub(super) fn page(document: &str, page: usize, scale: f32, limits: &str) -> String {
        Self::digest(format!("{document}:{page}:{}:{limits}", scale.to_bits()).as_bytes())
    }
}
