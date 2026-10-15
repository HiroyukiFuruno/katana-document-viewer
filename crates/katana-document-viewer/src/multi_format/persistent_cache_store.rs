use super::{PersistentCacheError, PersistentDocumentCache, key::CacheKey};
use cap_std::fs::{Dir, OpenOptions};
use std::fs::File;
use std::io::Read;

const MAGIC: &[u8; 8] = b"KDVC0001";
const HEADER_BYTES: u64 = 72;
const LEGACY_MAX_ARTIFACT_BYTES: u64 = 128 * 1024 * 1024;
const MAX_ARTIFACT_BYTES: u64 = super::payload_codec::MAX_PAYLOAD_BYTES as u64
    + super::payload_codec::MAX_METADATA_BYTES as u64
    + super::payload_codec::PREFIX_BYTES
    + HEADER_BYTES;

impl PersistentDocumentCache {
    pub(super) fn prepare(root: &std::path::Path) -> Result<(), PersistentCacheError> {
        std::fs::create_dir_all(root)?;
        let metadata = std::fs::symlink_metadata(root)?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(PersistentCacheError::UnsafeDirectory);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(root, std::fs::Permissions::from_mode(0o700))?;
        }
        Ok(())
    }

    pub(super) fn lock(&self) -> Result<File, PersistentCacheError> {
        self.root_identity.validate(&self.root)?;
        let directory = self.root_identity.directory();
        if directory
            .symlink_metadata(".lock")
            .is_ok_and(|m| !m.is_file() || m.file_type().is_symlink())
        {
            return Err(PersistentCacheError::UnsafeDirectory);
        }
        let file = directory
            .open_with(
                ".lock",
                OpenOptions::new()
                    .create(true)
                    .truncate(false)
                    .read(true)
                    .write(true),
            )?
            .into_std();
        file.lock()?;
        self.root_identity.validate(&self.root)?;
        Ok(file)
    }

    pub(super) fn load(&self, key: &str) -> Result<Option<Vec<u8>>, PersistentCacheError> {
        let _lock = self.lock()?;
        self.load_locked(key)
    }

    pub(super) fn load_locked(&self, key: &str) -> Result<Option<Vec<u8>>, PersistentCacheError> {
        let directory = self.root_identity.directory();
        let metadata = directory.symlink_metadata(key);
        if metadata
            .as_ref()
            .is_ok_and(|metadata| !metadata.is_file() || metadata.file_type().is_symlink())
        {
            return Err(PersistentCacheError::UnsafeDirectory);
        }
        if self.used_bytes_locked()? > self.max_bytes {
            return Err(PersistentCacheError::Capacity);
        }
        let metadata = match metadata {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        Self::read_entry(directory, key, metadata.len(), self.max_bytes).map(Some)
    }

    fn read_entry(
        directory: &Dir,
        key: &str,
        length: u64,
        max_bytes: u64,
    ) -> Result<Vec<u8>, PersistentCacheError> {
        reject_unframed_oversized(directory, key, length)?;
        if length > max_bytes.min(MAX_ARTIFACT_BYTES) {
            return Err(PersistentCacheError::Capacity);
        }
        if length < HEADER_BYTES {
            return Err(PersistentCacheError::Corrupt);
        }
        let mut bytes = Vec::new();
        directory
            .open(key)?
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
        self.save_locked(key, bytes)
    }

    pub(super) fn save_locked(&self, key: &str, bytes: &[u8]) -> Result<(), PersistentCacheError> {
        let required = (bytes.len() as u64).saturating_add(HEADER_BYTES);
        if required > MAX_ARTIFACT_BYTES {
            return Err(PersistentCacheError::Capacity);
        }
        validate_entry_payload(bytes, required)?;
        let directory = self.root_identity.directory();
        let existing = existing_regular_file(directory, key)?;
        let required = if existing { 0 } else { required };
        if self.used_bytes_locked()?.saturating_add(required) > self.max_bytes {
            return Err(PersistentCacheError::Capacity);
        }
        if existing {
            return Ok(());
        }
        super::temporary::publish(directory, key, bytes, MAGIC)?;
        Ok(())
    }
}

fn reject_unframed_oversized(
    directory: &Dir,
    key: &str,
    length: u64,
) -> Result<(), PersistentCacheError> {
    if length <= LEGACY_MAX_ARTIFACT_BYTES {
        return Ok(());
    }
    let mut magic = [0_u8; 8];
    directory.open(key)?.read_exact(&mut magic)?;
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

fn existing_regular_file(directory: &Dir, key: &str) -> Result<bool, PersistentCacheError> {
    match directory.symlink_metadata(key) {
        Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => Ok(true),
        Ok(_) => Err(PersistentCacheError::UnsafeDirectory),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}
