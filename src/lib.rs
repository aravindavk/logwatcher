use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::SeekFrom;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

pub use std::io::Error as LogWatcherError;

pub enum LogWatcherEvent {
    Line(String),
    LogRotation,
}

pub enum LogWatcherAction {
    None,
    SeekToEnd,
}

pub struct LogWatcher {
    filename: String,
    inode: u64,
    pos: u64,
    reader: BufReader<File>,
    finish: bool,
}

impl LogWatcher {
    pub fn register<P: AsRef<Path>>(filename: P) -> Result<LogWatcher, io::Error> {
        let f = match File::open(&filename) {
            Ok(x) => x,
            Err(err) => return Err(err),
        };

        let metadata = match f.metadata() {
            Ok(x) => x,
            Err(err) => return Err(err),
        };

        let mut reader = BufReader::new(f);
        let pos = metadata.len();
        reader.seek(SeekFrom::Start(pos)).unwrap();
        Ok(LogWatcher {
            filename: filename.as_ref().to_string_lossy().to_string(),
            inode: metadata.ino(),
            pos: pos,
            reader: reader,
            finish: false,
        })
    }

    fn reopen_if_log_rotated<F: ?Sized>(&mut self, callback: &mut F) -> bool
    where
        F: FnMut(std::result::Result<LogWatcherEvent, LogWatcherError>) -> LogWatcherAction,
    {
        loop {
            match File::open(&self.filename) {
                Ok(x) => {
                    let f = x;
                    let metadata = match f.metadata() {
                        Ok(m) => m,
                        Err(_) => {
                            sleep(Duration::new(1, 0));
                            continue;
                        }
                    };
                    if metadata.ino() != self.inode {
                        self.finish = true;
                        self.watch(callback);
                        self.finish = false;
                        self.reader = BufReader::new(f);
                        self.pos = 0;
                        self.inode = metadata.ino();
                        return true;
                    } else {
                        sleep(Duration::new(1, 0));
                    }
                    return false;
                }
                Err(err) => {
                    if err.kind() == ErrorKind::NotFound {
                        sleep(Duration::new(1, 0));
                        continue;
                    }
                }
            };
        }
    }

    fn handle_callback_action(&mut self, action: LogWatcherAction) {
        match action {
            LogWatcherAction::SeekToEnd => {
                self.reader.seek(SeekFrom::End(0)).unwrap();
            }
            LogWatcherAction::None => {}
        }
    }

    pub fn watch<F: ?Sized>(&mut self, callback: &mut F)
    where
        F: FnMut(std::result::Result<LogWatcherEvent, LogWatcherError>) -> LogWatcherAction,
    {
        loop {
            let mut line = String::new();
            let resp = self.reader.read_line(&mut line);
            match resp {
                Ok(len) => {
                    if len > 0 {
                        self.pos += len as u64;
                        self.reader.seek(SeekFrom::Start(self.pos)).unwrap();
                        let event = LogWatcherEvent::Line(line.replace("\n", ""));
                        self.handle_callback_action(callback(Ok(event)));
                        line.clear();
                    } else {
                        if self.finish {
                            break;
                        } else {
                            if self.reopen_if_log_rotated(callback) {
                                self.handle_callback_action(callback(Ok(
                                    LogWatcherEvent::LogRotation,
                                )));
                            }
                            self.reader.seek(SeekFrom::Start(self.pos)).unwrap();
                        }
                    }
                }
                Err(err) => {
                    self.handle_callback_action(callback(Err(err)));
                }
            }
        }
    }
}
