use super::*;

#[test]
fn rendered_export_cache_recovers_after_mutex_poisoning() {
    rendered_export_cache().clear_poison();
    let poison_result = std::panic::catch_unwind(|| {
        if let Ok(_guard) = rendered_export_cache().lock() {
            let already_poisoned = rendered_export_cache().is_poisoned();
            assert!(already_poisoned, "poison rendered export cache");
        }
    });

    assert!(poison_result.is_err());
    drop(rendered_export_cache_lock());
}
