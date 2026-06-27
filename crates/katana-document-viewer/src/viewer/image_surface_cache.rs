use super::ViewerImageSurface;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

const MAX_CACHED_SURFACES: usize = 64;

pub(super) struct ViewerImageSurfaceCache;

impl ViewerImageSurfaceCache {
    pub(super) fn get(fingerprint: &str) -> Option<ViewerImageSurface> {
        cache().lock().ok()?.get(fingerprint).cloned()
    }

    pub(super) fn put(surface: ViewerImageSurface) -> ViewerImageSurface {
        if let Ok(mut cache) = cache().lock() {
            if cache.len() >= MAX_CACHED_SURFACES
                && let Some(first_key) = cache.keys().next().cloned()
            {
                cache.remove(&first_key);
            }
            cache.insert(surface.fingerprint.clone(), surface.clone());
        }
        surface
    }

    #[cfg(test)]
    pub(super) fn clear_for_tests() {
        if let Ok(mut cache) = cache().lock() {
            cache.clear();
        }
    }

    #[cfg(test)]
    pub(super) fn len_for_tests() -> usize {
        cache().lock().map_or(0, |cache| cache.len())
    }
}

fn cache() -> &'static Mutex<HashMap<String, ViewerImageSurface>> {
    static CACHE: OnceLock<Mutex<HashMap<String, ViewerImageSurface>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}
