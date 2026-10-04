#[derive(Clone, Debug)]
pub enum TokenType {
    ParenLeft, ParenRight, BraceLeft, BraceRight,
    Plus, Minus, Star, Slash, Comma, Dot,

    Not, NotEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Lower, LowerEqual,
    And, Or,

    Ident(String), String(String), Number(u32),

    Let, Const, If, Else, Nil, Func,
    Return, True, False, While, For,
    EOF,
}

#[derive(Clone, Debug)]
pub struct Token {
    tk_type: TokenType,
    line: u32,
}

impl Token {
    pub fn new(tk_type: TokenType, line: u32) -> Self {
        Self {
            tk_type,
            line,
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.tk_type.clone()
    }

    pub fn line(&self) -> u32 {
        self.line
    }
}
