use std::env::args;
use std::process::exit;

extern crate logwatcher;
use logwatcher::{LogWatcher, LogMsg};


fn parse_line(msg: LogMsg) {
    println!("Line {} {} {} {}", msg.filename, msg.inode, msg.pos, msg.line);
}

fn main(){
    let filename = match args().nth(1) {
        Some(x) => x,
        None => {
            println!("File name required");
            exit(1);
        }
    };
    let mut log_watcher = LogWatcher::register(filename, -1, 0).unwrap();
    log_watcher.watch(parse_line);
}
