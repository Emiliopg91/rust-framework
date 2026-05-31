use std::{sync::Arc, thread::sleep, time::Duration};

use crate::{
    logger::{
        Logger,
        level::Level,
        sink::{Sink, file::FileSink},
    },
    utils::file::FileUtils,
};

const NAME: &str = "LoggerName";
const LEVEL: Level = Level::INFO;

fn initialize_logger() -> Logger {
    Logger::new(NAME, LEVEL, "[%d][%n][%l] - %m")
}

#[test]
fn test_01_test_constructor() {
    let logger = initialize_logger();
    assert_eq!(logger.get_level(), LEVEL);
    assert_eq!(logger.get_name(), NAME);
}

#[test]
fn test_02_test_set_level() {
    let new_level = Level::CRITICAL;
    let mut logger = initialize_logger();
    logger.set_level(new_level);
    assert_eq!(logger.get_level(), new_level);
}

#[test]
fn test_03_test_is_level_enabled() {
    let logger = initialize_logger();
    assert!(logger.is_level_enabled(Level::CRITICAL));
    assert!(logger.is_level_enabled(*logger.level.read().unwrap()));
    assert!(!logger.is_level_enabled(Level::DEBUG));
}

#[tokio::test]
async fn test_04_file_sink() {
    let path = FileUtils::new_tmp_file().unwrap();

    let mut logger = Logger::new(NAME, LEVEL, "[%d][%n][%l] - %m");
    let filesink: Arc<dyn Sink + Send + Sync> =
        Arc::new(FileSink::new(&path, *logger.level.read().unwrap(), 0).unwrap());

    logger.set_sinks(vec![filesink]);

    logger.info("Hello world");
    sleep(Duration::from_millis(100));
    assert!(
        FileUtils::read(&path)
            .await
            .unwrap()
            .contains("Hello world")
    )
}
