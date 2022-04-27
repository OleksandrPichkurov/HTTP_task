use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn log(message: String) {

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("server_logs.txt")
        .unwrap();

    if let Err(err) = writeln!(file, "{}", message) {
        eprintln!("{}", err);
    }
}