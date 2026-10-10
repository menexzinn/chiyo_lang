use std::fs;

pub fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Yo homie this file ain't existing")
}
