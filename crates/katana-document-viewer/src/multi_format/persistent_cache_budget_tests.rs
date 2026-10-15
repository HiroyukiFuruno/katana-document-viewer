use super::key::CacheKey;
use super::payload_codec::{MAX_METADATA_BYTES, MAX_PAYLOAD_BYTES, PayloadCodec};
use super::{PersistentCacheError, PersistentDocumentCache};
use std::fs::File;
use std::io::Write;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn codec_accepts_exact_metadata_budget_and_rejects_one_byte_over() -> TestResult {
    let exact = serde_json::json!("x".repeat(MAX_METADATA_BYTES - 2));
    let encoded = PayloadCodec::encode(&exact, &[])?;
    assert_eq!(MAX_METADATA_BYTES, encoded.len() - 8);
    let (decoded, payload): (serde_json::Value, &[u8]) = PayloadCodec::decode(&encoded)?;
    assert_eq!(exact, decoded);
    assert!(payload.is_empty());

    let over = serde_json::json!("x".repeat(MAX_METADATA_BYTES - 1));
    assert!(matches!(
        PayloadCodec::encode(&over, &[]),
        Err(PersistentCacheError::Capacity)
    ));
    Ok(())
}

#[test]
fn codec_rejects_payload_over_the_logical_budget() {
    let payload = vec![0_u8; MAX_PAYLOAD_BYTES + 1];
    assert!(matches!(
        PayloadCodec::encode(&serde_json::json!({}), &payload),
        Err(PersistentCacheError::Capacity)
    ));
}

#[test]
fn codec_decode_rejects_metadata_and_payload_over_budget() {
    let mut metadata = (MAX_METADATA_BYTES as u64 + 1).to_le_bytes().to_vec();
    metadata.extend(std::iter::repeat_n(b' ', MAX_METADATA_BYTES + 1));
    assert!(matches!(
        PayloadCodec::decode::<serde_json::Value>(&metadata),
        Err(PersistentCacheError::Capacity)
    ));

    let mut payload = 2_u64.to_le_bytes().to_vec();
    payload.extend_from_slice(b"{}");
    payload.extend(std::iter::repeat_n(0_u8, MAX_PAYLOAD_BYTES + 1));
    assert!(matches!(
        PayloadCodec::decode::<serde_json::Value>(&payload),
        Err(PersistentCacheError::Capacity)
    ));
}

#[test]
fn unknown_raw_entry_over_payload_budget_is_capacity() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        MAX_PAYLOAD_BYTES as u64 + 1024,
        "budget-v1".into(),
    )?;
    let path = cache.directory().join("unknown");
    let file = File::create(path)?;
    file.set_len(MAX_PAYLOAD_BYTES as u64 + 72 + 1)?;
    file.sync_all()?;
    assert!(matches!(
        cache.load("unknown"),
        Err(PersistentCacheError::Capacity)
    ));
    Ok(())
}

#[test]
fn save_rejects_unframed_payload_over_legacy_entry_limit() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        MAX_PAYLOAD_BYTES as u64 + MAX_METADATA_BYTES as u64 + 1024,
        "budget-v1".into(),
    )?;
    let payload = vec![0_u8; MAX_PAYLOAD_BYTES + 1];
    for length in [MAX_PAYLOAD_BYTES, MAX_PAYLOAD_BYTES + 1] {
        assert!(matches!(
            cache.save("unframed", &payload[..length]),
            Err(PersistentCacheError::Capacity)
        ));
    }
    Ok(())
}

#[test]
fn framed_invalid_large_entry_is_capacity() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        MAX_PAYLOAD_BYTES as u64 + MAX_METADATA_BYTES as u64 + 1024,
        "budget-v1".into(),
    )?;
    let key = "large";
    let payload = vec![0_u8; MAX_PAYLOAD_BYTES + 1];
    let mut file = File::create(cache.directory().join(key))?;
    file.write_all(b"KDVC0001")?;
    file.write_all(CacheKey::artifact_digest(key, &payload).as_bytes())?;
    file.write_all(&payload)?;
    file.sync_all()?;
    assert!(matches!(
        cache.load(key),
        Err(PersistentCacheError::Capacity)
    ));

    Ok(())
}

#[test]
fn framed_large_entry_with_bad_checksum_is_corrupt() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        MAX_PAYLOAD_BYTES as u64 + MAX_METADATA_BYTES as u64 + 1024,
        "budget-v1".into(),
    )?;
    let key = "large";
    let mut file = File::create(cache.directory().join(key))?;
    file.write_all(b"KDVC0001")?;
    file.write_all(CacheKey::artifact_digest(key, b"wrong").as_bytes())?;
    file.set_len(72 + MAX_PAYLOAD_BYTES as u64 + 1)?;
    file.sync_all()?;
    assert!(matches!(
        cache.load(key),
        Err(PersistentCacheError::Corrupt)
    ));
    Ok(())
}

#[test]
fn framed_entry_over_physical_budget_is_capacity() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        MAX_PAYLOAD_BYTES as u64 + MAX_METADATA_BYTES as u64 + 1024,
        "budget-v1".into(),
    )?;
    let path = cache.directory().join("over");
    let mut file = File::create(path)?;
    file.write_all(b"KDVC0001")?;
    file.set_len(MAX_PAYLOAD_BYTES as u64 + MAX_METADATA_BYTES as u64 + 8 + 72 + 1)?;
    file.sync_all()?;
    assert!(matches!(
        cache.load("over"),
        Err(PersistentCacheError::Capacity)
    ));
    Ok(())
}

#[test]
fn save_rejects_an_entry_one_byte_over_the_physical_budget() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        2 * MAX_PAYLOAD_BYTES as u64,
        "budget-v1".into(),
    )?;
    let payload = vec![0_u8; MAX_PAYLOAD_BYTES + MAX_METADATA_BYTES + 8 + 1];
    assert!(matches!(
        cache.save("oversized", &payload),
        Err(PersistentCacheError::Capacity)
    ));
    assert_eq!(None, cache.load("oversized")?);
    Ok(())
}
