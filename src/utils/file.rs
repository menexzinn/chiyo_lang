use std::env;
use std::fs::{File, read_to_string};
use std::io::Write;

use crate::utils::error::Error;

pub fn read_file_by_input() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        Error::Other.panic("ERROR: incorrect number of arguments provided, expected a file");
    }

    let filename = &args[1];
    read_to_string(filename).expect("Yo homie this file ain't existing")
}

pub fn write_string_to_file(filename: &str, bytes: &[u8]) {
    let Ok(mut file) = File::create(filename) else {
        Error::Other.panic("ERROR: failed to create assembly file");
        unreachable!();
    };

    if let Err(_) = file.write_all(bytes) {
        Error::Other.panic("ERROR: failed to write contents to assembly file");
    };
}
