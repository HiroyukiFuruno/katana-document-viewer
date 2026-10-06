use super::{PersistentCacheError, PersistentDocumentCache, key::CacheKey};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const MAGIC: &[u8; 8] = b"KDVC0001";
const HEADER_BYTES: u64 = 72;
const LEGACY_MAX_ARTIFACT_BYTES: u64 = 128 * 1024 * 1024;
const MAX_ARTIFACT_BYTES: u64 = super::payload_codec::MAX_PAYLOAD_BYTES as u64
    + super::payload_codec::MAX_METADATA_BYTES as u64
    + super::payload_codec::PREFIX_BYTES
    + HEADER_BYTES;

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
        let metadata = path.symlink_metadata();
        if metadata
            .as_ref()
            .is_ok_and(|metadata| !metadata.is_file() || metadata.file_type().is_symlink())
        {
            return Err(PersistentCacheError::UnsafeDirectory);
        }
        if self.used_bytes()? > self.max_bytes {
            return Err(PersistentCacheError::Capacity);
        }
        let metadata = match metadata {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        Self::read_entry(&path, key, metadata.len(), self.max_bytes).map(Some)
    }

    fn read_entry(
        path: &std::path::Path,
        key: &str,
        length: u64,
        max_bytes: u64,
    ) -> Result<Vec<u8>, PersistentCacheError> {
        reject_unframed_oversized(path, length)?;
        if length > max_bytes.min(MAX_ARTIFACT_BYTES) {
            return Err(PersistentCacheError::Capacity);
        }
        if length < HEADER_BYTES {
            return Err(PersistentCacheError::Corrupt);
        }
        let mut bytes = Vec::new();
        File::open(path)?
            .take(max_bytes.min(MAX_ARTIFACT_BYTES) + 1)
            .read_to_end(&mut bytes)?;
        Self::decode(bytes, key)
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
        validate_entry_payload(payload, bytes.len() as u64)?;
        Ok(payload.to_vec())
    }

    pub(super) fn save(&self, key: &str, bytes: &[u8]) -> Result<(), PersistentCacheError> {
        let _lock = self.lock()?;
        let required = (bytes.len() as u64).saturating_add(HEADER_BYTES);
        if required > MAX_ARTIFACT_BYTES {
            return Err(PersistentCacheError::Capacity);
        }
        validate_entry_payload(bytes, required)?;
        let destination = self.root.join(key);
        match destination.symlink_metadata() {
            Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => {
                return Ok(());
            }
            Ok(_) => return Err(PersistentCacheError::UnsafeDirectory),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        if self.used_bytes()?.saturating_add(required) > self.max_bytes {
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

fn reject_unframed_oversized(
    path: &std::path::Path,
    length: u64,
) -> Result<(), PersistentCacheError> {
    if length <= LEGACY_MAX_ARTIFACT_BYTES {
        return Ok(());
    }
    let mut magic = [0_u8; 8];
    File::open(path)?.read_exact(&mut magic)?;
    if magic == *MAGIC {
        Ok(())
    } else {
        Err(PersistentCacheError::Capacity)
    }
}

fn validate_entry_payload(bytes: &[u8], required: u64) -> Result<(), PersistentCacheError> {
    if required > LEGACY_MAX_ARTIFACT_BYTES && !super::payload_codec::validate_framed_payload(bytes)
    {
        return Err(PersistentCacheError::Capacity);
    }
    Ok(())
}
