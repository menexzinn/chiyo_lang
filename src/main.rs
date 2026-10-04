pub mod file;
pub mod token;
pub mod scanner;
pub mod ast;
pub mod parser;

fn main() {
    let source = file::read_file("main.chy");

    let mut scanner = scanner::Scanner::new();
    let tokens = scanner.scan_tokens(source);

    let mut parser = parser::Parser::new(tokens);
    let program = parser.generate_ast();
    println!("{:#?}", program);
}
