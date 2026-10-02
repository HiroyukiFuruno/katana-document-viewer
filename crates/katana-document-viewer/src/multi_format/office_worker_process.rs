#[cfg(target_os = "macos")]
use super::office_worker_monitor::MacOsMemoryMonitor;
#[path = "office_worker_process_command_config.rs"]
mod command_config;
#[cfg(target_os = "linux")]
#[path = "office_worker_process_linux.rs"]
mod linux;
#[cfg(windows)]
#[path = "office_worker_process_windows.rs"]
mod windows;
use super::{OfficeDocumentFormat, OfficeWorkerConfig, OfficeWorkerError};
#[cfg(not(windows))]
use command_config::configure_command;
#[cfg(windows)]
use command_config::cpu_seconds;
use command_config::format_argument;
#[cfg(target_os = "linux")]
use linux::wait_for_worker;
#[cfg(all(not(windows), not(target_os = "linux")))]
use process_control::{ChildExt, Control};
use std::path::Path;
#[cfg(not(windows))]
use std::process::Command;

pub(crate) struct OfficeWorkerProcess;

impl OfficeWorkerProcess {
    #[cfg(not(windows))]
    pub(crate) fn run(
        workspace: &Path,
        format: OfficeDocumentFormat,
        config: &OfficeWorkerConfig,
    ) -> Result<Option<i64>, OfficeWorkerError> {
        Self::run_mode(workspace, format_argument(format), config)
    }

    #[cfg(not(windows))]
    pub(crate) fn run_pdf_raster(
        workspace: &Path,
        config: &OfficeWorkerConfig,
    ) -> Result<Option<i64>, OfficeWorkerError> {
        Self::run_mode(workspace, "pdf-raster", config)
    }

    #[cfg(not(windows))]
    fn run_mode(
        workspace: &Path,
        format_argument: &str,
        config: &OfficeWorkerConfig,
    ) -> Result<Option<i64>, OfficeWorkerError> {
        let mut command = Command::new(&config.executable);
        configure_command(&mut command, workspace, format_argument, config);
        #[cfg(coverage)]
        let coverage_profile = super::coverage_profile::ChildCoverageProfile::configure(
            &mut command,
            workspace,
            "office",
        );
        let mut child = {
            let _spawn = super::debug_trace::DebugTrace::start("office.worker_spawn");
            spawn_worker(&mut command, config)?
        };
        #[cfg(target_os = "macos")]
        let memory_monitor = MacOsMemoryMonitor::start(child.id(), config.max_memory_bytes);
        let result = wait_for_worker(&mut child, config)?;
        #[cfg(target_os = "macos")]
        finish_memory_monitor(memory_monitor, config.max_memory_bytes)?;
        #[cfg(coverage)]
        if let Some(profile) = coverage_profile {
            let _ = profile.collect();
        }
        Ok(result)
    }

    #[cfg(windows)]
    pub(crate) fn run(
        workspace: &Path,
        format: OfficeDocumentFormat,
        config: &OfficeWorkerConfig,
    ) -> Result<Option<i64>, OfficeWorkerError> {
        windows::OfficeWorkerWindowsProcess::run(workspace, format, config)
    }

    #[cfg(windows)]
    pub(crate) fn run_pdf_raster(
        workspace: &Path,
        config: &OfficeWorkerConfig,
    ) -> Result<Option<i64>, OfficeWorkerError> {
        windows::OfficeWorkerWindowsProcess::run_pdf_raster(workspace, config)
    }
}

#[cfg(target_os = "macos")]
fn finish_memory_monitor(
    memory_monitor: MacOsMemoryMonitor,
    limit: usize,
) -> Result<(), OfficeWorkerError> {
    let _monitor = super::debug_trace::DebugTrace::start("office.monitor_finish");
    if memory_monitor.finish() {
        return Err(OfficeWorkerError::WorkerMemoryLimitExceeded { limit });
    }
    Ok(())
}

#[cfg(not(windows))]
fn spawn_worker(
    command: &mut Command,
    config: &OfficeWorkerConfig,
) -> Result<std::process::Child, OfficeWorkerError> {
    command
        .spawn()
        .map_err(|error| OfficeWorkerError::unavailable(config, error.to_string()))
}

#[cfg(all(not(windows), not(target_os = "linux")))]
fn wait_for_worker(
    child: &mut std::process::Child,
    config: &OfficeWorkerConfig,
) -> Result<Option<i64>, OfficeWorkerError> {
    let result = child
        .controlled()
        .time_limit(config.timeout)
        .terminate_for_timeout()
        .strict_errors()
        .wait();
    normalize_wait_result(config, result)
}

#[cfg(not(windows))]
fn normalize_wait_result(
    config: &OfficeWorkerConfig,
    result: std::io::Result<Option<process_control::ExitStatus>>,
) -> Result<Option<i64>, OfficeWorkerError> {
    let status = match result {
        Ok(status) => status,
        Err(error) => return Err(OfficeWorkerError::unavailable(config, error.to_string())),
    };
    match status {
        Some(status) => Ok(status.code()),
        None => Err(OfficeWorkerError::WorkerTimedOut),
    }
}

#[cfg(not(windows))]
#[cfg(test)]
#[path = "office_worker_process_tests.rs"]
mod tests;
