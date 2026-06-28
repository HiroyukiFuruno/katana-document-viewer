mod cases;
mod exported;
mod fixture_files;

pub use cases::{DirectVisualCase, DirectVisualCases};
pub use exported::ExportedDirectVisual;

use std::error::Error;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn unique_output_dir() -> Result<PathBuf, Box<dyn Error>> {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!("kdv-direct-visual-e2e-{nanos}")))
}
