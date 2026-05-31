#[cfg(test)]
mod tests;

use std::{cmp::Ordering, str::FromStr};

#[derive(Debug)]
pub struct ParseLevelError(String);

impl std::fmt::Display for ParseLevelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid log level: {}", self.0)
    }
}

impl std::error::Error for ParseLevelError {}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Level {
    DEBUG = 10,
    INFO = 20,
    WARNING = 30,
    ERROR = 40,
    CRITICAL = 50,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lit = match self {
            Level::DEBUG => "DEBUG",
            Level::INFO => "INFO",
            Level::WARNING => "WARN",
            Level::ERROR => "ERROR",
            Level::CRITICAL => "CRIT",
        };

        write!(f, "{lit}")
    }
}

impl FromStr for Level {
    type Err = ParseLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEBUG" => Ok(Level::DEBUG),
            "INFO" => Ok(Level::INFO),
            "WARN" => Ok(Level::WARNING),
            "ERROR" => Ok(Level::ERROR),
            "CRIT" => Ok(Level::CRITICAL),
            _ => Err(ParseLevelError(s.to_string())),
        }
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}
