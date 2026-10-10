pub mod utils;
pub mod frontend;

use utils::file::read_file;
use frontend::{Scanner, Parser};

fn main() {
    let source = read_file("main.chy");

    let mut scanner = Scanner::new();
    let tokens = scanner.scan_tokens(source);

    let mut parser = Parser::new(tokens);
    let program = parser.generate_ast();
    println!("{:#?}", program);
}
