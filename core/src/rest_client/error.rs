use thiserror::Error;

use crate::serialization::error::SerializationError;

#[derive(Error, Debug)]
pub enum RestClientError {
    #[error("Error on connection to '{0}':\n  {1}")]
    ErrorSendingRequest(String, Box<dyn std::error::Error>),
    #[error("Error reading response:\n  {0}")]
    ErrorReadingResponse(Box<dyn std::error::Error>),
    #[error("Invalid request header \"{0}: {1}\":\n  {2}")]
    InvalidRequestHeader(String, String, Box<dyn std::error::Error>),
    #[error("Error serializing request body:\n  {0}")]
    RequestBodySerializationError(SerializationError),
}

pub type Result<T> = std::result::Result<T, RestClientError>;
