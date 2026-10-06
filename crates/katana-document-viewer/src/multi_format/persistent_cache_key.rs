use super::PersistentDocumentCache;
use crate::multi_format::{BinaryDocumentSource, OfficeDocumentSource, OfficeWorkerConfig};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;
use std::sync::OnceLock;

static ENGINE_REVISION: OnceLock<String> = OnceLock::new();

pub(super) struct CacheKey;

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
            Self::executable_digest(&config.executable)?
        );
        Ok(Self::document(cache, &source.bytes, &settings))
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
        let revision = Self::executable_digest(&std::env::current_exe()?)?;
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
