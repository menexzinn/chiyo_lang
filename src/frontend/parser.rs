use crate::frontend::{Token, TokenType};
use crate::frontend::{Stmt, BinaryOperator, UnaryOperator, FunctionArgument};
use crate::utils::error::{Error, Result};

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

    fn current_line(&self) -> u32 {
        self.token_at().line()
    }

    fn advance_pointer(&mut self) {
        self.pointer += 1;
    }

    // prescidence order!:
    // primary expression -> unary -> multiplication -> addition ->
    // comparison -> logic -> function call -> assignment expression

    // for now we just focus primary -> multiplication -> addition
    // but first just primary -> addition
    fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.token_at().token_type() {
            TokenType::Let => self.parse_variable_declaration(false),
            TokenType::Const => self.parse_variable_declaration(true),
            TokenType::Func => self.parse_function_declaration(),
            _ => self.parse_expr(),
        }
    }

    // func function() {}
    // func function(arg: type) {}
    // func function(arg1: type1, arg2: type2, ...) {}
    // func function(): type {}
    fn parse_function_declaration(&mut self) -> Result<Stmt> {
        self.advance_pointer(); // consume "func"
        
        let Stmt::Identifier { value: name } = self.parse_expr()? else {
            return Err(( Error::FunctionInvalidName, self.current_line() ));
        };

        match self.token_at().token_type() {
            TokenType::ParenLeft => self.advance_pointer(),
            _ => return Err(( Error::MissingParentheses, self.current_line() )),
        };

        // func function(arg1: type1, )
        let mut args: Vec<FunctionArgument> = Vec::new();
        loop {
            match self.token_at().token_type() {
                TokenType::ParenRight => break self.advance_pointer(),
                _ => (),
            };

            let Stmt::Identifier { value: arg_name } = self.parse_expr()? else {
                return Err(( Error::FunctionArgumentMissingName, self.current_line() ));
            };

            match self.token_at().token_type() {
                TokenType::Colon => self.advance_pointer(),
                _ => return Err(( Error::MissingColon, self.current_line() )),
            };

            let Stmt::Identifier { value: arg_type } = self.parse_expr()? else {
                return Err(( Error::FunctionArgumentMissingType, self.current_line() ));
            };

            match self.token_at().token_type() {
                TokenType::ParenRight => {
                    args.push(FunctionArgument{ arg_name, arg_type });
                    break self.advance_pointer();
                },
                TokenType::Comma => {
                    self.advance_pointer();
                    match self.token_at().token_type() {
                        TokenType::ParenRight => return Err(( Error::MisplacedParentheses, self.current_line() )),
                        _ => args.push(FunctionArgument { arg_name, arg_type }),
                    };
                },
                _ => return Err(( Error::UnclosedParentheses, self.current_line() )),
            };
        };

        let return_type: Option<String> = match self.token_at().token_type() {
            TokenType::Colon => {
                self.advance_pointer();

                match self.parse_expr()? {
                    Stmt::Identifier { value } => Some(value),
                    _ => return Err(( Error::FunctionInvalidType, self.current_line() )),
                }
            },
            _ => None,
        };

        match self.token_at().token_type() {
            TokenType::BraceLeft => self.advance_pointer(),
            _ => return Err(( Error::MissingBraces, self.current_line() )),
        };

        let mut body: Vec<Box<Stmt>> = Vec::new();
        loop {
            match self.token_at().token_type() {
                TokenType::BraceRight => break self.advance_pointer(),
                TokenType::EOF => return Err(( Error::UnclosedBraces, self.current_line() )),
                _ => body.push(Box::new(self.parse_stmt()?)),
            };
        };

        Ok(Stmt::FunctionDeclaration {
            name,
            return_type,
            args,
            body,
        })
    }
    // as an error myself, I fucking hate error handling

    fn parse_variable_declaration(&mut self, is_constant: bool) -> Result<Stmt> {
        self.advance_pointer(); // consume "let" or "const"

        let Stmt::Identifier{ value: name } = self.parse_expr()? else {
            return Err(( Error::VariableInvalidName, self.current_line() ));
        };

        match self.token_at().token_type() {
            TokenType::Colon => self.advance_pointer(),
            _ => return Err(( Error::MissingColon, self.current_line() )),
        };

        let Stmt::Identifier{ value: var_type } = self.parse_expr()? else {
            return Err(( Error::VariableInvalidType, self.current_line() ));
        };

        match self.token_at().token_type() {
            TokenType::Equal => self.advance_pointer(),
            _ => return Err(( Error::VariableMissingEquals, self.current_line() )),
        };

        Ok(Stmt::VariableDeclaration {
            is_constant,
            name,
            var_type,
            value: Box::new(self.parse_expr()?),
        })
    }

    fn parse_expr(&mut self) -> Result<Stmt> {
        self.parse_additive_expr()
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

    fn parse_primary_expr(&mut self) -> Result<Stmt> {
        let primary_expr = match self.token_at().token_type() {
            TokenType::Ident(name) => Stmt::Identifier{ value: name },

            TokenType::Number(number) => Stmt::NumericLiteral{ value: number },

            TokenType::True => Stmt::BooleanLiteral{ value: true },
            TokenType::False => Stmt::BooleanLiteral{ value: false },

            TokenType::Nil => Stmt::NullLiteral,

            TokenType::ParenLeft => {
                self.advance_pointer();
                let value = self.parse_expr()?;
                match self.token_at().token_type() {
                    TokenType::ParenRight => value,
                    _ => return Err(( Error::UnclosedParentheses, self.current_line() )),
                }
            },

            _ => return Err(( Error::UnrecognizableToken, self.current_line() )),
        };
        self.advance_pointer();
        Ok(primary_expr)
    }

    pub fn generate_ast(&mut self) -> Result<Stmt> {
        let mut body: Vec<Box<Stmt>> = Vec::new();

        loop {
            if let TokenType::EOF = self.token_at().token_type() { break };
            body.push(Box::new(self.parse_stmt()?));
        }

        Ok(Stmt::Program{ body })
    }
}
