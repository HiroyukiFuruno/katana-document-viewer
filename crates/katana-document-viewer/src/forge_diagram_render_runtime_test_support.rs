use std::env;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Mutex;

use crate::RenderedDiagram;

static RUNTIME_ENV_LOCK: Mutex<()> = Mutex::new(());

pub(super) fn with_runtime_env(
    name: &str,
    value: Option<&str>,
    test: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    let _guard = match RUNTIME_ENV_LOCK.lock() {
        Ok(guard) => guard,
        Err(error) => return Err(format!("runtime env lock failed: {error}")),
    };
    let previous = env::var_os(name);
    match value {
        Some(value) => unsafe { env::set_var(name, value) },
        None => unsafe { env::remove_var(name) },
    }
    let result = catch_unwind(AssertUnwindSafe(test));
    match previous {
        Some(value) => unsafe { env::set_var(name, value) },
        None => unsafe { env::remove_var(name) },
    }
    match result {
        Ok(result) => result,
        Err(_) => Err("diagram runtime environment test panicked".to_string()),
    }
}

pub(super) fn must_render_error(result: Result<RenderedDiagram, String>) -> Result<String, String> {
    match result {
        Ok(rendered) => Err(format!(
            "diagram render unexpectedly succeeded: {}",
            rendered.node_id
        )),
        Err(error) => Ok(error),
    }
}
