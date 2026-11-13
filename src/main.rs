pub mod backend;
pub mod dst;
pub mod frontend;
pub mod utils;

use backend::Generator;
use frontend::{Parser, Scanner};
use utils::file::read_file_by_input;

fn main() {
    let source = read_file_by_input();

    let mut scanner = Scanner::new();
    let tokens = match scanner.scan_tokens(source) {
        Ok(tks) => tks,
        Err((error, line)) => {
            error.handle_error(line, "tokenization");
            unreachable!();
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.generate_ast() {
        Ok(prog) => prog,
        Err((error, line)) => {
            error.handle_error(line, "parsing");
            unreachable!();
        }
    };

    let mut code_generator = Generator::new();
    code_generator.generate_code(program);

    println!("INFO: generated assembly code file successfully");
}
