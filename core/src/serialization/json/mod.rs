#[cfg(test)]
mod tests;

use std::any::type_name;

use serde::{Serialize, de::DeserializeOwned};

use crate::serialization::{
    Serializer,
    error::{Result, SerializationError},
};

pub struct JsonSerializer;

impl Serializer for JsonSerializer {
    fn deserialize<T: DeserializeOwned>(string: &str) -> Result<T> {
        serde_json::from_str(string).map_err(|e| {
            SerializationError::UnmarshallError(
                Self::get_type(),
                type_name::<T>().to_string(),
                Box::new(e),
            )
        })
    }

    fn serialize<T: Serialize>(obj: &T) -> Result<String> {
        serde_json::to_string(obj)
            .map_err(|e| SerializationError::MarshallError(Self::get_type(), Box::new(e)))
    }

    fn get_type() -> String {
        "JSON".to_string()
    }
}
