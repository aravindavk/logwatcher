use std::fs::File;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use std::os::unix::fs::MetadataExt;
use std::io::ErrorKind;

pub struct LogWatcher{
    filename: String,
    inode: u64,
    pos: u64,
    reader: BufReader<File>,
    finish: bool
}

pub struct LogMsg {
    pub filename: String,
    pub inode: u64,
    pub pos: u64,
    pub line: String,
}

impl LogWatcher {
    pub fn register(filename: String, file_inode: i64, start_pos: i64) -> Result<LogWatcher, io::Error> {
        let f = match File::open(filename.clone()) {
            Ok(x) => x,
            Err(err) => return Err(err)
        };
        
        let metadata = match f.metadata() {
            Ok(x) => x,
            Err(err) => return Err(err)
        };

        let mut reader = BufReader::new(f);
        let inode = metadata.ino();
        let pos = if start_pos == -1 {
            metadata.len()
        } else {
            if file_inode > -1 && file_inode == (inode as i64) {
                start_pos as u64
            } else {
                0
            }
        };
        reader.seek(SeekFrom::Start(pos)).unwrap();
        Ok(LogWatcher{filename: filename,
                      inode: inode,
                      pos: pos,
                      reader: reader,
                      finish: false})
    }

    fn reopen_if_log_rotated(&mut self, callback: fn (logmsg: LogMsg)){
        loop {
            match File::open(self.filename.clone()) {
                Ok(x) => {
                    let f = x;
                    let metadata = match f.metadata() {
                        Ok(m) => m,
                        Err(_) => {
                            sleep(Duration::new(1, 0));
                            continue;
                        }
                    };
                    if metadata.ino() != self.inode{
                        self.finish = true;
                        self.watch(callback);
                        self.finish = false;
                        println!("reloading log file");
                        self.reader = BufReader::new(f);
                        self.pos = 0;
                        self.inode = metadata.ino();
                    }
                    else{
                        sleep(Duration::new(1, 0));
                    }
                    break;
                },
                Err(err) => {
                    if err.kind() == ErrorKind::NotFound{
                        sleep(Duration::new(1, 0));
                        continue;
                    }
                }
            };
        }
    }

    pub fn watch(&mut self, callback: fn (logmsg: LogMsg)) {
        loop{
            let mut line = String::new();
            let resp = self.reader.read_line(&mut line);
            match resp{
                Ok(len) => {
                    if len > 0{
                        self.pos += len as u64;
                        self.reader.seek(SeekFrom::Start(self.pos)).unwrap();
                        callback(LogMsg{
                            filename: self.filename.clone(),
                            inode: self.inode.clone(),
                            pos: self.pos.clone(),
                            line: line.replace("\n", "")
                        });
                        line.clear();
                    }else {
                        if self.finish{
                            break;
                        }
                        else{
                            self.reopen_if_log_rotated(callback);
                            self.reader.seek(SeekFrom::Start(self.pos)).unwrap();
                        }
                    }
                },
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }
}


#[test]
fn it_works() {
}
