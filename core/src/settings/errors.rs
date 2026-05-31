use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Could not load settings from {0}:\n  {1}")]
    LoadError(PathBuf, Box<dyn std::error::Error>),
    #[error("Could not save settings to {0}:\n  {1}")]
    SaveError(PathBuf, Box<dyn std::error::Error>),
}

pub type Result<T> = std::result::Result<T, SettingsError>;
