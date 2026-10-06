use std::process::Command;

#[path = "support/persistent_cache_process.rs"]
mod process;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn persistent_artifacts_survive_real_process_restart_and_invalidate() -> TestResult {
    for format in ["pdf", "docx", "pptx"] {
        let root = tempfile::tempdir()?;
        for (revision, expected_hit) in [("one", false), ("one", true), ("two", false)] {
            let output = Command::new(std::env::current_exe()?)
                .args([
                    "--exact",
                    "persistent_cache_process_step",
                    "--ignored",
                    "--nocapture",
                ])
                .env("KDV_CACHE_ROOT", root.path())
                .env("KDV_CACHE_FORMAT", format)
                .env("KDV_CACHE_REVISION", revision)
                .env("KDV_CACHE_EXPECT_HIT", expected_hit.to_string())
                .env_remove("KDV_CACHE_INPUT")
                .env("DEBUG", "true")
                .output()?;
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("{}", String::from_utf8_lossy(&output.stdout));
            assert!(
                output.status.success(),
                "{format}: {stderr}\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
            if expected_hit {
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
    }
    Ok(())
}

#[test]
#[ignore = "real child-process entrypoint invoked by the parent regression"]
fn persistent_cache_process_step() -> TestResult {
    process::run()
}

#[test]
fn capacity_corruption_clear_and_changed_content_are_explicit() -> TestResult {
    process::storage_contract()
}
