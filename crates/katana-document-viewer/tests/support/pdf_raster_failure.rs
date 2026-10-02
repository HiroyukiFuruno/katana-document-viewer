use process_control::{ChildExt, Control};
use serde::Deserialize;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

pub type TestResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FailureResponse {
    status: String,
    stage: String,
    message: String,
}

pub fn workspace() -> Result<tempfile::TempDir, Box<dyn std::error::Error>> {
    let workspace = tempfile::tempdir()?;
    std::fs::write(workspace.path().join("input.office"), pdf())?;
    std::fs::write(
        workspace.path().join("request.json"),
        br#"{"page_index":0,"scale_bits":1065353216,"max_rgba_bytes":67108864}"#,
    )?;
    Ok(workspace)
}

pub fn pdf() -> &'static [u8] {
    include_bytes!("../../../../assets/reference/katana/pdf/sample.pdf")
}

pub fn assert_failure(workspace: &Path, max_output: u64, stage: &str) -> TestResult {
    assert_exit(workspace, max_output, 70)?;
    assert_failure_response(workspace, stage)
}

fn assert_failure_response(workspace: &Path, stage: &str) -> TestResult {
    let bytes = std::fs::read(workspace.join("response.json"))?;
    assert!(bytes.len() <= 64 * 1024);
    let response: FailureResponse = serde_json::from_slice(&bytes)?;
    assert_eq!("failed", response.status);
    assert_eq!(stage, response.stage);
    assert!(!response.message.is_empty());
    Ok(())
}

pub fn assert_exit(workspace: &Path, max_output: u64, exit: i64) -> TestResult {
    wait_for_exit(workspace, worker_command(workspace, max_output), exit)
}

pub fn assert_cpu_limit_failure(workspace: &Path, max_output: u64) -> TestResult {
    let mut command = worker_command(workspace, max_output);
    // childだけのCPU hard limitを先に下げ、実OSによる再引上げ拒否を検証する。
    unsafe {
        command.pre_exec(|| rlimit::setrlimit(rlimit::Resource::CPU, 1, 1));
    }
    wait_for_exit(workspace, command, 70)?;
    assert_failure_response(workspace, "cpu_limit")
}

fn worker_command(workspace: &Path, max_output: u64) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kdv-office-worker"));
    command
        .arg(workspace)
        .args(["pdf-raster", "1073741824", "15"])
        .arg(max_output.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
}

fn wait_for_exit(workspace: &Path, mut command: Command, exit: i64) -> TestResult {
    let profile = coverage_profile(workspace, &mut command);
    let mut child = command.spawn()?;
    let status = child
        .controlled()
        .time_limit(Duration::from_secs(15))
        .terminate_for_timeout()
        .strict_errors()
        .wait()?;
    assert_eq!(Some(exit), status.and_then(|status| status.code()));
    collect_profile(profile)?;
    Ok(())
}

fn coverage_profile(workspace: &Path, command: &mut Command) -> Option<(PathBuf, PathBuf)> {
    let original = std::env::var_os("LLVM_PROFILE_FILE")?;
    let root = Path::new(&original).parent()?.to_path_buf();
    let name = workspace.file_name()?.to_string_lossy();
    let source = workspace.join("pdf-raster-failure.profraw");
    let target = root.join(format!(
        "pdf-raster-failure-{}-{name}.profraw",
        std::process::id()
    ));
    command.env("LLVM_PROFILE_FILE", &source);
    Some((source, target))
}

fn collect_profile(profile: Option<(PathBuf, PathBuf)>) -> TestResult {
    if let Some((source, target)) = profile
        && source.is_file()
    {
        let _ = std::fs::copy(source, target)?;
    }
    Ok(())
}
