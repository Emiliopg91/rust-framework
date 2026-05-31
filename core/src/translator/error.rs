use thiserror::Error;

use crate::serialization::error::SerializationError;

#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("Could not load translations from '{0}':\n  {1}")]
    ErrorLoadingFile(String, SerializationError),
}

pub type Result<T> = std::result::Result<T, TranslationError>;
