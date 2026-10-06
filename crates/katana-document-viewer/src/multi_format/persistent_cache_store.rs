use super::{PersistentCacheError, PersistentDocumentCache, key::CacheKey};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const MAGIC: &[u8; 8] = b"KDVC0001";
const HEADER_BYTES: u64 = 72;
const MAX_ARTIFACT_BYTES: u64 = 128 * 1024 * 1024;

impl PersistentDocumentCache {
    pub(super) fn prepare(&self) -> Result<(), PersistentCacheError> {
        std::fs::create_dir_all(&self.root)?;
        let metadata = std::fs::symlink_metadata(&self.root)?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(PersistentCacheError::UnsafeDirectory);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&self.root, std::fs::Permissions::from_mode(0o700))?;
        }
        Ok(())
    }

    fn lock(&self) -> Result<File, PersistentCacheError> {
        let path = self.root.join(".lock");
        if path
            .symlink_metadata()
            .is_ok_and(|m| !m.is_file() || m.file_type().is_symlink())
        {
            return Err(PersistentCacheError::UnsafeDirectory);
        }
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(path)?;
        file.lock()?;
        Ok(file)
    }

    pub(super) fn load(&self, key: &str) -> Result<Option<Vec<u8>>, PersistentCacheError> {
        let _lock = self.lock()?;
        let path = self.root.join(key);
        let metadata = match path.symlink_metadata() {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(PersistentCacheError::UnsafeDirectory);
        }
        if metadata.len() > self.max_bytes.min(MAX_ARTIFACT_BYTES) {
            return Err(PersistentCacheError::Capacity);
        }
        if metadata.len() < HEADER_BYTES {
            return Err(PersistentCacheError::Corrupt);
        }
        let mut bytes = Vec::new();
        File::open(path)?
            .take(self.max_bytes.min(MAX_ARTIFACT_BYTES) + 1)
            .read_to_end(&mut bytes)?;
        Self::decode(bytes, key).map(Some)
    }

    fn decode(bytes: Vec<u8>, key: &str) -> Result<Vec<u8>, PersistentCacheError> {
        let payload = bytes
            .get(HEADER_BYTES as usize..)
            .ok_or(PersistentCacheError::Corrupt)?;
        if bytes.get(..8) != Some(MAGIC.as_slice())
            || bytes.get(8..72) != Some(CacheKey::artifact_digest(key, payload).as_bytes())
        {
            return Err(PersistentCacheError::Corrupt);
        }
        Ok(payload.to_vec())
    }

    pub(super) fn save(&self, key: &str, bytes: &[u8]) -> Result<(), PersistentCacheError> {
        let _lock = self.lock()?;
        let required = (bytes.len() as u64).saturating_add(HEADER_BYTES);
        let destination = self.root.join(key);
        if destination.try_exists()? {
            return Ok(());
        }
        if required > MAX_ARTIFACT_BYTES
            || self.used_bytes()?.saturating_add(required) > self.max_bytes
        {
            return Err(PersistentCacheError::Capacity);
        }
        let mut temporary = tempfile::NamedTempFile::new_in(&self.root)?;
        temporary.write_all(MAGIC)?;
        temporary.write_all(CacheKey::artifact_digest(key, bytes).as_bytes())?;
        temporary.write_all(bytes)?;
        temporary.as_file().sync_all()?;
        temporary
            .persist_noclobber(destination)
            .map_err(|error| error.error)?;
        Ok(())
    }

    pub fn used_bytes(&self) -> Result<u64, PersistentCacheError> {
        let mut bytes = 0_u64;
        for entry in std::fs::read_dir(&self.root)? {
            let entry = entry?;
            let metadata = entry.path().symlink_metadata()?;
            if !metadata.is_file() || metadata.file_type().is_symlink() {
                return Err(PersistentCacheError::UnsafeDirectory);
            }
            bytes = bytes.saturating_add(metadata.len());
        }
        Ok(bytes)
    }

    pub fn clear(&self) -> Result<(), PersistentCacheError> {
        let _lock = self.lock()?;
        self.used_bytes()?;
        for entry in std::fs::read_dir(&self.root)? {
            let entry = entry?;
            if entry.file_name() != ".lock" {
                std::fs::remove_file(entry.path())?;
            }
        }
        Ok(())
    }
}
