use std::env;
use std::ffi::OsString;
use std::sync::Mutex;

static RUNTIME_ENV_LOCK: Mutex<()> = Mutex::new(());

pub(crate) struct RenderRuntimeTestEnv;

impl RenderRuntimeTestEnv {
    pub(crate) fn with_mathjax_env<T>(value: Option<&str>, test: impl FnOnce() -> T) -> T {
        let guard = RUNTIME_ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let previous = env::var_os("MATHJAX_JS");
        let restore = MathJaxEnvRestore { previous };
        Self::set_mathjax_env(value);
        let result = test();
        drop(restore);
        drop(guard);
        result
    }

    fn set_mathjax_env(value: Option<&str>) {
        match value {
            Some(value) => unsafe { env::set_var("MATHJAX_JS", value) },
            None => unsafe { env::remove_var("MATHJAX_JS") },
        }
    }
}

struct MathJaxEnvRestore {
    previous: Option<OsString>,
}

impl Drop for MathJaxEnvRestore {
    fn drop(&mut self) {
        match &self.previous {
            Some(value) => unsafe { env::set_var("MATHJAX_JS", value) },
            None => unsafe { env::remove_var("MATHJAX_JS") },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RUNTIME_ENV_LOCK, RenderRuntimeTestEnv};

    #[test]
    fn with_mathjax_env_restores_before_resuming_panic() {
        let previous = std::env::var_os("MATHJAX_JS");

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            RenderRuntimeTestEnv::with_mathjax_env(Some("/tmp/current-mathjax.js"), || {
                assert_eq!(
                    Some(std::ffi::OsString::from("/tmp/current-mathjax.js")),
                    std::env::var_os("MATHJAX_JS")
                );
                std::panic::resume_unwind(Box::new(String::from("runtime env panic branch")));
            });
        }));

        assert!(result.is_err());
        assert_eq!(previous, std::env::var_os("MATHJAX_JS"));
    }

    #[test]
    fn with_mathjax_env_recovers_poisoned_runtime_lock() {
        let handle = std::thread::spawn(|| {
            let _guard = RUNTIME_ENV_LOCK.lock();
            std::panic::resume_unwind(Box::new(String::from("poison runtime env lock")));
        });
        let _ = handle.join();

        RenderRuntimeTestEnv::with_mathjax_env(None, || ());
    }
}
