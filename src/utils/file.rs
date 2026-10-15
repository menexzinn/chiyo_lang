use std::env;
use std::fs;


pub fn read_file_by_input() -> String {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        panic!("ERROR: incorrect number of arguments provided, expected 1 file");
    }

    let filename = &args[1];
    fs::read_to_string(filename).expect("Yo homie this file ain't existing")
}
