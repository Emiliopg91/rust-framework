use crate::logger::{level::Level, provider::Provider};

#[test]
fn test_01_generate_logger() {
    const LOGGER_NAME: &str = "LoggerName";
    let logger = Provider::get_logger(LOGGER_NAME);
    assert!(logger.name == LOGGER_NAME);
}

#[test]
fn test_02_level_management() {
    const LOGGER_NAME1: &str = "LoggerName1";
    const LOGGER_NAME2: &str = "LoggerName2";
    const NEW_LEVEL: Level = Level::DEBUG;
    let def_level: Level = Provider::get_level();

    assert!(Provider::get_logger(LOGGER_NAME1).get_level() == def_level);

    Provider::set_level(NEW_LEVEL);
    assert!(Provider::get_logger(LOGGER_NAME1).get_level() == def_level);
    assert!(Provider::get_logger(LOGGER_NAME2).get_level() == NEW_LEVEL);
}

#[test]
fn test_03_pattern_management() {
    const LOGGER_NAME1: &str = "LoggerName1";
    const LOGGER_NAME2: &str = "LoggerName3";
    const NEW_PATTERN: &str = "%m";
    let def_pattern = Provider::get_pattern();

    assert!(Provider::get_logger(LOGGER_NAME1).get_pattern() == def_pattern);

    Provider::set_pattern(NEW_PATTERN);
    assert!(Provider::get_logger(LOGGER_NAME1).get_pattern() == def_pattern);
    assert!(Provider::get_logger(LOGGER_NAME2).get_pattern() == NEW_PATTERN);

    Provider::set_pattern(&def_pattern);
}
