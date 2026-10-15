pub mod utils;
pub mod frontend;
pub mod codegen;

use utils::file::read_file;
use frontend::{Scanner, Parser};
use codegen::Generator;

// TODO: move process to utils
use std::process::Command;

fn main() {
    let source = read_file("main.chy");

    let mut scanner = Scanner::new();
    let tokens = match scanner.scan_tokens(source) {
        Ok(tks) => tks,
        Err((error, line)) => { error.handle_error(line); unreachable!(); },
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.generate_ast() {
        Ok(prog) => prog,
        Err((error, line)) => { error.handle_error(line); unreachable!(); },
    };

    let mut code_generator = Generator::new();
    code_generator.generate_code(program).expect("Something ain\'t braining");

    Command::new("nasm")
        .args(["-f", "elf64", "-o", "main.o", "main.asm"])
        .output()
        .expect("Could not generate object with \'nasm\' command!");

    Command::new("ld")
        .args(["-o", "main", "main.o"])
        .output()
        .expect("Could not generate executable with \'ld\' command!");

    println!("The executable file \'main\' was generated successfully! :D");
}
