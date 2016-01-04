# Log Watcher

[![Build Status](https://travis-ci.org/aravindavk/logwatcher.svg?branch=master)](https://travis-ci.org/aravindavk/logwatcher)

A [Rust](https://www.rust-lang.org/) library to watch the log files.

Note: Tested only in Linux

### Features:
1. Automatically reloads log file when log rotated
2. Calls callback function when new line to parse

### Usage

First, add the following to your `Cargo.toml`

    [dependencies]
    logwatcher = "0.1"

Add to your code,

    extern crate logwatcher;
    use logwatcher::LogWatcher;

Create a callback function, which accepts String as input

    fn parse_line(line: String) {
        println!("Line {}", line);
    }

Register the logwatcher and watch it!

    let mut log_watcher = LogWatcher::register("/var/log/check.log".to_string()).unwrap();
    log_watcher.watch(parse_line);

