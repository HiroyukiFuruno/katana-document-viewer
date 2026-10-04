use super::TestResult;
use super::windows_command_line::WindowsCommandLine;
use super::worker_environment::worker_environment;
use process_control::{ChildExt, Control};
use rappct::acl::{AccessMask, ResourcePath, grant_to_package};
use rappct::{
    AppContainerProfile, JobLimits, LaunchOptions, SecurityCapabilitiesBuilder, StdioConfig,
};
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

const PROFILE_NAME: &str = "Katana.DocumentViewer.OfficeWorker";
static PROFILE: OnceLock<Mutex<Option<AppContainerProfile>>> = OnceLock::new();

fn profile() -> TestResult<AppContainerProfile> {
    let mut cached = PROFILE
        .get_or_init(|| Mutex::new(None))
        .lock()
        .map_err(|error| format!("wrapped-title AppContainer profile cache: {error}"))?;
    if let Some(profile) = cached.as_ref() {
        return Ok(profile.clone());
    }
    let profile = AppContainerProfile::ensure(
        PROFILE_NAME,
        "KatanA document viewer office worker",
        Some("Network-denied Office document helper"),
    )?;
    *cached = Some(profile.clone());
    Ok(profile)
}

pub(super) fn workspace() -> TestResult<tempfile::TempDir> {
    let _profile = profile()?;
    let root = std::env::var_os("LOCALAPPDATA")
        .ok_or("LOCALAPPDATA is unavailable for the wrapped-title AppContainer")?;
    let root = Path::new(&root)
        .join("Packages")
        .join(PROFILE_NAME)
        .join("AC")
        .join("Temp");
    std::fs::create_dir_all(&root)?;
    // 空白を含む実workspaceで、本番と同じWindows引数quoteも検証する。
    Ok(tempfile::Builder::new()
        .prefix("wrapped title ")
        .tempdir_in(root)?)
}

pub(super) fn run(worker: &Path, workspace: &Path, timeout: Duration) -> TestResult<i64> {
    let profile = profile()?;
    let staged = workspace.join("kdv-office-worker.exe");
    std::fs::copy(worker, &staged)?;
    for resource in [
        ResourcePath::Directory(workspace.to_path_buf()),
        ResourcePath::File(workspace.join("input.office")),
        ResourcePath::File(staged.clone()),
    ] {
        grant_to_package(resource, &profile.sid, AccessMask::GENERIC_ALL)?;
    }
    let capabilities = SecurityCapabilitiesBuilder::new(&profile.sid).build()?;
    let options = options(&staged, workspace);
    let child = rappct::launch::launch_in_container_with_io(&capabilities, &options)?;
    wait_with_drained_output(child, timeout)
}

fn wait_with_drained_output(
    mut child: rappct::launch::LaunchedIo,
    timeout: Duration,
) -> TestResult<i64> {
    drop(child.stdin.take());
    let mut stdout = child
        .stdout
        .take()
        .ok_or("worker stdout pipe unavailable")?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or("worker stderr pipe unavailable")?;
    std::thread::scope(|scope| -> TestResult<i64> {
        let output = std::thread::Builder::new().spawn_scoped(scope, move || {
            std::io::copy(&mut stdout, &mut std::io::sink())
        })?;
        let errors = std::thread::Builder::new().spawn_scoped(scope, move || {
            std::io::copy(&mut stderr, &mut std::io::sink())
        })?;
        let status = child.wait(Some(timeout));
        output
            .join()
            .map_err(|_| "worker stdout reader panicked")??;
        errors
            .join()
            .map_err(|_| "worker stderr reader panicked")??;
        Ok(i64::from(status?))
    })
}

fn options(worker: &Path, workspace: &Path) -> LaunchOptions {
    LaunchOptions {
        exe: worker.to_path_buf(),
        cmdline: Some(WindowsCommandLine::from_arguments([
            worker.to_string_lossy().into_owned(),
            workspace.to_string_lossy().into_owned(),
            "pptx".to_owned(),
            "2147483648".to_owned(),
            "46".to_owned(),
            "134217728".to_owned(),
        ])),
        cwd: Some(workspace.to_path_buf()),
        env: Some(worker_environment(workspace)),
        // Nullは親の全継承可能handleを渡すため、専用pipeだけを明示して起動する。
        stdio: StdioConfig::Pipe,
        join_job: Some(JobLimits {
            memory_bytes: Some(2_147_483_648),
            cpu_rate_percent: None,
            kill_on_job_close: true,
        }),
        ..LaunchOptions::default()
    }
}

#[test]
fn worker_pipes_keep_launch_and_resource_limits_explicit() {
    let workspace = Path::new("C:\\wrapped title workspace");
    let launch = options(&workspace.join("kdv-office-worker.exe"), workspace);
    assert!(matches!(launch.stdio, StdioConfig::Pipe));
    let limits = launch.join_job.as_ref();
    assert_eq!(
        limits.and_then(|limits| limits.memory_bytes),
        Some(2_147_483_648)
    );
    assert!(limits.is_some_and(|limits| limits.kill_on_job_close));
    assert!(!launch.suspended);
}

#[test]
fn direct_worker_is_rejected_outside_appcontainer() -> TestResult {
    let directory = workspace()?;
    std::fs::write(
        directory.path().join("input.office"),
        super::title_pptx(false, "ctr")?,
    )?;
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_kdv-office-worker"))
        .arg(directory.path())
        .args(["pptx", "2147483648", "46", "134217728"])
        .env_clear()
        .envs(worker_environment(directory.path()))
        .stdin(std::process::Stdio::null())
        .spawn()?;
    let status = child
        .controlled()
        .time_limit(Duration::from_secs(45))
        .terminate_for_timeout()
        .strict_errors()
        .wait()?
        .ok_or("non-AppContainer worker exceeded its existing deadline")?;
    assert_eq!(status.code(), Some(70));
    let response: serde_json::Value =
        serde_json::from_slice(&std::fs::read(directory.path().join("response.json"))?)?;
    assert_eq!(response["status"], "failed");
    assert_eq!(response["stage"], "sandbox");
    Ok(())
}
