#[cfg(test)]
mod tests;

use std::any::type_name;

use serde::{Serialize, de::DeserializeOwned};

use crate::serialization::{
    Serializer,
    error::{Result, SerializationError},
};

pub struct TomlSerializer;

impl Serializer for TomlSerializer {
    fn deserialize<T: DeserializeOwned>(string: &str) -> Result<T> {
        toml::from_str(string).map_err(|e| {
            SerializationError::UnmarshallError(
                Self::get_type(),
                type_name::<T>().to_string(),
                Box::new(e),
            )
        })
    }

    fn serialize<T: Serialize>(obj: &T) -> Result<String> {
        toml::to_string(obj)
            .map_err(|e| SerializationError::MarshallError(Self::get_type(), Box::new(e)))
    }

    fn get_type() -> String {
        "TOML".to_string()
    }
}
