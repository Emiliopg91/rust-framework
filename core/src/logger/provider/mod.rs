#[cfg(test)]
mod tests;

use crate::logger::{
    Logger,
    level::Level,
    sink::{Sink, stdout::StdoutSink},
};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

pub struct Provider {
    default_level: Level,
    default_pattern: String,
    default_sinks: Vec<Arc<dyn Sink + Send + Sync>>,
    logger_map: HashMap<String, Arc<Logger>>,
    level_map: HashMap<String, Level>,
}

static PROVIDER: LazyLock<RwLock<Provider>> = LazyLock::new(|| RwLock::new(Provider::new()));

impl Provider {
    fn new() -> Self {
        Self {
            default_level: Level::INFO,
            logger_map: HashMap::new(),
            default_pattern: "[%d][%n][%l] - %m".to_string(),
            default_sinks: vec![Arc::new(StdoutSink::new(Level::INFO))],
            level_map: HashMap::new(),
        }
    }

    pub fn set_levels(levels: HashMap<String, Level>) {
        let mut w_prov = PROVIDER.write().unwrap();
        w_prov.level_map = levels.clone();
    }

    pub fn get_logger(logger_name: &str) -> Arc<Logger> {
        {
            let r_prov = PROVIDER.read().unwrap();
            if let Some(logger) = r_prov.logger_map.get(logger_name) {
                return logger.clone();
            }
        }

        let mut w_prov = PROVIDER.write().unwrap();

        if let Some(logger) = w_prov.logger_map.get(logger_name) {
            return logger.clone();
        }

        let mut level = w_prov.default_level;
        if let Some(lvl) = w_prov.level_map.get(logger_name) {
            level = *lvl;
        }

        let inst = Arc::new(Logger::new(logger_name, level, &w_prov.default_pattern));

        w_prov
            .logger_map
            .insert(logger_name.to_string(), inst.clone());

        inst
    }

    pub fn get_sinks() -> Vec<Arc<dyn Sink + Send + Sync>> {
        PROVIDER.read().unwrap().default_sinks.clone()
    }

    pub fn set_sinks(sinks: Vec<Arc<dyn Sink + Send + Sync>>) {
        PROVIDER.write().unwrap().default_sinks = sinks;
    }

    pub fn get_level() -> Level {
        PROVIDER.read().unwrap().default_level
    }

    pub fn set_level(level: Level) {
        PROVIDER.write().unwrap().default_level = level;
    }

    pub fn get_pattern() -> String {
        PROVIDER.read().unwrap().default_pattern.clone()
    }

    pub fn set_pattern(pattern: &str) {
        PROVIDER.write().unwrap().default_pattern = pattern.to_string();
    }
}
