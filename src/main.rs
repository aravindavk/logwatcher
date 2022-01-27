use std::env::args;
use std::process::exit;

extern crate logwatcher;
use logwatcher::{LogWatcher, LogWatcherAction, LogWatcherEvent};

fn main() {
    let filename = match args().nth(1) {
        Some(x) => x,
        None => {
            println!("File name required");
            exit(1);
        }
    };

    let mut log_watcher = LogWatcher::register(filename).unwrap();

    log_watcher.watch(&mut move |result| {
        match result {
            Ok(event) => match event {
                LogWatcherEvent::Line(line) => {
                    println!("Line {}", line);
                }
                LogWatcherEvent::LogRotation => {
                    println!("Logfile rotation");
                }
            },
            Err(err) => {
                println!("Error {}", err);
            }
        }
        LogWatcherAction::None
    });
}
