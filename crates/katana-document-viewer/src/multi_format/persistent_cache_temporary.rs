use super::{PersistentCacheError, key::CacheKey};
use cap_std::fs::{Dir, File, OpenOptions};
use std::{io::Write, path::PathBuf};

struct Temporary<'a> {
    directory: &'a Dir,
    name: PathBuf,
}

impl<'a> Temporary<'a> {
    fn new(directory: &'a Dir) -> std::io::Result<(Self, File)> {
        let mut name = PathBuf::new();
        // 名前生成だけを借り、作成・後始末は保持directoryに固定する。
        let temporary = tempfile::Builder::new()
            .prefix(".kdv-")
            .disable_cleanup(true)
            .make_in(".", |path| {
                name = PathBuf::from(path.file_name().ok_or(std::io::ErrorKind::InvalidInput)?);
                directory.open_with(&name, &options())
            })?;
        let (file, _) = temporary.into_parts();
        Ok((Self { directory, name }, file))
    }
}

impl Drop for Temporary<'_> {
    fn drop(&mut self) {
        let _ = self.directory.remove_file(&self.name);
    }
}

pub(super) fn publish(
    directory: &Dir,
    key: &str,
    bytes: &[u8],
    magic: &[u8],
) -> Result<(), PersistentCacheError> {
    let (temporary, mut file) = Temporary::new(directory)?;
    let result = (|| {
        file.write_all(magic)?;
        file.write_all(CacheKey::artifact_digest(key, bytes).as_bytes())?;
        file.write_all(bytes)?;
        file.sync_all()?;
        directory.hard_link(&temporary.name, directory, key)?;
        Ok(())
    })();
    drop(file);
    result
}

fn options() -> OpenOptions {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use cap_std::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options
}
