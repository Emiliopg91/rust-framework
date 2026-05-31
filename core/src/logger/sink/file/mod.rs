use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::mpsc::{self, RecvTimeoutError, SyncSender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::logger::level::Level;
use crate::logger::sink::Sink;

pub struct FileSink {
    level: Level,
    tx: Option<SyncSender<String>>,
    worker: Option<JoinHandle<()>>,
}

impl FileSink {
    pub fn new<T>(path: &T, level: Level, flush_interval: u64) -> std::io::Result<Self>
    where
        T: AsRef<Path>,
    {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        let (tx, rx) = mpsc::sync_channel(100);
        let tx = Some(tx);

        let path = path.as_ref().to_owned();
        let worker = Some(thread::spawn(move || {
            const POLL_TIMEOUT: Duration = Duration::from_millis(100);
            let flush_timeout: Duration = Duration::from_millis(flush_interval);
            let mut writer = BufWriter::new(file);
            let mut last_flush = Instant::now();
            let mut pending_flush = false;

            loop {
                match rx.recv_timeout(POLL_TIMEOUT) {
                    Ok(msg) => {
                        if let Err(e) = writeln!(writer, "{}", msg) {
                            eprintln!("Error writing log to {}:\n  {}", path.display(), e)
                        } else {
                            pending_flush = true;
                        }
                    }
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => {
                        let _ = writer.flush();
                        drop(writer);
                        break;
                    }
                }

                if pending_flush && (flush_interval == 0 || last_flush.elapsed() >= flush_timeout) {
                    let _ = writer.flush();
                    last_flush = Instant::now();
                    pending_flush = false;
                }
            }
        }));

        Ok(Self { level, tx, worker })
    }
}

impl Sink for FileSink {
    fn sink_message(&self, msg: &str, level: Level) {
        if level >= self.level
            && let Some(tx) = &self.tx
        {
            let _ = tx.send(msg.to_string());
        }
    }
}

impl Drop for FileSink {
    fn drop(&mut self) {
        if let Some(tx) = self.tx.take() {
            drop(tx);
        }

        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
