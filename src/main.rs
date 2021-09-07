extern crate logwatcher;

use logwatcher::{LogWatcher, LogWatcherAction, StartFrom};
use std::env::args;
use std::process::exit;

fn main() {
    let filename = match args().nth(1) {
        Some(x) => x,
        None => {
            println!("File name required");
            exit(1);
        }
    };

    let mut log_watcher = LogWatcher::register(StartFrom::End, filename).unwrap();

    log_watcher.watch(&mut move |pos: u64, len: usize, line: String| {
        println!("Pos #{}, len {} char, Line: `{}`", pos, len, line);
        LogWatcherAction::None
    });
}
