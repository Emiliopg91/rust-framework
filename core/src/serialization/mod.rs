pub mod error;
pub mod json;
pub mod toml;
pub mod yaml;

use std::any::type_name;

use serde::{Serialize, de::DeserializeOwned};

use crate::{
    serialization::error::{Result, SerializationError},
    utils::file::FileUtils,
};

pub trait Serializer {
    fn serialize<S: Serialize>(obj: &S) -> Result<String>;
    fn deserialize<T: DeserializeOwned>(string: &str) -> Result<T>;

    fn get_type() -> String;

    fn serialize_to_file<S, P>(obj: &S, path: P) -> impl Future<Output = Result<()>> + Send
    where
        P: AsRef<std::path::Path> + Send,
        S: Serialize + Send + Sync,
    {
        let path = path.as_ref().to_owned();

        async move {
            let content = Self::serialize(obj)?;
            FileUtils::write(&path, false, &content).await.map_err(|e| {
                SerializationError::MarshallError(Self::get_type().to_string(), Box::new(e))
            })
        }
    }

    fn deserialize_from_file<T, P>(path: &P) -> impl Future<Output = Result<T>> + Send
    where
        T: DeserializeOwned,
        P: AsRef<std::path::Path> + Send + Sync,
    {
        async move {
            Self::deserialize(&FileUtils::read(path).await.map_err(|e| {
                SerializationError::UnmarshallError(
                    Self::get_type(),
                    type_name::<T>().to_string(),
                    Box::new(e),
                )
            })?)
        }
    }
}
