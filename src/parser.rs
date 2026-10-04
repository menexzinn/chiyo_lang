use crate::token::{Token, TokenType};
use crate::ast::Stmt;

pub struct Parser {
    tokens: Vec<Token>,
    pointer: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pointer: 0,
        }
    }

    fn token_at(&self) -> Token {
        self.tokens[self.pointer].clone()
    }

    fn advance_pointer(&mut self) {
        self.pointer += 1;
    }

    // statement call expression
    // expression call primary expression
    // makes sense for me

    // prescidence order!:
    // primary expression -> unary -> multiplication -> addition ->
    // comparison -> logic -> function call -> assignment expression

    // for now we just focus primary -> multiplication -> addition
    // but first just primary -> addition
    fn parse_stmt(&mut self) -> Stmt {
        match self.token_at().token_type() {
            TokenType::Let => self.parse_variable_declaration(false),
            TokenType::Const => self.parse_variable_declaration(true),
            _ => self.parse_expr(),
        }
    }

    fn parse_variable_declaration(&mut self, is_constant: bool) -> Stmt {
        self.advance_pointer(); // consume "let" or "const"

        let Stmt::Identifier{ value: name } = self.parse_expr() else {
            panic!("Name of the variable should be a word or sum");
        };

        match self.token_at().token_type() {
            TokenType::Equal => self.advance_pointer(),
            _ => panic!("You shall use \"=\" to declare a variable"),
        };

        let value = self.parse_expr();

        Stmt::VariableDeclaration {
            name,
            value: Box::new(value),
            is_constant,
        }
    }

    fn parse_expr(&mut self) -> Stmt {
        self.parse_additive_expr()
    }

    fn parse_additive_expr(&mut self) -> Stmt {
        let mut left = self.parse_multiplicative_expr(); // Identifier or Numeric Literal

        loop {
            let operator = match self.token_at().token_type() {
                TokenType::Plus => '+',
                TokenType::Minus => '-',
                _ => break,
            };
            self.advance_pointer();

            left = Stmt::BinaryExpr {
                operator,
                lhs: Box::new(left),
                rhs: Box::new(self.parse_multiplicative_expr()),
            };
        }

        left
    }

    fn parse_multiplicative_expr(&mut self) -> Stmt {
        let mut left = self.parse_primary_expr();

        loop {
            let operator: char = match self.token_at().token_type() {
                TokenType::Star => '*',
                TokenType::Slash => '/',
                _ => break,
            };
            self.advance_pointer();

            left = Stmt::BinaryExpr {
                operator,
                lhs: Box::new(left),
                rhs: Box::new(self.parse_primary_expr()),
            };
        }

        left
    }

    fn parse_primary_expr(&mut self) -> Stmt {
        let primary_expr = match self.token_at().token_type() {
            TokenType::Ident(name) => Stmt::Identifier{ value: name },
            TokenType::Number(number) => Stmt::NumericLiteral{ value: number },
            TokenType::ParenLeft => {
                self.advance_pointer();
                let value = self.parse_expr();
                match self.token_at().token_type() {
                    TokenType::ParenRight => value,
                    _ => panic!("Error: parentheses not closed at line {}", self.token_at().line()),
                }
            },
            _ => {
                panic!("Error: found an unexpected token at line {}", self.token_at().line());
            },
        };
        self.advance_pointer();
        primary_expr
    }

    pub fn generate_ast(&mut self) -> Stmt {
        let mut body: Vec<Box<Stmt>> = Vec::new();

        loop {
            if let TokenType::EOF = self.token_at().token_type() { break };
            let stmt = self.parse_stmt();
            body.push(Box::new(stmt));
        }

        Stmt::Program{ body }
    }
}
