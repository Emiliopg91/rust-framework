pub mod file;
pub mod stdout;

use crate::logger::level::Level;

pub trait Sink: Send + Sync {
    fn sink_message(&self, msg: &str, level: Level);
}
