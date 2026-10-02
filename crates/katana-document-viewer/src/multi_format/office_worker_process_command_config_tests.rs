use super::super::command_config::{
    configure_command_with_debug, configure_mode_command_with_debug, cpu_seconds,
};
use crate::multi_format::{OfficeDocumentFormat, OfficeWorkerConfig, debug_trace::DebugTrace};
use std::path::PathBuf;
use std::time::Duration;

#[test]
fn debug_environment_is_propagated_only_when_enabled() {
    for debug_enabled in [false, true] {
        let mut command = std::process::Command::new("worker");
        let config = OfficeWorkerConfig::new(PathBuf::from("worker"));
        configure_command_with_debug(
            &mut command,
            std::path::Path::new("workspace"),
            OfficeDocumentFormat::Docx,
            &config,
            debug_enabled,
        );
        let has_debug = command
            .get_envs()
            .any(|(name, value)| name == "DEBUG" && value == Some(std::ffi::OsStr::new("true")));
        assert_eq!(debug_enabled, has_debug);
    }
}

#[test]
fn pdf_raster_mode_uses_the_standard_worker_arguments() {
    let mut command = std::process::Command::new("worker");
    let mut config = OfficeWorkerConfig::new(PathBuf::from("worker"));
    config.timeout = Duration::from_secs(30);
    configure_mode_command_with_debug(
        &mut command,
        std::path::Path::new("workspace"),
        "pdf-raster",
        &config,
        false,
    );
    let arguments: Vec<_> = command
        .get_args()
        .map(|argument| argument.to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        vec![
            "workspace",
            "pdf-raster",
            &config.max_memory_bytes.to_string(),
            &cpu_seconds(config.timeout).to_string(),
            &config.max_output_bytes.to_string(),
        ],
        arguments
    );
}

#[test]
fn legacy_office_mode_keeps_its_worker_arguments() {
    let mut command = std::process::Command::new("worker");
    let config = OfficeWorkerConfig::new(PathBuf::from("worker"));
    configure_command_with_debug(
        &mut command,
        std::path::Path::new("workspace"),
        OfficeDocumentFormat::Docx,
        &config,
        false,
    );
    let arguments: Vec<_> = command
        .get_args()
        .map(|argument| argument.to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        vec![
            "workspace",
            "docx",
            &config.max_memory_bytes.to_string(),
            &cpu_seconds(config.timeout).to_string(),
            &config.max_output_bytes.to_string(),
        ],
        arguments
    );
}

#[test]
fn debug_worker_environment_carries_the_office_session_correlation() {
    let _trace_session = DebugTrace::session((42, 0x0123_4567_89ab_cdef));
    let mut command = std::process::Command::new("worker");
    let config = OfficeWorkerConfig::new(PathBuf::from("worker"));
    configure_command_with_debug(
        &mut command,
        std::path::Path::new("workspace"),
        OfficeDocumentFormat::Docx,
        &config,
        true,
    );
    let environment: Vec<_> = command
        .get_envs()
        .filter_map(|(name, value)| value.map(|value| (name, value)))
        .collect();
    assert!(environment.iter().any(|(name, value)| {
        *name == std::ffi::OsStr::new("KDV_TRACE_SESSION") && *value != std::ffi::OsStr::new("0")
    }));
    assert!(environment.iter().any(|(name, value)| {
        *name == std::ffi::OsStr::new("KDV_TRACE_SOURCE") && value.len() == 16
    }));
}
