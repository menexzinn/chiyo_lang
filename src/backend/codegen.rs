use crate::dst::*;
use crate::frontend::{BinaryOperator, Stmt};
use crate::utils::error::Error;
use crate::utils::file::write_string_to_file;

use std::collections::HashMap;

const FILE_PATH: &str = "main.asm";

pub struct Generator {
    counter: usize,
    strings: HashMap<String, String>,
    variables: HashMap<String, usize>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            counter: 0,
            strings: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    fn write_str<S: std::string::ToString>(&mut self, buffer: &mut String, line: S) {
        buffer.push_str(&line.to_string());
    }

    fn insert_string(&mut self, key: String, value: String) {
        self.strings.insert(key, value);
    }

    fn get_string(&self, key: &String) -> Option<&String> {
        self.strings.get(key)
    }

    fn insert_variable(&mut self, key: String) {
        self.counter += 8;
        self.variables.insert(key, self.counter);
    }

    fn get_variable(&self, key: &String) -> Option<&usize> {
        self.variables.get(key)
    }

    fn generate_number<N: std::fmt::Display>(&mut self, buffer: &mut String, value: N) {
        self.write_str(buffer, format!("  push {}\n", value));
    }

    fn match_binary_expression_term(&mut self, buffer: &mut String, stmt: Stmt) {
        match stmt {
            Stmt::IntegerLiteral(value) => self.generate_number(buffer, value),
            Stmt::FloatLiteral(value) => self.generate_number(buffer, value),
            Stmt::BinaryExpression {
                operator: ino,
                lhs: inl,
                rhs: inr,
            } => self.generate_binary_expression(buffer, ino, *inl, *inr),
            Stmt::Identifier(name) => match self.get_variable(&name) {
                Some(offset) => self.write_str(buffer, format!("  push [rbp - {offset}]\n")),
                _ => Error::Other.panic(format!("Undefined variable {name}")),
            },
            _ => unimplemented!(),
        }
    }
    fn generate_binary_expression(
        &mut self,
        buffer: &mut String,
        operator: BinaryOperator,
        lhs: Stmt,
        rhs: Stmt,
    ) {
        self.match_binary_expression_term(buffer, lhs);
        self.match_binary_expression_term(buffer, rhs);

        let op = match operator {
            BinaryOperator::Add => "add",
            BinaryOperator::Subtract => "sub",
            BinaryOperator::Multiply => "mul",
            BinaryOperator::Divide => "div",
            _ => unimplemented!(),
        };

        self.write_str(buffer, "  pop rcx\n"); // second value (top of stack)
        self.write_str(buffer, "  pop rax\n"); // first value (bottom of stack)
        self.write_str(buffer, format!("  {op} rax, rcx\n")); // operate
        self.write_str(buffer, "  push rax\n"); // push value to stack
    }

    fn generate_function_call(
        &mut self,
        buffer: &mut String,
        name: String,
        args: Vec<Box<Stmt>>,
        is_dst: bool,
    ) {
        if !is_dst {
            unimplemented!("For now you can use only library functions!")
        }

        let args: Vec<Stmt> = args
            .into_iter()
            .map(|stmt| match *stmt {
                Stmt::Identifier(name) => match self.get_variable(&name) {
                    Some(value) => Stmt::FloatLiteral(*value as f32),
                    _ => {
                        Error::Other.panic("Undefined variable {name}");
                        unreachable!();
                    }
                },
                _ => *stmt,
            })
            .collect();
        let to_append = match name.as_str() {
            "exit" => exit(args),
            "print" => print(args),
            _ => unimplemented!(),
        };

        for line in to_append {
            self.write_str(buffer, line);
        }
    }

    // TODO: scopes...
    fn handle_stmt(&mut self, buffer: &mut String, stmt: Stmt) {
        println!("{:#?}", stmt);
        match stmt {
            Stmt::IntegerLiteral(value) => self.generate_number(buffer, value),
            Stmt::FloatLiteral(value) => self.generate_number(buffer, value),
            Stmt::Identifier(name) => match self.get_variable(&name) {
                Some(value) => self.write_str(buffer, format!("  mov rax, [rbp - {value}]\n")),
                None => Error::Other.panic(format!("Variable {name} not found")),
            },
            Stmt::BinaryExpression { operator, lhs, rhs } => {
                self.generate_binary_expression(buffer, operator, *lhs, *rhs);
            }
            Stmt::VariableDeclaration {
                name,
                var_type: _,
                value,
            } => {
                match *value {
                    Stmt::StringLiteral(value) => {
                        self.insert_string(name, value);
                    }
                    _ => {
                        self.insert_variable(name);
                        self.handle_stmt(buffer, *value);
                    }
                };
            }
            Stmt::FunctionDeclaration {
                name: _,
                return_type: _,
                args: _,
                body: _,
            } => unimplemented!(),
            Stmt::FunctionCall { name, args, is_dst } => {
                self.generate_function_call(buffer, name, args, is_dst)
            }
            Stmt::StringLiteral(_) => todo!(),
            _ => unimplemented!(),
        };
    }

    pub fn generate_code(&mut self, program: Stmt) {
        let Stmt::Program { body } = program else {
            unreachable!()
        };

        let mut buffer = String::new();

        self.write_str(&mut buffer, "extern exit\n");
        self.write_str(&mut buffer, "extern print\n");

        self.write_str(&mut buffer, "section .text\n");
        self.write_str(&mut buffer, "global _start\n");
        self.write_str(&mut buffer, "_start:\n");

        // initialize stack or something like that
        self.write_str(&mut buffer, "  push rbp\n");
        self.write_str(&mut buffer, "  mov rbp, rsp\n");

        let _: () = body
            .into_iter()
            .map(|stmt| self.handle_stmt(&mut buffer, *stmt))
            .collect();

        // undo stack (I think that's what it does) and exit with success
        self.write_str(&mut buffer, "  pop rbp\n");
        self.write_str(&mut buffer, "  mov rax, 60\n");
        self.write_str(&mut buffer, "  mov rdi, 0\n");
        self.write_str(&mut buffer, "  syscall\n");

        // variables' section baby
        self.write_str(&mut buffer, "section .data\n");

        for (key, value) in self.strings.clone().into_iter() {
            self.write_str(&mut buffer, format!("  {key} db \'{value}\', 0x0a\n"));
            self.write_str(&mut buffer, format!("  {key}Len equ $ - {key}\n"));
        }

        let bytes = buffer.as_bytes();
        write_string_to_file(FILE_PATH, bytes);
    }
}
