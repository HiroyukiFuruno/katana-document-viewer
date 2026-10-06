use super::ConversionArtifact;
use crate::multi_format::office_worker_parent::OfficeWorkerOutput;
use crate::multi_format::persistent_cache::key::CacheKey;
use crate::multi_format::persistent_cache::payload_codec::MAX_PAYLOAD_BYTES;
use crate::multi_format::{
    OfficeDocumentFormat, OfficeDocumentSource, OfficeWorkerConfig, PersistentCacheError,
    PersistentDocumentCache, ViewerSourceIdentity,
};
use serde_json::json;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn large_pdf_conversion_round_trips_within_the_cache_capacity() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        MAX_PAYLOAD_BYTES as u64 + 2 * 1024 * 1024,
        "conversion-binary-v1".into(),
    )?;
    let source = source();
    let config = OfficeWorkerConfig::new(std::env::current_exe()?);
    assert_eq!(MAX_PAYLOAD_BYTES as u64, config.max_output_bytes);
    let key = CacheKey::office(&cache, &source, &config)?;
    let output = OfficeWorkerOutput {
        pdf: vec![0x25; MAX_PAYLOAD_BYTES],
        warnings: Vec::new(),
        preflight_diagnostics: Vec::new(),
    };

    cache.save(&key, &ConversionArtifact::encode(&output)?)?;
    let (loaded, cache_hit) = ConversionArtifact::output(&cache, &key, &source, &config)?;
    assert!(cache_hit);
    assert_eq!(output.pdf, loaded.pdf);
    Ok(())
}

#[test]
fn sixty_four_megabyte_pdf_conversion_still_round_trips() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        128 * 1024 * 1024,
        "conversion-binary-v1".into(),
    )?;
    let source = source();
    let mut config = OfficeWorkerConfig::new(std::env::current_exe()?);
    config.max_output_bytes = 64 * 1024 * 1024;
    let key = CacheKey::office(&cache, &source, &config)?;
    let output = OfficeWorkerOutput {
        pdf: vec![0x25; 64 * 1024 * 1024],
        warnings: Vec::new(),
        preflight_diagnostics: Vec::new(),
    };

    cache.save(&key, &ConversionArtifact::encode(&output)?)?;
    let (loaded, cache_hit) = ConversionArtifact::output(&cache, &key, &source, &config)?;
    assert!(cache_hit);
    assert_eq!(output.pdf, loaded.pdf);
    Ok(())
}

#[test]
fn malformed_conversion_binary_payloads_are_rejected() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(root.path().join("cache"), 1024, "codec-v1".into())?;
    let source = source();
    let config = OfficeWorkerConfig::new(std::env::current_exe()?);
    let key = CacheKey::office(&cache, &source, &config)?;
    let output = OfficeWorkerOutput {
        pdf: vec![0x25; 16],
        warnings: Vec::new(),
        preflight_diagnostics: Vec::new(),
    };
    let encoded = ConversionArtifact::encode(&output)?;
    assert_corrupt(&cache, &key, &source, &config, &encoded[..7])?;

    let mut bad_length = encoded.clone();
    bad_length[..8].copy_from_slice(&u64::MAX.to_le_bytes());
    assert_corrupt(&cache, &key, &source, &config, &bad_length)?;

    let metadata_len = usize::try_from(u64::from_le_bytes(encoded[..8].try_into()?))?;
    let mut metadata: serde_json::Value = serde_json::from_slice(&encoded[8..8 + metadata_len])?;
    metadata["pdf"] = json!([37, 37, 37]);
    let metadata = serde_json::to_vec(&metadata)?;
    let mut embedded = (metadata.len() as u64).to_le_bytes().to_vec();
    embedded.extend_from_slice(&metadata);
    assert_corrupt(&cache, &key, &source, &config, &embedded)
}

fn assert_corrupt(
    cache: &PersistentDocumentCache,
    key: &str,
    source: &OfficeDocumentSource,
    config: &OfficeWorkerConfig,
    bytes: &[u8],
) -> TestResult {
    cache.clear()?;
    cache.save(key, bytes)?;
    assert!(matches!(
        ConversionArtifact::output(cache, key, source, config),
        Err(PersistentCacheError::Corrupt)
    ));
    Ok(())
}

fn source() -> OfficeDocumentSource {
    OfficeDocumentSource::new(
        ViewerSourceIdentity::new("fixture:conversion-binary", "sha256:binary"),
        OfficeDocumentFormat::Docx,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        include_bytes!("../../../../assets/fixtures/multi-format/representative.docx").to_vec(),
    )
}
