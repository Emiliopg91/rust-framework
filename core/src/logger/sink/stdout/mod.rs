use crate::logger::{level::Level, sink::Sink};

pub struct StdoutSink {
    level: Level,
}

impl StdoutSink {
    pub fn new(level: Level) -> Self {
        Self { level }
    }

    fn get_color(level: Level) -> String {
        match level {
            Level::DEBUG => "36m".to_string(),
            Level::INFO => "32m".to_string(),
            Level::WARNING => "33m".to_string(),
            Level::ERROR => "31m".to_string(),
            Level::CRITICAL => "1;31m".to_string(),
        }
    }
}

impl Sink for StdoutSink {
    fn sink_message(&self, msg: &str, level: Level) {
        if level >= self.level {
            let msg = format!("\x1b[{}{}\x1b[0m", Self::get_color(level), msg);
            if level < Level::ERROR {
                println!("{}", msg)
            } else {
                eprintln!("{}", msg)
            }
        }
    }
}
