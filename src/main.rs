pub mod file;
pub mod token;
pub mod scanner;

use scanner::Scanner;
use file::read_file;

fn main() {
    let source = read_file("main.chy");

    let mut scanner = Scanner::new();
    let tokens = scanner.scan_tokens(source.to_string());

    for token in tokens {
        println!("{:?}", token);
    }
}
