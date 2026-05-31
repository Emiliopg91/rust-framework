#[cfg(test)]
mod tests;

pub mod errors;

use std::{
    ops::{Deref, DerefMut},
    path::Path,
    sync::{Arc, LazyLock},
};

use serde::{Serialize, de::DeserializeOwned};

use crate::{
    logger::{Logger, provider::Provider},
    serialization::{Serializer, yaml::YamlSerializer},
    settings::errors::{Result, SettingsError},
    utils::file::FileUtils,
};

#[derive(Default)]
pub struct Settings<T>
where
    T: Default + Serialize + DeserializeOwned,
{
    value: T,
}

impl<T> Deref for Settings<T>
where
    T: Default + Serialize + DeserializeOwned,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Settings<T>
where
    T: Default + Serialize + DeserializeOwned,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

static LOGGER: LazyLock<Arc<Logger>> = LazyLock::new(|| Provider::get_logger("Settings"));

impl<T> Settings<T>
where
    T: Default + Serialize + DeserializeOwned,
{
    pub async fn load<P>(file_path: &P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file_path = file_path.as_ref();
        LOGGER.info(format!("Loading settings from {}...", file_path.display()));
        let content = FileUtils::read(file_path)
            .await
            .map_err(|e| SettingsError::LoadError(file_path.to_path_buf(), Box::new(e)))?;
        Ok(Self {
            value: YamlSerializer::deserialize(&content)
                .map_err(|e| SettingsError::LoadError(file_path.to_path_buf(), Box::new(e)))?,
        })
    }

    pub async fn save<P>(&self, file_path: &P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let file_path = file_path.as_ref();
        LOGGER.info(format!("Saving settings to {}...", file_path.display()));
        let content = YamlSerializer::serialize(&self.value)
            .map_err(|e| SettingsError::SaveError(file_path.to_path_buf(), Box::new(e)))?;
        FileUtils::write(&file_path, false, &content)
            .await
            .map_err(|e| SettingsError::SaveError(file_path.to_path_buf(), Box::new(e)))?;
        Ok(())
    }
}
