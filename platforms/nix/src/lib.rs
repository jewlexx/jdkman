use std::{
    fs::{read_dir, DirEntry},
    io::Error,
    path::Path,
};

use thiserror::Error as AsError;

#[derive(Debug, AsError)]
pub enum PlatformError {
    #[error("std io error")]
    IOError(#[from] std::io::Error),
    #[error("found dir in bin path")]
    FoundDirInBinPath,
}

fn test_entry(entry: Result<DirEntry, Error>) -> Result<bool, Error> {
    Ok(!entry?.metadata()?.is_dir())
}

pub fn symlink_dir(path: impl AsRef<Path>) -> Result<(), PlatformError> {
    let mut dir = read_dir(path)?;

    if dir.any(|entry| test_entry(entry).unwrap_or(false)) {
        return Err(PlatformError::FoundDirInBinPath);
    }

    Ok(())
}
