use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("Error marshalling value to {0}:\n  {1}")]
    MarshallError(String, Box<dyn std::error::Error>),
    #[error("Error unmarshalling {0} value to {1}:\n  {2}")]
    UnmarshallError(String, String, Box<dyn std::error::Error>),
}

pub type Result<T> = std::result::Result<T, SerializationError>;
