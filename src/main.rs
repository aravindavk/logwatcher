use std::env::args;
use std::process::exit;

extern crate logwatcher;
use logwatcher::{LogWatcher, LogWatcherAction};

fn main() {
    let filename = match args().nth(1) {
        Some(x) => x,
        None => {
            println!("File name required");
            exit(1);
        }
    };

    let mut log_watcher = LogWatcher::register(filename).unwrap();

    log_watcher.watch(&mut move |line: String| {
        println!("Line {}", line);
        LogWatcherAction::None
    });
}
