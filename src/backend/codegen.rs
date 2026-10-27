use crate::dst::*;
use crate::frontend::{BinaryOperator, Stmt};
use crate::utils::error::Error;
use crate::utils::file::write_string_to_file;

use std::collections::HashMap;

const FILE_PATH: &str = "main.asm";

pub struct Generator {
    buffer: String,
    variables: HashMap<String, Stmt>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            variables: HashMap::new(),
        }
    }

    fn write_str<S: std::string::ToString>(&mut self, line: S) {
        self.buffer.push_str(&line.to_string());
    }

    fn insert_var(&mut self, key: String, value: Stmt) {
        self.variables.insert(key, value);
    }

    fn get_var(&self, key: &String) -> Option<&Stmt> {
        self.variables.get(key)
    }

    fn generate_number<N: std::fmt::Display>(&mut self, value: N) {
        self.write_str(format!("  push {}\n", value));
    }

    fn generate_binary_expression(&mut self, operator: BinaryOperator, lhs: Stmt, rhs: Stmt) {
        match lhs {
            Stmt::IntegerLiteral { value } => self.generate_number(value),
            Stmt::DecimalLiteral { value } => self.generate_number(value),
            Stmt::BinaryExpression {
                operator: ino,
                lhs: inl,
                rhs: inr,
            } => self.generate_binary_expression(ino, *inl, *inr),
            Stmt::Identifier { value: _ } => todo!(),
            _ => unimplemented!(),
        };

        match rhs {
            Stmt::IntegerLiteral { value } => self.generate_number(value),
            Stmt::DecimalLiteral { value } => self.generate_number(value),
            Stmt::BinaryExpression {
                operator: ino,
                lhs: inl,
                rhs: inr,
            } => self.generate_binary_expression(ino, *inl, *inr),
            Stmt::Identifier { value: _ } => todo!(),
            _ => unimplemented!(),
        };

        let op = match operator {
            BinaryOperator::Add => "add",
            BinaryOperator::Subtract => "sub",
            BinaryOperator::Multiply => "mul",
            BinaryOperator::Divide => "div",
            _ => unimplemented!(),
        };

        self.write_str("  pop rcx\n"); // second value (top of stack)
        self.write_str("  pop rax\n"); // first value (bottom of stack)
        self.write_str(format!("  {op} rax, rcx\n")); // operate
        self.write_str("  push rax\n"); // push value to stack
    }

    fn generate_function_call(&mut self, name: String, args: Vec<Box<Stmt>>, is_dst: bool) {
        if !is_dst {
            unimplemented!("For now you can use only library functions!")
        }

        let to_append = match name.as_str() {
            "exit" => exit(args),
            "print" => print(args),
            _ => unimplemented!(),
        };

        for line in to_append {
            self.write_str(line);
        }
    }

    fn handle_stmt(&mut self, stmt: Stmt) {
        println!("{:#?}", stmt);
        match stmt {
            Stmt::Identifier { value } => match self.get_var(&value) {
                Some(st) => self.handle_stmt(st.clone()),
                _ => Error::Other.panic("CODEGEN: variable not defined dude..."),
            },
            Stmt::IntegerLiteral { value } => self.generate_number(value),
            Stmt::DecimalLiteral { value } => self.generate_number(value),
            Stmt::BinaryExpression { operator, lhs, rhs } => {
                self.generate_binary_expression(operator, *lhs, *rhs);
            }
            Stmt::VariableDeclaration {
                name,
                var_type: _,
                value,
            } => {
                self.insert_var(name, *value.clone());
                self.handle_stmt(*value);
            }
            Stmt::FunctionDeclaration {
                name: _,
                return_type: _,
                args: _,
                body: _,
            } => unimplemented!(),
            Stmt::FunctionCall { name, args, is_dst } => {
                self.generate_function_call(name, args, is_dst)
            }
            Stmt::StringLiteral { value: _ } => (),
            _ => unimplemented!(),
        };
    }

    pub fn generate_code(&mut self, program: Stmt) {
        let Stmt::Program { body } = program else {
            unreachable!()
        };

        self.write_str("extern exit\n");
        self.write_str("extern print\n");

        self.write_str("section .text\n");
        self.write_str("global _start\n");
        self.write_str("_start:\n");

        // initialize stack or something like that
        self.write_str("  push rbp\n");
        self.write_str("  mov rbp, rsp\n");

        let _: () = body
            .into_iter()
            .map(|stmt| self.handle_stmt(*stmt))
            .collect();

        // undo stack (I think that's what it does) and exit with success
        self.write_str("  pop rbp\n");
        self.write_str("  mov rax, 60\n");
        self.write_str("  mov rdi, 0\n");
        self.write_str("  syscall\n");

        // variables' section baby
        self.write_str("section .data\n");

        for (key, value) in self.variables.clone().into_iter() {
            match value {
                Stmt::StringLiteral { value } => {
                    self.write_str(format!("  {key} db \'{value}\', 0x0a\n"));
                    self.write_str(format!("  {key}Len equ $ - {key}\n"));
                }
                Stmt::IntegerLiteral { value } => self.write_str(format!("  {key} dw {value}\n")),
                Stmt::DecimalLiteral { value } => self.write_str(format!("  {key} dw {value}\n")),
                _ => unimplemented!(),
            }
        }

        let bytes = self.buffer.as_bytes();
        write_string_to_file(FILE_PATH, bytes);
    }
}
