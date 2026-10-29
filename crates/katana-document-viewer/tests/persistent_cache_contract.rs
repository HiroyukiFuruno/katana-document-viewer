use std::path::Path;
use std::process::Command;

#[path = "support/persistent_cache_process.rs"]
mod process;
type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn persistent_artifacts_survive_real_process_restart_and_invalidate() -> TestResult {
    for format in ["pdf", "docx", "pptx"] {
        let root = tempfile::tempdir()?;
        for (revision, expected_hit) in [("one", false), ("one", true), ("two", false)] {
            let output = run_process(root.path(), format, revision, expected_hit, false)?;
            verify_output(&output, format, expected_hit)?;
        }
    }
    Ok(())
}

#[test]
fn path_named_worker_survives_cold_conversion_and_process_restart() -> TestResult {
    let root = tempfile::tempdir()?;
    for expected_hit in [false, true] {
        let output = run_process(root.path(), "docx", "path-worker", expected_hit, true)?;
        verify_output(&output, "docx", expected_hit)?;
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

fn verify_output(output: &std::process::Output, format: &str, hit: bool) -> TestResult {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{stdout}");
    assert!(output.status.success(), "{format}: {stderr}\n{}", stdout);
    verify_first_frame_clock(&stdout, format)?;
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
    Ok(())
}

fn verify_first_frame_clock(stdout: &str, format: &str) -> TestResult {
    let lines = stdout.lines().collect::<Vec<_>>();
    for mode in ["process_open", "warm_reopen"] {
        verify_mode_clock(&lines, format, mode)?;
    }
    Ok(())
}

fn verify_mode_clock(lines: &[&str], format: &str, mode: &str) -> TestResult {
    let ready_marker = format!("KDV_FIRST_FRAME_READY format={format} mode={mode}");
    let repeat_marker = format!("KDV_REPEAT_VERIFIED format={format} mode={mode}");
    let measure_prefix = format!("KDV_CACHE_MEASURE format={format} mode={mode} ");
    let ready = required_position(lines, &ready_marker, true)?;
    let repeat = required_position(lines, &repeat_marker, true)?;
    let measure = required_position(lines, &measure_prefix, false)?;
    if !(ready < repeat && repeat < measure) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid measurement marker order for {format}/{mode}"),
        )
        .into());
    }
    let line = lines[measure];
    let acquire = measurement_value(line, "acquire_seconds")?;
    let first_frame = measurement_value(line, "first_frame_seconds")?;
    let cycle = measurement_value(line, "cycle_seconds")?;
    if !(acquire.is_finite()
        && first_frame.is_finite()
        && cycle.is_finite()
        && 0.0 <= acquire
        && acquire <= first_frame
        && first_frame <= cycle)
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid timing relationship for {format}/{mode}: {line}"),
        )
        .into());
    }
    Ok(())
}

fn required_position(lines: &[&str], marker: &str, exact: bool) -> TestResult<usize> {
    lines
        .iter()
        .position(|line| {
            if exact {
                *line == marker
            } else {
                line.starts_with(marker)
            }
        })
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("missing output marker {marker}"),
            )
            .into()
        })
}

fn measurement_value(line: &str, key: &str) -> TestResult<f64> {
    let prefix = format!("{key}=");
    let field = line
        .split_whitespace()
        .find_map(|field| field.strip_prefix(&prefix))
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("missing {key} in measurement: {line}"),
            )
        })?;
    Ok(field.parse()?)
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
