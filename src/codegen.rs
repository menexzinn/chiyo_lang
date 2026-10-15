use crate::frontend::{Stmt, BinaryOperator};

// TODO: move this file handling to file utils
use std::fs::File;
use std::io::{Result, Write};

const FILE_PATH: &str = "main.asm";

pub struct Generator {
    buffer: String,
}

impl Generator {
    pub fn new() -> Self {
        Self { buffer: String::new() }
    }

    fn write_str(&mut self, line: &str) {
        self.buffer.push_str(line);
    }

    fn generate_number(&mut self, value: u32) {
        self.write_str(&format!("  push {}\n", value));
    }

    fn generate_binary_expression(&mut self, operator: BinaryOperator, lhs: Stmt, rhs: Stmt) {
        match lhs {
            Stmt::NumericLiteral { value } => self.generate_number(value),
            Stmt::BinaryExpression {
                operator: ino,
                lhs: inl,
                rhs: inr,
            } => self.generate_binary_expression(ino, *inl, *inr),
            Stmt::Identifier { value: _ } => todo!(),
            _ => unimplemented!(),
        };

        match rhs {
            Stmt::NumericLiteral { value } => self.generate_number(value),
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
        self.write_str(&format!("  {op} rax, rcx\n")); // operate
        self.write_str("  push rax\n"); // push value to stack
    }

    fn handle_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Identifier { value } => todo!(),
            Stmt::NumericLiteral { value } => self.generate_number(value),
            Stmt::BinaryExpression { operator, lhs, rhs } => {
                self.generate_binary_expression(operator, *lhs, *rhs);
            },
            Stmt::VariableDeclaration { is_constant: _, name: _, var_type: _, value } => {
                self.handle_stmt(*value);
            },
            Stmt::FunctionDeclaration {
                name: _,
                return_type: _,
                args: _,
                body: _,
            } => todo!(),
            _ => unimplemented!(),
        };
    }

    pub fn generate_code(&mut self, program: Stmt) -> Result<()> {
        let body = match program {
            Stmt::Program { body } => body,
            _ => unreachable!(),
        };

        self.write_str("global _start\n");
        self.write_str("_start:\n");

        for stmt in body {
            self.handle_stmt(*stmt);
        }

        self.write_str("  mov rax, 60\n");
        self.write_str("  pop rdi\n");
        self.write_str("  syscall\n");

        let bytes = self.buffer.as_bytes();

        let mut file = File::create(FILE_PATH)?;
        file.write_all(bytes)?;
        Ok(())
    }
}
