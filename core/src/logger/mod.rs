pub mod level;
pub mod provider;
pub mod sink;

#[cfg(test)]
mod tests;

use std::sync::{Arc, RwLock};

use chrono::Local;

use crate::logger::{
    level::Level,
    sink::{Sink, stdout::StdoutSink},
};

type TokenFn = fn(message: &str, name: &str, level: Level) -> String;

pub struct Logger {
    name: String,
    level: RwLock<Level>,
    sinks: Vec<Arc<dyn Sink + Send + Sync>>,
    pattern_orig: String,
    pattern: String,
    token_fn: Vec<TokenFn>,
}

impl Logger {
    pub fn new(name: &str, level: Level, pattern: &str) -> Self {
        let mut chars = pattern.chars().peekable();
        let mut token_fn: Vec<TokenFn> = Vec::new();
        let mut pattern_fmt = String::new();

        while let Some(c) = chars.next() {
            if c == '%' {
                match chars.peek() {
                    Some('%') => {
                        // Escaped %
                        chars.next();
                        pattern_fmt.push('%');
                    }
                    Some(next) => {
                        let func: Option<TokenFn> = match next {
                            'd' => Some(|_, _, _| {
                                Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string()
                            }),
                            'n' => Some(|_, name, _| format!("{:15.15}", name)),
                            'l' => Some(|_, _, level| level.to_string()),
                            'm' => Some(|msg, _, _| msg.to_string()),
                            _ => None,
                        };
                        match func {
                            Some(f) => {
                                pattern_fmt.push_str(&format!("{{{}}}", token_fn.len()));
                                token_fn.push(f);
                                chars.next();
                            }
                            None => pattern_fmt.push(*next),
                        }
                    }
                    None => {
                        pattern_fmt.push('%');
                    }
                }
            } else {
                pattern_fmt.push(c);
            }
        }

        Self {
            name: name.to_string(),
            pattern_orig: pattern.to_string(),
            pattern: pattern_fmt,
            token_fn,
            level: RwLock::new(level),
            sinks: vec![Arc::new(StdoutSink::new(level))],
        }
    }

    pub fn get_sinks(&self) -> Vec<Arc<dyn Sink + Send + Sync>> {
        self.sinks.clone()
    }

    pub fn set_sinks(&mut self, sinks: Vec<Arc<dyn Sink + Send + Sync>>) {
        self.sinks = sinks;
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_pattern(&self) -> &str {
        self.pattern_orig.as_ref()
    }

    pub fn get_level(&self) -> Level {
        *self.level.read().unwrap()
    }

    pub fn set_level(&mut self, level: Level) {
        *self.level.write().unwrap() = level;
    }

    pub fn is_level_enabled(&self, level: Level) -> bool {
        *self.level.read().unwrap() <= level
    }

    pub fn debug<T>(&self, message: T)
    where
        T: AsRef<str>,
    {
        self.log(message, Level::DEBUG);
    }

    pub fn info<T>(&self, message: T)
    where
        T: AsRef<str>,
    {
        self.log(message, Level::INFO);
    }

    pub fn warning<T>(&self, message: T)
    where
        T: AsRef<str>,
    {
        self.log(message, Level::WARNING);
    }

    pub fn error<T>(&self, message: T)
    where
        T: AsRef<str>,
    {
        self.log(message, Level::ERROR);
    }

    pub fn critical<T>(&self, message: T)
    where
        T: AsRef<str>,
    {
        self.log(message, Level::CRITICAL);
    }

    fn format_message(&self, message: &str, name: &str, level: Level) -> String {
        let mut formatted = self.pattern.clone();
        for (i, token_fn) in self.token_fn.iter().enumerate() {
            formatted = formatted.replace(&format!("{{{}}}", i), &token_fn(message, name, level));
        }
        formatted
    }

    fn log<T>(&self, message: T, level: Level)
    where
        T: AsRef<str>,
    {
        if self.is_level_enabled(level) {
            for sink in &self.sinks {
                sink.as_ref().sink_message(
                    &self.format_message(message.as_ref(), &self.name, level),
                    level,
                );
            }
        }
    }
}
