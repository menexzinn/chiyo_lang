use crate::dst::*;
use crate::frontend::{BinaryOperator, Stmt};
use crate::utils::error::Error;
use crate::utils::file::write_string_to_file;

use std::collections::HashMap;

const FILE_PATH: &str = "main.asm";

// GOOD PRACTICES I SHOULD FOLLOW SOMEDAY
// 1. RAX for math and return of functions
// 2. RCX for counter

#[derive(Clone)]
struct Condition {
    name: String,
    body: String,
}

impl Condition {
    pub fn new(name: String, body: String) -> Self {
        Self { name, body }
    }
}

pub struct Generator {
    stack_counter: usize,
    strings: HashMap<String, String>,
    variables: HashMap<String, usize>,

    condition_counter: usize,
    conditions: Vec<Condition>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            stack_counter: 0,
            strings: HashMap::new(),
            variables: HashMap::new(),

            condition_counter: 0,
            conditions: Vec::new(),
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
        self.stack_counter += 8;
        self.variables.insert(key, self.stack_counter);
    }

    fn get_variable(&self, key: &String) -> Option<&usize> {
        self.variables.get(key)
    }

    fn insert_condition(&mut self, condition: Condition) {
        self.condition_counter += 1;
        self.conditions.push(condition);
    }

    fn variable_exists(&self, key: &String) -> usize {
        match self.get_variable(&key) {
            Some(loc) => *loc,
            _ => {
                Error::panic(format!("Undefined variable {key}"));
                unreachable!()
            }
        }
    }

    fn generate_number<N: std::fmt::Display>(&mut self, buffer: &mut String, value: N) {
        self.write_str(buffer, format!("  push {}\n", value));
    }

    // TODO: accept unary expressions and boolean variable
    fn generate_conditional_expr(
        &mut self,
        buffer: &mut String,
        condition: Stmt,
        body: Vec<Box<Stmt>>,
    ) {
        let Stmt::BinaryExpression { operator, lhs, rhs } = condition else {
            Error::panic("Unexpected definition for condition");
            unreachable!();
        };

        let op = match operator {
            BinaryOperator::Equals => "je",
            BinaryOperator::NotEqual => "jne",
            BinaryOperator::Greater => "jg",
            BinaryOperator::GreaterEqual => "jge",
            BinaryOperator::Less => "jl",
            BinaryOperator::LessEqual => "jle",
            BinaryOperator::And => todo!(),
            BinaryOperator::Or => todo!(),
            _ => {
                Error::panic("Unexpected operator for condition");
                unreachable!();
            }
        };

        self.match_binary_expr_term(buffer, *lhs);
        self.match_binary_expr_term(buffer, *rhs);

        self.write_str(buffer, "  pop rsi\n"); // rhs
        self.write_str(buffer, "  pop rdi\n"); // lhs
        self.write_str(buffer, "  cmp rdi, rsi\n");
        let name = format!("C{}", self.condition_counter);
        self.write_str(buffer, format!("  {op} {name}\n"));

        let mut body_string = String::new();
        for stmt in body {
            self.handle_stmt(&mut body_string, *stmt);
        }
        self.insert_condition(Condition::new(name, body_string));
    }

    fn match_binary_expr_term(&mut self, buffer: &mut String, stmt: Stmt) {
        match stmt {
            Stmt::IntegerLiteral(value) => self.generate_number(buffer, value),
            Stmt::FloatLiteral(value) => self.generate_number(buffer, value),
            Stmt::BinaryExpression {
                operator: ino,
                lhs: inl,
                rhs: inr,
            } => self.generate_binary_expr(buffer, ino, *inl, *inr),
            Stmt::Identifier(name) => {
                let loc = self.variable_exists(&name);
                self.write_str(buffer, format!("  push [rbp - {loc}]\n"));
            }
            _ => unimplemented!(),
        };
    }
    fn generate_binary_expr(
        &mut self,
        buffer: &mut String,
        operator: BinaryOperator,
        lhs: Stmt,
        rhs: Stmt,
    ) {
        self.match_binary_expr_term(buffer, lhs);
        self.match_binary_expr_term(buffer, rhs);

        let op = match operator {
            BinaryOperator::Add => "add",
            BinaryOperator::Subtract => "sub",
            BinaryOperator::Multiply => "mul",
            BinaryOperator::Divide => "div",
            _ => unimplemented!(),
        };

        self.write_str(buffer, "  pop rsi\n"); // second value (top of stack)
        self.write_str(buffer, "  pop rdi\n"); // first value (bottom of stack)
        self.write_str(buffer, format!("  {op} rdi, rsi\n")); // operate
        self.write_str(buffer, "  push rdi\n"); // push value to stack
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
                Stmt::Identifier(name) => match self.strings.get(&name) {
                    Some(_) => Stmt::Identifier(name),
                    _ => match self.variables.get(&name) {
                        Some(loc) => Stmt::FloatLiteral(*loc as f32),
                        _ => {
                            Error::panic(format!("Undefined variable {name}"));
                            unreachable!();
                        }
                    },
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
        match stmt {
            Stmt::IntegerLiteral(value) => self.generate_number(buffer, value),
            Stmt::FloatLiteral(value) => self.generate_number(buffer, value),
            Stmt::Identifier(name) => {
                let loc = self.variable_exists(&name);
                self.write_str(buffer, format!("  mov rdi, [rbp - {loc}]\n"));
            }
            Stmt::BinaryExpression { operator, lhs, rhs } => {
                self.generate_binary_expr(buffer, operator, *lhs, *rhs);
            }
            Stmt::VariableDeclaration {
                name,
                var_type: _,
                value,
            } => match *value {
                Stmt::StringLiteral(value) => {
                    self.insert_string(name, value);
                }
                _ => {
                    self.insert_variable(name);
                    self.handle_stmt(buffer, *value);
                }
            },
            Stmt::VariableAssignment { name, value } => {
                let loc = self.variable_exists(&name);
                self.handle_stmt(buffer, *value);
                self.write_str(buffer, format!("  pop [rbp - {loc}]\n"));
            }
            Stmt::ConditionalExpression { condition, body } => {
                self.generate_conditional_expr(buffer, *condition, body)
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

        for condition in self.conditions.clone() {
            self.write_str(&mut buffer, format!("{}:\n", condition.name));
            self.write_str(&mut buffer, condition.body);
            self.write_str(&mut buffer, "  ret\n");
        }

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
