use super::*;

#[test]
fn fixture_export_cache_recovers_after_mutex_poisoning() {
    export_cache().clear_poison();
    drop(export_cache_lock());
    let poison_result = std::panic::catch_unwind(|| {
        if let Ok(_guard) = export_cache().lock() {
            let already_poisoned = export_cache().is_poisoned();
            assert!(already_poisoned, "poison fixture export cache");
        }
    });

    assert!(poison_result.is_err());
    drop(export_cache_lock());
}

#[test]
fn join_payload_returns_error_when_worker_panics() {
    let result = thread::scope(|scope| {
        join_payload(
            "PNG",
            scope.spawn(|| -> Result<Vec<u8>, String> {
                let worker_should_succeed = false;
                assert!(worker_should_succeed, "render worker panic");
                Ok(Vec::new())
            }),
        )
    });

    assert!(
        matches!(result, Err(ForgeError::Export(message)) if message == "PNG payload worker panicked")
    );
}
