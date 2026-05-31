use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("Unable to get working directory:\n  {0}")]
    InvalidCurrentDir(std::io::Error),
    #[error("Command '{0}' execution failed:\n  {1}")]
    CommandFailed(String, std::io::Error),
    #[error("Command '{0}' finished with exit status {1}")]
    NonZeroStatus(String, i32),
    #[error("Command '{0}' terminated by signal")]
    TerminatedBySignal(String),
    #[error("Command '{0}' not found")]
    CommandNotFound(String),
}

pub type Result<T> = std::result::Result<T, ShellError>;
