#[cfg(test)]
mod tests;

pub mod error;

use std::{
    collections::HashMap,
    env, iter,
    path::Path,
    sync::{Arc, LazyLock, OnceLock},
};

use crate::{
    logger::{Logger, provider::Provider},
    serialization::{Serializer, yaml::YamlSerializer},
    translator::error::{Result, TranslationError},
};

static TRANSLATOR: OnceLock<Arc<Translator>> = OnceLock::new();
static LOGGER: LazyLock<Arc<Logger>> = LazyLock::new(|| Provider::get_logger("Settings"));

pub struct Translator {
    msg_map: HashMap<String, String>,
}

impl Translator {
    const DEFAULT_LANG: &str = "en";

    pub async fn initialize<T>(file_path: T) -> Result<()>
    where
        T: AsRef<Path> + Send + Sync,
    {
        let mut lang = env::var("LANG").unwrap_or(Self::DEFAULT_LANG.to_string());
        if lang == "C" {
            lang = Self::DEFAULT_LANG.to_string();
        }
        if lang.contains("_") {
            lang = lang.split("_").nth(0).unwrap().to_string();
        }

        LOGGER.info(format!(
            "Initializing translator for {} from {}...",
            lang.to_uppercase(),
            file_path.as_ref().display()
        ));

        let content_mapped = YamlSerializer::deserialize_from_file::<
            HashMap<String, HashMap<String, String>>,
            _,
        >(&file_path)
        .await
        .map_err(|e| {
            TranslationError::ErrorLoadingFile(file_path.as_ref().display().to_string(), e)
        })?;
        let mut msg_map: HashMap<String, String> = HashMap::new();

        for key in content_mapped.keys() {
            let entry = content_mapped.get(key).unwrap();
            let mut literal = entry.get(&lang);
            if literal.is_none() {
                literal = entry.get(Self::DEFAULT_LANG);
                if literal.is_none() {
                    literal = Some(key);
                }
            }
            msg_map.insert(key.clone(), literal.unwrap().clone());
        }
        LOGGER.info(format!("Loaded {} translations", msg_map.len()));

        let inst = Self { msg_map };
        TRANSLATOR.get_or_init(|| Arc::new(inst));
        Ok(())
    }

    pub fn translate<K: AsRef<str>>(key: K) -> String {
        Self::translate_with_data(key, iter::empty::<String>())
    }

    pub fn translate_with_data<K, D, I>(key: K, data: I) -> String
    where
        K: AsRef<str>,
        I: IntoIterator<Item = D>,
        D: AsRef<str>,
    {
        let translator = TRANSLATOR.get().unwrap();

        let key = key.as_ref();

        if !translator.msg_map.contains_key(key) {
            return key.to_string();
        }

        let mut result = translator
            .msg_map
            .get(key)
            .unwrap_or(&key.to_string())
            .to_owned();

        for item in data {
            result = result.replacen("{}", item.as_ref(), 1);
        }

        result
    }
}
