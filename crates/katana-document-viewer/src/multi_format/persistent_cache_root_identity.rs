use super::PersistentCacheError;
use std::{fs::File, path::Path, sync::Arc};

#[derive(Debug, Clone)]
pub(super) struct DirectoryIdentity {
    file: Arc<File>,
    directory: Arc<cap_std::fs::Dir>,
}

impl DirectoryIdentity {
    pub(super) fn new(root: &Path) -> Result<Self, PersistentCacheError> {
        let metadata = root.symlink_metadata()?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(PersistentCacheError::UnsafeDirectory);
        }
        let directory = cap_std::fs::Dir::open_ambient_dir(root, cap_std::ambient_authority())?;
        let file = directory.try_clone()?.into_std_file();
        Ok(Self {
            file: Arc::new(file),
            directory: Arc::new(directory),
        })
    }

    pub(super) fn directory(&self) -> &cap_std::fs::Dir {
        &self.directory
    }

    pub(super) fn validate(&self, root: &Path) -> Result<(), PersistentCacheError> {
        let current = Self::new(root)?;
        if identity(&self.file)? != identity(&current.file)? {
            return Err(PersistentCacheError::UnsafeDirectory);
        }
        Ok(())
    }
}

#[cfg(unix)]
fn identity(file: &File) -> std::io::Result<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    let metadata = file.metadata()?;
    Ok((metadata.dev(), metadata.ino()))
}

#[cfg(windows)]
fn identity(file: &File) -> std::io::Result<(u64, [u8; 16])> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        FILE_ID_INFO, FileIdInfo, GetFileInformationByHandleEx,
    };
    let mut info = FILE_ID_INFO::default();
    // ReFSの128bit識別子も保持し、異なるdirectoryを同一実体と判定しない。
    let success = unsafe {
        GetFileInformationByHandleEx(
            file.as_raw_handle(),
            FileIdInfo,
            std::ptr::from_mut(&mut info).cast(),
            std::mem::size_of::<FILE_ID_INFO>() as u32,
        )
    };
    if success == 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok((info.VolumeSerialNumber, info.FileId.Identifier))
}
