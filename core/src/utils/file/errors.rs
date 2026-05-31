use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Could not delete {0}:\n  {1}")]
    FileDeletionError(PathBuf, std::io::Error),
    #[error("Could not touch {0}:\n  {1}")]
    FileTouchError(PathBuf, std::io::Error),
    #[error("Could not create directory '{0}':\n  {1}")]
    MakeDirError(PathBuf, std::io::Error),
    #[error("Could not write content to {0}:\n  {1}")]
    FileWriteError(PathBuf, Box<dyn std::error::Error>),
    #[error("Could not read content from {0}:\n  {1}")]
    FileReadError(PathBuf, Box<dyn std::error::Error>),
    #[error("Could not create tmp file:\n  {0}")]
    TmpFileError(std::io::Error),
    #[error("Could not open file '{0}':\n  {1}")]
    OpenFileError(String, std::io::Error),
}

pub type Result<T> = std::result::Result<T, FileError>;
