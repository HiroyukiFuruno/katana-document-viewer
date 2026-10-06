use super::PersistentDocumentCache;
use crate::multi_format::{BinaryDocumentSource, OfficeDocumentSource, OfficeWorkerConfig};
use sha2::{Digest, Sha256};
use std::io::Read;

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
        let mut file = std::fs::File::open(&config.executable)?;
        let mut digest = Sha256::new();
        let mut buffer = [0_u8; 65536];
        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            digest.update(&buffer[..count]);
        }
        let settings = format!(
            "{:?}:{config:?}:{}",
            (&source.identity, source.format, &source.mime),
            Self::hex(&digest.finalize())
        );
        Ok(Self::document(cache, &source.bytes, &settings))
    }

    fn document(cache: &PersistentDocumentCache, bytes: &[u8], settings: &str) -> String {
        let metadata = format!(
            "kdv-artifact-v1:{}:office2pdf-0.8.1:hayro-0.7.1:{}:{settings}",
            env!("CARGO_PKG_VERSION"),
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
