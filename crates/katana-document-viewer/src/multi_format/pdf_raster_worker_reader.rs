use super::OfficeWorkerError;
use std::io::Read;
use std::path::Path;

pub(super) fn read_regular_bounded(path: &Path, limit: u64) -> Result<Vec<u8>, OfficeWorkerError> {
    let metadata = std::fs::symlink_metadata(path).map_err(OfficeWorkerError::protocol_io)?;
    if !metadata.file_type().is_file() {
        return Err(OfficeWorkerError::protocol(
            "raster output is not a regular file".to_owned(),
        ));
    }
    if metadata.len() > limit {
        return Err(OfficeWorkerError::OutputLimitExceeded {
            actual: metadata.len(),
            limit,
        });
    }
    let file = std::fs::File::open(path).map_err(OfficeWorkerError::protocol_io)?;
    read_bounded_file(file, metadata.len(), limit)
}

fn read_bounded_file(
    file: std::fs::File,
    expected_length: u64,
    limit: u64,
) -> Result<Vec<u8>, OfficeWorkerError> {
    // metadataだけで容量を予約せず、上限付きの実readに応じて確保する。
    let mut bytes = Vec::new();
    file.take(limit.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(OfficeWorkerError::protocol_io)?;
    if bytes.len() as u64 != expected_length {
        return Err(OfficeWorkerError::protocol(
            "raster output changed while reading".to_owned(),
        ));
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::{read_bounded_file, read_regular_bounded};
    use crate::multi_format::OfficeWorkerError;

    #[test]
    fn raster_reader_reports_a_changed_file_and_actual_read_error()
    -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("frame.rgba");
        std::fs::write(&path, [1, 2, 3, 4])?;
        let file = std::fs::File::open(&path)?;
        std::fs::OpenOptions::new()
            .write(true)
            .open(&path)?
            .set_len(2)?;
        assert!(matches!(
            read_bounded_file(file, 4, 4),
            Err(OfficeWorkerError::Protocol { .. })
        ));
        let write_only = std::fs::File::create(&path)?;
        assert!(matches!(
            read_bounded_file(write_only, 0, 4),
            Err(OfficeWorkerError::Protocol { .. })
        ));
        Ok(())
    }

    #[test]
    fn raster_reader_accepts_bounded_bytes_and_rejects_missing_large_or_directory_outputs()
    -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("frame.rgba");
        assert!(read_regular_bounded(&path, 4).is_err());
        std::fs::write(&path, [1, 2, 3, 4])?;
        assert_eq!(vec![1, 2, 3, 4], read_regular_bounded(&path, 4)?);
        assert_eq!(
            Err(OfficeWorkerError::OutputLimitExceeded {
                actual: 4,
                limit: 3
            }),
            read_regular_bounded(&path, 3)
        );
        assert!(read_regular_bounded(dir.path(), 4).is_err());
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn raster_reader_does_not_follow_a_worker_symlink() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let input = dir.path().join("input");
        let output = dir.path().join("output");
        std::fs::write(&input, [1, 2, 3, 4])?;
        std::os::unix::fs::symlink(&input, &output)?;
        assert!(read_regular_bounded(&output, 4).is_err());
        Ok(())
    }
}
