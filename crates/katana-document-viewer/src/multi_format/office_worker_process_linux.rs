use super::{OfficeWorkerConfig, OfficeWorkerError, normalize_wait_result};
use process_control::{ChildExt, Control};

pub(super) fn wait_for_worker(
    child: &mut std::process::Child,
    config: &OfficeWorkerConfig,
) -> Result<Option<i64>, OfficeWorkerError> {
    let result = child
        .controlled()
        .memory_limit(config.max_memory_bytes)
        .time_limit(config.timeout)
        .terminate_for_timeout()
        .strict_errors()
        .wait();
    normalize_linux_wait_result(child, config, result)
}

pub(super) fn normalize_linux_wait_result(
    child: &mut std::process::Child,
    config: &OfficeWorkerConfig,
    result: std::io::Result<Option<process_control::ExitStatus>>,
) -> Result<Option<i64>, OfficeWorkerError> {
    match result {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            linux_parent_wait_result(config, child.wait())
        }
        result => normalize_wait_result(config, result),
    }
}

pub(super) fn linux_parent_wait_result(
    config: &OfficeWorkerConfig,
    result: std::io::Result<std::process::ExitStatus>,
) -> Result<Option<i64>, OfficeWorkerError> {
    match result {
        Ok(status) => Ok(status.code().map(i64::from)),
        Err(error) => Err(linux_parent_wait_error(config, error)),
    }
}

pub(super) fn linux_parent_wait_error(
    config: &OfficeWorkerConfig,
    error: std::io::Error,
) -> OfficeWorkerError {
    OfficeWorkerError::unavailable(config, error.to_string())
}
