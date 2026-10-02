use super::OfficeDocumentFormat;
#[cfg(not(windows))]
use super::OfficeWorkerConfig;
use std::time::Duration;

#[cfg(not(windows))]
use std::path::Path;
#[cfg(not(windows))]
use std::process::{Command, Stdio};

#[cfg(not(windows))]
pub(super) fn configure_command(
    command: &mut Command,
    workspace: &Path,
    format_argument: &str,
    config: &OfficeWorkerConfig,
) {
    configure_mode_command_with_debug(
        command,
        workspace,
        format_argument,
        config,
        super::super::debug_trace::DebugTrace::enabled(),
    );
}

#[cfg(not(windows))]
pub(super) fn configure_mode_command_with_debug(
    command: &mut Command,
    workspace: &Path,
    format_argument: &str,
    config: &OfficeWorkerConfig,
    debug_enabled: bool,
) {
    command
        .arg(workspace)
        .arg(format_argument)
        .arg(config.max_memory_bytes.to_string())
        .arg(cpu_seconds(config.timeout).to_string())
        .arg(config.max_output_bytes.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .env_clear();
    if debug_enabled {
        command.stderr(Stdio::inherit()).env("DEBUG", "true");
        if let Some((session, source)) = super::super::debug_trace::DebugTrace::worker_environment()
        {
            command
                .env("KDV_TRACE_SESSION", session)
                .env("KDV_TRACE_SOURCE", source);
        }
    } else {
        command.stderr(Stdio::null());
    }
}

pub(super) const fn format_argument(format: OfficeDocumentFormat) -> &'static str {
    match format {
        OfficeDocumentFormat::Docx => "docx",
        OfficeDocumentFormat::Pptx => "pptx",
        OfficeDocumentFormat::Xlsx => "xlsx",
    }
}

pub(super) fn cpu_seconds(timeout: Duration) -> u64 {
    timeout.as_secs().saturating_add(1).max(1)
}

#[cfg(all(not(windows), test))]
pub(super) fn configure_command_with_debug(
    command: &mut Command,
    workspace: &Path,
    format: OfficeDocumentFormat,
    config: &OfficeWorkerConfig,
    debug_enabled: bool,
) {
    configure_mode_command_with_debug(
        command,
        workspace,
        format_argument(format),
        config,
        debug_enabled,
    );
}
