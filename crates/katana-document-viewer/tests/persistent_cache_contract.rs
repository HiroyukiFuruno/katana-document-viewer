use std::path::Path;
use std::process::Command;

#[path = "support/persistent_cache_process.rs"]
mod process;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn persistent_artifacts_survive_real_process_restart_and_invalidate() -> TestResult {
    for format in ["pdf", "docx", "pptx"] {
        let root = tempfile::tempdir()?;
        for (revision, expected_hit) in [("one", false), ("one", true), ("two", false)] {
            let output = run_process(root.path(), format, revision, expected_hit, false)?;
            verify_output(&output, format, expected_hit);
        }
    }
    Ok(())
}

#[test]
fn path_named_worker_survives_cold_conversion_and_process_restart() -> TestResult {
    let root = tempfile::tempdir()?;
    for expected_hit in [false, true] {
        let output = run_process(root.path(), "docx", "path-worker", expected_hit, true)?;
        verify_output(&output, "docx", expected_hit);
    }
    Ok(())
}

fn run_process(
    root: &Path,
    format: &str,
    revision: &str,
    hit: bool,
    bare: bool,
) -> Result<std::process::Output, Box<dyn std::error::Error>> {
    let mut command = Command::new(std::env::current_exe()?);
    command
        .args([
            "--exact",
            "persistent_cache_process_step",
            "--ignored",
            "--nocapture",
        ])
        .env("KDV_CACHE_ROOT", root)
        .env("KDV_CACHE_FORMAT", format)
        .env("KDV_CACHE_REVISION", revision)
        .env("KDV_CACHE_EXPECT_HIT", hit.to_string())
        .env_remove("KDV_CACHE_INPUT")
        .env_remove(WORKER_NAME_ENV)
        .env("DEBUG", "true");
    if bare {
        configure_bare_worker(&mut command)?;
    }
    Ok(command.output()?)
}

fn verify_output(output: &std::process::Output, format: &str, hit: bool) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("{}", String::from_utf8_lossy(&output.stdout));
    assert!(
        output.status.success(),
        "{format}: {stderr}\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
    if hit {
        assert!(
            !stderr.contains("stage=pdf.render "),
            "restart must suppress raster: {stderr}"
        );
        assert!(
            !stderr.contains("stage=office.conversion "),
            "restart must suppress conversion: {stderr}"
        );
    }
}

fn configure_bare_worker(command: &mut Command) -> TestResult {
    let worker = Path::new(env!("CARGO_BIN_EXE_kdv-office-worker"));
    let parent = worker.parent().ok_or("worker parent directory missing")?;
    let mut paths = vec![parent.to_path_buf()];
    if let Some(path) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&path));
    }
    let name = worker.file_name().ok_or("worker filename missing")?;
    command
        .env(WORKER_NAME_ENV, name)
        .env("PATH", std::env::join_paths(paths)?);
    Ok(())
}

const WORKER_NAME_ENV: &str = "KDV_CACHE_WORKER_NAME";

#[test]
#[ignore = "real child-process entrypoint invoked by the parent regression"]
fn persistent_cache_process_step() -> TestResult {
    process::run()
}

#[test]
fn capacity_corruption_clear_and_changed_content_are_explicit() -> TestResult {
    process::storage_contract()
}
