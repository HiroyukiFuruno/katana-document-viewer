use super::PersistentDocumentCache;
use std::{
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::Duration,
};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn public_usage_waits_for_atomic_publication_lock() -> TestResult {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("cache");
    let cache = PersistentDocumentCache::new(root.clone(), 1024, "usage-lock-v1".into())?;
    cache.save("existing", b"stable artifact")?;
    let stable_before = cache.used_bytes()?;
    let lock = cache.lock()?;
    let (temporary, published) = create_in_progress_publication(&root)?;

    let (started_tx, started_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();
    let reader = spawn_usage_reader(cache, started_tx, result_tx);
    assert_reader_waits(started_rx, &result_rx)?;

    std::fs::remove_file(temporary)?;
    drop(lock);
    let actual = result_rx
        .recv_timeout(Duration::from_secs(5))?
        .map_err(std::io::Error::other)?;
    reader
        .join()
        .map_err(|_| std::io::Error::other("usage reader panicked"))?;
    assert_eq!(stable_before + b"new artifact".len() as u64, actual);
    assert!(published.is_file());
    Ok(())
}

fn create_in_progress_publication(
    root: &std::path::Path,
) -> TestResult<(std::path::PathBuf, std::path::PathBuf)> {
    let temporary = root.join(".kdv-in-progress");
    let published = root.join("published");
    std::fs::write(&temporary, b"new artifact")?;
    std::fs::hard_link(&temporary, &published)?;
    Ok((temporary, published))
}

fn spawn_usage_reader(
    cache: PersistentDocumentCache,
    started: mpsc::Sender<()>,
    result: mpsc::Sender<Result<u64, String>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let _ = started.send(());
        let usage = cache.used_bytes().map_err(|error| error.to_string());
        let _ = result.send(usage);
    })
}

fn assert_reader_waits(
    started: mpsc::Receiver<()>,
    result: &mpsc::Receiver<Result<u64, String>>,
) -> TestResult {
    started.recv_timeout(Duration::from_secs(5))?;
    assert_eq!(
        Err(RecvTimeoutError::Timeout),
        result.recv_timeout(Duration::from_millis(100))
    );
    Ok(())
}
