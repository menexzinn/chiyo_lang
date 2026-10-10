use crate::frontend::{Token, TokenType};
use crate::frontend::{Stmt, BinaryOperator, UnaryOperator};
use crate::utils::error::{Error, handle_error};

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
            handle_error(Error::VariableInvalidName, self.token_at().line());
            unreachable!();
        };

        match self.token_at().token_type() {
            TokenType::Colon => self.advance_pointer(),
            _ => {
                handle_error(Error::VariableMissingColon, self.token_at().line());
                unreachable!();
            },
        };

        let Stmt::Identifier{ value: var_type } = self.parse_expr() else {
            handle_error(Error::VariableMissingType, self.token_at().line());
            unreachable!();
        };

        match self.token_at().token_type() {
            TokenType::Equal => self.advance_pointer(),
            _ => {
                handle_error(Error::VariableMissingEquals, self.token_at().line());
                unreachable!();
            },
        };

        Stmt::VariableDeclaration {
            is_constant,
            name,
            var_type,
            value: Box::new(self.parse_expr()),
        }
    }

    fn parse_expr(&mut self) -> Stmt {
        self.parse_additive_expr()
    }

    fn parse_additive_expr(&mut self) -> Stmt {
        let mut left = self.parse_multiplicative_expr(); // Identifier or Numeric Literal

        loop {
            let operator = match self.token_at().token_type() {
                TokenType::Plus => BinaryOperator::Add,
                TokenType::Minus => BinaryOperator::Subtract,
                _ => break,
            };
            self.advance_pointer();

            left = Stmt::BinaryExpression {
                operator,
                lhs: Box::new(left),
                rhs: Box::new(self.parse_multiplicative_expr()),
            };
        }

        left
    }

    fn parse_multiplicative_expr(&mut self) -> Stmt {
        let mut left = self.parse_unary_expr();

        loop {
            let operator = match self.token_at().token_type() {
                TokenType::Star => BinaryOperator::Multiply,
                TokenType::Slash => BinaryOperator::Divide,
                _ => break,
            };
            self.advance_pointer();

            left = Stmt::BinaryExpression {
                operator,
                lhs: Box::new(left),
                rhs: Box::new(self.parse_unary_expr()),
            };
        }

        left
    }

    fn parse_unary_expr(&mut self) -> Stmt {
        let operator = match self.token_at().token_type() {
            TokenType::Minus => UnaryOperator::MathematicalNegative,
            TokenType::Not => UnaryOperator::LogicalNegation,
            _ => return self.parse_primary_expr(),
        };
        self.advance_pointer();

        Stmt::UnaryExpression {
            operator,
            rhs: Box::new(self.parse_primary_expr()),
        }
    }

    fn parse_primary_expr(&mut self) -> Stmt {
        let primary_expr = match self.token_at().token_type() {
            TokenType::Ident(name) => Stmt::Identifier{ value: name },

            TokenType::Number(number) => Stmt::NumericLiteral{ value: number },

            TokenType::True => Stmt::BooleanLiteral{ value: true },
            TokenType::False => Stmt::BooleanLiteral{ value: false },

            TokenType::Nil => Stmt::NullLiteral,

            TokenType::ParenLeft => {
                self.advance_pointer();
                let value = self.parse_expr();
                match self.token_at().token_type() {
                    TokenType::ParenRight => value,
                    _ => {
                        handle_error(Error::UnclosedParentheses, self.token_at().line());
                        unreachable!();
                    },
                }
            },

            _ => {
                handle_error(Error::UnrecognizableToken, self.token_at().line());
                unreachable!();
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
