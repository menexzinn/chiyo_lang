pub mod utils;
pub mod frontend;
pub mod optimizer;
pub mod codegen;
pub mod dst;

use utils::file::read_file_by_input;
use utils::command::{assemble, link};
use frontend::{Scanner, Parser};
use optimizer::Optimizer;
use codegen::Generator;

fn main() {
    let source = read_file_by_input();

    let mut scanner = Scanner::new();
    let tokens = match scanner.scan_tokens(source) {
        Ok(tks) => tks,
        Err((error, line)) => {
            error.handle_error(line, "tokenization");
            unreachable!();
        },
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.generate_ast() {
        Ok(prog) => prog,
        Err((error, line)) => {
            error.handle_error(line, "parsing");
            unreachable!();
        },
    };

    let optimized = Optimizer::new().optimize(program);

    let mut code_generator = Generator::new();
    code_generator.generate_code(optimized).expect("Something ain\'t braining");

    // "avengers assemble" ahh function
    assemble();
    link();

    println!("The executable file \'main\' was generated successfully! :D");
}
