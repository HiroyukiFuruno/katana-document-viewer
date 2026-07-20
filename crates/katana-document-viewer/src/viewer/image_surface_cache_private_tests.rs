use super::{ViewerImageSurfaceCache, cache};
use crate::viewer::ViewerImageSurface;

#[test]
fn poisoned_cache_recovers_for_subsequent_surface_operations() {
    let handle = std::thread::spawn(|| {
        let _guard = match cache().lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let cache_was_already_poisoned = cache().is_poisoned();
        assert!(
            cache_was_already_poisoned,
            "poison cache for recovery contract"
        );
    });
    assert!(handle.join().is_err());

    let surface = ViewerImageSurface {
        fingerprint: "poison-recovery".to_string(),
        width: 1,
        height: 1,
        display_width: 1.0,
        display_height: 1.0,
        content_scale: 100,
        rgba: vec![0, 0, 0, 255],
    };

    assert_eq!(surface, ViewerImageSurfaceCache::put(surface.clone()));
    assert_eq!(
        Some(surface),
        ViewerImageSurfaceCache::get("poison-recovery")
    );
}
