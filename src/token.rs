#[derive(Debug)]
pub enum TokenType {
    ParenLeft, ParenRight, BraceLeft, BraceRight,
    Plus, Minus, Star, Slash, Comma, Dot, Semicolon,

    Not, NotEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Lower, LowerEqual,

    Ident(String), String(String), Number(u32),

    Let, And, Or, If, Else, Nil, Func,
    Return, True, False, While, For,
}

#[derive(Debug)]
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
}
