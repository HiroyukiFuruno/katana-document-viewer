use super::{PersistentCacheError, PersistentDocumentCache};

impl PersistentDocumentCache {
    pub fn used_bytes(&self) -> Result<u64, PersistentCacheError> {
        let _lock = self.lock()?;
        self.used_bytes_locked()
    }

    pub(super) fn used_bytes_locked(&self) -> Result<u64, PersistentCacheError> {
        let mut bytes = 0_u64;
        for entry in self.root_identity.directory().entries()? {
            let entry = entry?;
            let metadata = self
                .root_identity
                .directory()
                .symlink_metadata(entry.file_name())?;
            if !metadata.is_file() || metadata.file_type().is_symlink() {
                return Err(PersistentCacheError::UnsafeDirectory);
            }
            bytes = bytes.saturating_add(metadata.len());
        }
        Ok(bytes)
    }

    pub fn clear(&self) -> Result<(), PersistentCacheError> {
        let _lock = self.lock()?;
        self.clear_locked()
    }

    pub(super) fn clear_locked(&self) -> Result<(), PersistentCacheError> {
        self.used_bytes_locked()?;
        for entry in self.root_identity.directory().entries()? {
            let entry = entry?;
            if entry.file_name() != ".lock" {
                entry.remove_file()?;
            }
        }
        Ok(())
    }
}
