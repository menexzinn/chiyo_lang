use crate::frontend::{BinaryOperator, FunctionArgument, Stmt, UnaryOperator};
use crate::frontend::{Token, TokenType};
use crate::utils::error::{Error, Result};

pub struct Parser {
    tokens: Vec<Token>,
    pointer: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pointer: 0 }
    }

    fn token_at(&self) -> Token {
        self.tokens[self.pointer].clone()
    }

    fn token_peek_type(&self) -> TokenType {
        if self.pointer == self.tokens.len() {
            return TokenType::Nil;
        }
        self.tokens[self.pointer + 1].token_type().clone()
    }

    fn current_line(&self) -> u32 {
        self.token_at().line()
    }

    fn advance_pointer(&mut self) {
        self.pointer += 1;
    }

    fn expect_and_consume(&mut self, token: TokenType, error: Error) -> Result<()> {
        if token != self.token_at().token_type() {
            return Err((error, self.current_line()));
        }
        self.advance_pointer();
        Ok(())
    }

    // prescidence order!:
    // primary expression -> unary -> multiplication -> addition ->
    // comparison -> logic -> function call -> assignment expression

    // for now we just focus primary -> multiplication -> addition
    // but first just primary -> addition
    fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.token_at().token_type() {
            TokenType::Let => self.parse_variable_declaration(),
            TokenType::Func => self.parse_function_declaration(),
            TokenType::If => self.parse_if_expr(),
            _ => self.parse_expr(),
        }
    }

    // func function() {}
    // func function(arg: type) {}
    // func function(arg1: type1, arg2: type2, ...) {}
    // func function(): type {}
    fn parse_function_declaration(&mut self) -> Result<Stmt> {
        self.advance_pointer(); // consume "func"

        let Stmt::Identifier(name) = self.parse_expr()? else {
            return Err((Error::FunctionInvalidName, self.current_line()));
        };

        self.expect_and_consume(TokenType::ParenLeft, Error::MisplacedParentheses)?;

        // func function(arg1: type1, )
        let mut args: Vec<FunctionArgument> = Vec::new();
        loop {
            match self.token_at().token_type() {
                TokenType::ParenRight => break self.advance_pointer(),
                _ => (),
            };

            let Stmt::Identifier(arg_name) = self.parse_expr()? else {
                return Err((Error::FunctionArgumentMissingName, self.current_line()));
            };

            self.expect_and_consume(TokenType::Colon, Error::MissingColon)?;

            let Stmt::Identifier(arg_type) = self.parse_expr()? else {
                return Err((Error::FunctionArgumentMissingType, self.current_line()));
            };

            args.push(FunctionArgument { arg_name, arg_type });
            match self.token_at().token_type() {
                TokenType::ParenRight => break self.advance_pointer(),
                TokenType::Comma => {
                    self.advance_pointer();
                    match self.token_at().token_type() {
                        TokenType::ParenRight => {
                            return Err((Error::MisplacedParentheses, self.current_line()));
                        }
                        _ => (),
                    };
                }
                _ => return Err((Error::UnexpectedToken, self.current_line())),
            };
        }

        let return_type: Option<String> = match self.token_at().token_type() {
            TokenType::Colon => {
                self.advance_pointer();

                match self.parse_expr()? {
                    Stmt::Identifier(value) => Some(value),
                    _ => return Err((Error::FunctionInvalidType, self.current_line())),
                }
            }
            _ => None,
        };

        self.expect_and_consume(TokenType::BraceLeft, Error::MissingBraces)?;

        let mut body: Vec<Box<Stmt>> = Vec::new();
        loop {
            match self.token_at().token_type() {
                TokenType::BraceRight => break self.advance_pointer(),
                TokenType::EOF => return Err((Error::UnclosedBraces, self.current_line())),
                _ => body.push(Box::new(self.parse_stmt()?)),
            };
        }

        Ok(Stmt::FunctionDeclaration {
            name,
            return_type,
            args,
            body,
        })
    }

    fn parse_function_call(&mut self, is_dst: bool) -> Result<Stmt> {
        let name = match self.token_at().token_type() {
            TokenType::Exit => {
                self.advance_pointer();
                "exit".to_string()
            }
            TokenType::Print => {
                self.advance_pointer();
                "print".to_string()
            }
            _ => match self.parse_expr()? {
                Stmt::Identifier(value) => value,
                _ => return Err((Error::FunctionInvalidName, self.current_line())),
            },
        };
        self.expect_and_consume(TokenType::ParenLeft, Error::MisplacedParentheses)?;

        let mut args: Vec<Box<Stmt>> = Vec::new();
        loop {
            match self.token_at().token_type() {
                TokenType::ParenRight => break,
                _ => (),
            };

            args.push(Box::new(self.parse_expr()?));

            match self.token_at().token_type() {
                TokenType::Comma => {
                    self.advance_pointer();
                    match self.token_at().token_type() {
                        TokenType::ParenRight => {
                            return Err((Error::MisplacedParentheses, self.current_line()));
                        }
                        _ => (),
                    };
                }
                TokenType::ParenRight => (),
                _ => return Err((Error::UnexpectedToken, self.current_line())),
            };
        }
        Ok(Stmt::FunctionCall { name, args, is_dst })
    }

    fn parse_variable_declaration(&mut self) -> Result<Stmt> {
        self.advance_pointer(); // consume "let" or "const"

        let Stmt::Identifier(name) = self.parse_expr()? else {
            return Err((Error::VariableInvalidName, self.current_line()));
        };

        self.expect_and_consume(TokenType::Colon, Error::MissingColon)?;

        let var_type = match self.token_at().token_type() {
            TokenType::Ident(value) => value,
            TokenType::Nil => "nil".to_string(),
            _ => return Err((Error::VariableInvalidType, self.current_line())),
        };
        self.advance_pointer();

        self.expect_and_consume(TokenType::Equal, Error::VariableMissingEquals)?;

        Ok(Stmt::VariableDeclaration {
            name,
            var_type,
            value: Box::new(self.parse_expr()?),
        })
    }

    fn parse_variable_assignment(&mut self) -> Result<Stmt> {
        let TokenType::Ident(name) = self.token_at().token_type() else {
            return Err((Error::VariableInvalidName, self.current_line()));
        };
        self.advance_pointer();
        self.expect_and_consume(TokenType::Equal, Error::VariableMissingEquals)?;
        let value = Box::new(self.parse_expr()?);
        self.pointer -= 1;
        Ok(Stmt::VariableAssignment { name, value })
    }

    fn parse_expr(&mut self) -> Result<Stmt> {
        match self.token_at().token_type() {
            TokenType::Exit | TokenType::Print => self.parse_function_call(true),
            _ => self.parse_conditional_expr(),
        }
    }

    // TODO: add support for else blocks
    fn parse_if_expr(&mut self) -> Result<Stmt> {
        self.advance_pointer(); // consume if

        self.expect_and_consume(TokenType::ParenLeft, Error::MissingParentheses)?;
        let condition = Box::new(self.parse_conditional_expr()?);
        self.expect_and_consume(TokenType::ParenRight, Error::UnclosedParentheses)?;
        self.expect_and_consume(TokenType::BraceLeft, Error::MissingBraces)?;

        let mut body: Vec<Box<Stmt>> = Vec::new();
        while self.token_at().token_type() != TokenType::BraceRight {
            if self.token_at().token_type() == TokenType::EOF {
                return Err((Error::UnclosedBraces, self.current_line()));
            }
            body.push(Box::new(self.parse_stmt()?));
        }
        self.expect_and_consume(TokenType::BraceRight, Error::UnclosedBraces)?;

        Ok(Stmt::ConditionalExpression { condition, body })
    }

    fn parse_conditional_expr(&mut self) -> Result<Stmt> {
        let mut left = self.parse_additive_expr()?;
        loop {
            let operator = match self.token_at().token_type() {
                TokenType::EqualEqual => BinaryOperator::Equals,
                TokenType::NotEqual => BinaryOperator::NotEqual,
                TokenType::Greater => BinaryOperator::Greater,
                TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
                TokenType::Less => BinaryOperator::Less,
                TokenType::LessEqual => BinaryOperator::LessEqual,
                TokenType::And => BinaryOperator::And,
                TokenType::Or => BinaryOperator::Or,
                _ => break,
            };
            self.advance_pointer();

            left = Stmt::BinaryExpression {
                operator,
                lhs: Box::new(left),
                rhs: Box::new(self.parse_additive_expr()?),
            };
        }
        Ok(left)
    }

    fn parse_additive_expr(&mut self) -> Result<Stmt> {
        let mut left = self.parse_multiplicative_expr()?; // Identifier or Numeric Literal
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
                rhs: Box::new(self.parse_multiplicative_expr()?),
            };
        }
        Ok(left)
    }

    fn parse_multiplicative_expr(&mut self) -> Result<Stmt> {
        let mut left = self.parse_unary_expr()?;
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
                rhs: Box::new(self.parse_unary_expr()?),
            };
        }
        Ok(left)
    }

    fn parse_unary_expr(&mut self) -> Result<Stmt> {
        let operator = match self.token_at().token_type() {
            TokenType::Minus => UnaryOperator::MathematicalNegative,
            TokenType::Not => UnaryOperator::LogicalNegation,
            _ => return self.parse_primary_expr(),
        };
        self.advance_pointer();
        Ok(Stmt::UnaryExpression {
            operator,
            rhs: Box::new(self.parse_primary_expr()?),
        })
    }

    fn parse_dst_call(&mut self) -> Result<Stmt> {
        self.advance_pointer();
        match self.token_at().token_type() {
            TokenType::Arrow => self.advance_pointer(),
            _ => return Err((Error::UnexpectedToken, self.current_line())),
        };
        self.parse_expr()
    }

    fn parse_primary_expr(&mut self) -> Result<Stmt> {
        let primary_expr = match self.token_at().token_type() {
            TokenType::Ident(name) => match self.token_peek_type() {
                TokenType::Equal => self.parse_variable_assignment()?,
                _ => Stmt::Identifier(name),
            },
            TokenType::String(string) => Stmt::StringLiteral(string),

            TokenType::Integer(int) => Stmt::IntegerLiteral(int as i32),
            TokenType::Float(float) => Stmt::FloatLiteral(float),

            TokenType::True => Stmt::BooleanLiteral(true),
            TokenType::False => Stmt::BooleanLiteral(false),

            TokenType::Nil => Stmt::NilLiteral,

            TokenType::ParenLeft => {
                self.advance_pointer();
                let value = self.parse_expr()?;
                self.expect_and_consume(TokenType::ParenRight, Error::UnclosedParentheses)?;
                value
            }

            TokenType::DST => self.parse_dst_call()?,

            _ => return Err((Error::UnrecognizableToken, self.current_line())),
        };
        self.advance_pointer();
        Ok(primary_expr)
    }

    pub fn generate_ast(&mut self) -> Result<Stmt> {
        let mut body: Vec<Box<Stmt>> = Vec::new();

        loop {
            if let TokenType::EOF = self.token_at().token_type() {
                break;
            };
            body.push(Box::new(self.parse_stmt()?));
        }

        Ok(Stmt::Program { body })
    }
}
