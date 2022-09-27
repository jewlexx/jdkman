use std::{
    fs::{read_dir, DirEntry},
    io::Error,
    path::Path,
};

#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("std io error")]
    IOError(#[from] std::io::Error),
    #[error("found dir in bin path")]
    FoundDirInBinPath,
}

fn test_entry(entry: &DirEntry) -> Result<bool, Error> {
    Ok(!entry.metadata()?.is_dir())
}

pub fn symlink_dir(path: impl AsRef<Path>, target: impl AsRef<Path>) -> Result<(), PlatformError> {
    let dir = read_dir(path)?.collect::<Result<Vec<_>, _>>()?;

    if dir.iter().any(|entry| test_entry(entry).unwrap_or(false)) {
        return Err(PlatformError::FoundDirInBinPath);
    }

    for entry in dir {
        let path = entry.path();
        let file_name = path.file_name().expect("invalid file name");
        let target = target.as_ref().join(file_name);
        std::os::unix::fs::symlink(path, target)?;
    }

    Ok(())
}
