use crate::frontend::{Token, TokenType};
use crate::utils::error::{Error, handle_error};

pub struct Scanner {
    tokens: Vec<Token>,
    line: u32,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            line: 1,
        }
    }

    fn add_token(&mut self, tk_type: TokenType) {
        self.tokens.push(Token::new(tk_type, self.line));
    }

    pub fn scan_tokens(&mut self, source: String) -> Vec<Token> {
        let mut source_iter = source.chars().peekable();

        while let Some(c) = source_iter.next() {
            match c {
                '(' => self.add_token(TokenType::ParenLeft),
                ')' => self.add_token(TokenType::ParenRight),
                '{' => self.add_token(TokenType::BraceLeft),
                '}' => self.add_token(TokenType::BraceRight),
                '+' => self.add_token(TokenType::Plus),
                '-' => self.add_token(TokenType::Minus),
                '*' => self.add_token(TokenType::Star),
                // no slash here, scroll a bit further
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                ':' => self.add_token(TokenType::Colon),

                '&' => self.add_token(TokenType::And),
                '|' => self.add_token(TokenType::Or),

                '!' => match source_iter.peek() {
                    Some(&'=') => {
                        self.add_token(TokenType::NotEqual);
                        source_iter.next();
                    },
                    _ => self.add_token(TokenType::Not),
                },
                '=' => match source_iter.peek() {
                    Some(&'=') => {
                        self.add_token(TokenType::EqualEqual);
                        source_iter.next();
                    },
                    _ => self.add_token(TokenType::Equal),
                },
                '>' => match source_iter.peek() {
                    Some(&'=') => {
                        self.add_token(TokenType::GreaterEqual);
                        source_iter.next();
                    },
                    _ => self.add_token(TokenType::Greater),
                },
                '<' => match source_iter.peek() {
                    Some(&'=') => {
                        self.add_token(TokenType::LowerEqual);
                        source_iter.next();
                    },
                    _ => self.add_token(TokenType::Lower),
                },
                '/' => match source_iter.peek() {
                    Some(&'/') => {
                        // consume the second slash
                        source_iter.next();
                        // imma ignore yapping (comments) till next line
                        // if None, then the file just ends there
                        while let Some(n) = source_iter.next() {
                            if n == '\n' {
                                self.line += 1;
                                break;
                            }
                        };
                    },
                    _ => self.add_token(TokenType::Slash),
                },

                ' ' => continue,
                '\r' => continue,
                '\t' => continue,
                '\n' => self.line += 1,

                '"' => {
                    let mut string = String::new();
                    let mut is_second_quote_here = false;

                    while let Some(s) = source_iter.next() {
                        if s == '"' {
                            is_second_quote_here = true;
                            break;
                        }
                        string.push(s);
                    }

                    if !is_second_quote_here {
                        handle_error(Error::UnclosedQuotes, self.line);
                    }

                    self.add_token(TokenType::String(string));
                },

                n @ '0'..='9' => {
                    let mut number_string = n.to_string();

                    while let Some(&s) = source_iter.peek() {
                        if !s.is_ascii_digit() {
                            break;
                        }

                        number_string.push(s);
                        source_iter.next();
                    }

                    // I use unwrap here because it's guaranteed to be a number
                    let number: u32 = number_string.parse().unwrap();
                    self.add_token(TokenType::Number(number));
                },

                x if x.is_ascii_alphabetic() => {
                    let mut string = x.to_string();

                    while let Some(&s) = source_iter.peek() {
                        if !s.is_ascii_alphabetic() {
                            break;
                        }
                        string.push(s);
                        // peek element, if alphabetic then consume, else break loop ez
                        source_iter.next();
                    };

                    match string.as_str() {
                        "let" => self.add_token(TokenType::Let),
                        "const" => self.add_token(TokenType::Const),
                        "if" => self.add_token(TokenType::If),
                        "else" => self.add_token(TokenType::Else),
                        "nil" => self.add_token(TokenType::Nil),
                        "func" => self.add_token(TokenType::Func),
                        "return" => self.add_token(TokenType::Return),
                        "true" => self.add_token(TokenType::True),
                        "false" => self.add_token(TokenType::False),
                        "while" => self.add_token(TokenType::While),
                        "for" => self.add_token(TokenType::For),
                        _ => self.add_token(TokenType::Ident(string)),
                    };
                },

                _ => handle_error(Error::UnrecognizableCharacter, self.line),
            };
        }
        self.add_token(TokenType::EOF);
        self.tokens.clone()
    }
}
