use super::{ConversionArtifact, PersistentOfficeViewerSession};
use crate::multi_format::OfficeStaticViewerSession;
use crate::multi_format::persistent_cache::key::CacheKey;
use crate::multi_format::{
    OfficeDocumentFormat, OfficeDocumentSource, OfficeWorkerConfig, PersistentCacheError,
    PersistentDocumentCache, ViewerSourceIdentity,
};
use std::path::PathBuf;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const DOCX: &[u8] = include_bytes!("../../../../assets/fixtures/multi-format/representative.docx");

fn source(format: OfficeDocumentFormat) -> OfficeDocumentSource {
    let mime = match format {
        OfficeDocumentFormat::Docx => {
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        }
        OfficeDocumentFormat::Xlsx => {
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        }
        OfficeDocumentFormat::Pptx => {
            "application/vnd.openxmlformats-officedocument.presentationml.presentation"
        }
    };
    OfficeDocumentSource::new(
        ViewerSourceIdentity::new("fixture:office", "test"),
        format,
        mime,
        DOCX.to_vec(),
    )
}

fn cache() -> Result<(tempfile::TempDir, PersistentDocumentCache), Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        128 * 1024 * 1024,
        "test-environment".into(),
    )?;
    Ok((root, cache))
}

#[test]
fn open_rejects_xlsx_before_touching_the_worker() -> Result<(), Box<dyn std::error::Error>> {
    let (_root, cache) = cache()?;
    let config = OfficeWorkerConfig::new(std::env::current_exe()?);

    assert!(matches!(
        PersistentOfficeViewerSession::open(source(OfficeDocumentFormat::Xlsx), config, cache),
        Err(PersistentCacheError::Office(
            crate::multi_format::OfficeWorkerError::UnsupportedFormat(OfficeDocumentFormat::Xlsx)
        ))
    ));
    Ok(())
}

#[test]
fn cached_conversion_exceeding_output_limit_is_rejected() -> Result<(), Box<dyn std::error::Error>>
{
    let (_root, cache) = cache()?;
    let source = source(OfficeDocumentFormat::Docx);
    let mut config = OfficeWorkerConfig::new(std::env::current_exe()?);
    config.max_output_bytes = 1;
    let key = CacheKey::office(&cache, &source, &config)?;
    let output = crate::multi_format::office_worker_parent::OfficeWorkerOutput {
        pdf: vec![0, 1],
        warnings: Vec::new(),
        preflight_diagnostics: Vec::new(),
    };
    let bytes = super::ConversionArtifact::encode(&output)?;
    cache.save(&key, &bytes)?;

    assert!(matches!(
        super::ConversionArtifact::output(&cache, &key, &source, &config),
        Err(PersistentCacheError::Capacity)
    ));
    Ok(())
}

#[test]
fn malformed_cached_conversion_is_reported_as_corrupt() -> Result<(), Box<dyn std::error::Error>> {
    let (_root, cache) = cache()?;
    let source = source(OfficeDocumentFormat::Docx);
    let config = OfficeWorkerConfig::new(std::env::current_exe()?);
    let key = CacheKey::office(&cache, &source, &config)?;
    cache.save(&key, b"not-json")?;

    assert!(matches!(
        super::ConversionArtifact::output(&cache, &key, &source, &config),
        Err(PersistentCacheError::Corrupt)
    ));
    Ok(())
}

#[test]
fn persistent_office_accessors_expose_cached_state_and_artifact()
-> Result<(), Box<dyn std::error::Error>> {
    let (_root, cache) = cache()?;
    let source = source(OfficeDocumentFormat::Docx);
    let identity = source.identity.clone();
    let config = OfficeWorkerConfig::new(PathBuf::from("worker"));
    let output = valid_output();
    let conversion_key =
        crate::multi_format::office_conversion_key::OfficeConversionKey::new(&source, &config);
    let session =
        OfficeStaticViewerSession::from_output(source, config, output, conversion_key, None)?;
    let persistent = PersistentOfficeViewerSession {
        session,
        cache,
        key: "office-accessors".to_owned(),
        conversion_cache_hit: true,
    };

    assert!(persistent.conversion_cache_hit());
    assert_eq!(identity, persistent.artifact().identity);
    assert_eq!(13, persistent.artifact().item_count);
    Ok(())
}

#[test]
fn failed_pdf_validation_does_not_persist_a_conversion() -> Result<(), Box<dyn std::error::Error>> {
    let (_root, cache) = cache()?;
    let output = crate::multi_format::office_worker_parent::OfficeWorkerOutput {
        pdf: b"invalid-pdf".to_vec(),
        warnings: Vec::new(),
        preflight_diagnostics: Vec::new(),
    };
    assert!(
        PersistentOfficeViewerSession::validated_session(
            source(OfficeDocumentFormat::Docx),
            OfficeWorkerConfig::new(PathBuf::from("worker")),
            cache.clone(),
            "invalid-output".into(),
            output,
            false,
            None,
        )
        .is_err()
    );
    assert_eq!(None, cache.load("invalid-output")?);
    Ok(())
}

#[test]
fn validated_conversion_is_persisted_and_cache_hits_do_not_write_again() -> TestResult {
    let (_root, cache) = cache()?;
    let config = OfficeWorkerConfig::new(std::env::current_exe()?);
    let source = source(OfficeDocumentFormat::Docx);
    let key = CacheKey::office(&cache, &source, &config)?;
    let session = PersistentOfficeViewerSession::validated_session(
        source.clone(),
        config.clone(),
        cache.clone(),
        key.clone(),
        valid_output(),
        false,
        None,
    )?;
    assert!(!session.conversion_cache_hit());
    let stored = cache.load(&key)?.ok_or("validated conversion absent")?;
    let (output, hit) = ConversionArtifact::output(&cache, &key, &source, &config)?;
    let session = PersistentOfficeViewerSession::validated_session(
        source,
        config,
        cache.clone(),
        key.clone(),
        output,
        hit,
        None,
    )?;
    assert!(session.conversion_cache_hit());
    assert_eq!(Some(stored), cache.load(&key)?);
    Ok(())
}

fn valid_output() -> crate::multi_format::office_worker_parent::OfficeWorkerOutput {
    crate::multi_format::office_worker_parent::OfficeWorkerOutput {
        pdf: include_bytes!("../../../../assets/reference/katana/pdf/sample.pdf").to_vec(),
        warnings: Vec::new(),
        preflight_diagnostics: Vec::new(),
    }
}
