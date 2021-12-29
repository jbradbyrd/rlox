use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

pub struct Token<'src> {
    pub token_type: TokenType,
    pub string: &'src str,
    pub line: i32,
}

pub struct Error {
    pub message: &'static str,
    pub line: i32,
}

pub struct Scanner<'src> {
    source: &'src str,
    chars: Peekable<Chars<'src>>,
    start: usize,
    current: usize,
    line: i32,
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Scanner {
            source,
            chars: source.chars().peekable(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Result<Option<Token<'src>>, Error> {
        self.skip_whitespace();
        self.start = self.current;

        if let Some(ch) = self.advance() {
            let token_type = match ch {
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                '{' => TokenType::LeftBrace,
                '}' => TokenType::RightBrace,
                ';' => TokenType::Semicolon,
                ',' => TokenType::Comma,
                '.' => TokenType::Dot,
                '-' => TokenType::Minus,
                '+' => TokenType::Plus,
                '/' => TokenType::Slash,
                '*' => TokenType::Star,
                '!' => {
                    if self.matches('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                }
                '=' => {
                    if self.matches('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                }
                '<' => {
                    if self.matches('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                }
                '>' => {
                    if self.matches('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                }
                _ => return Err(self.make_error("Unexpected character.")),
            };
            
            Ok(Some(self.make_token(token_type)))
        } else {
            Ok(None)
        }
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.chars.next() {
            self.current += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        if let Some(ch) = self.chars.peek() {
            if *ch == expected {
                self.chars.next();
                self.current += 1;
                return true;
            }
        }

        false
    }

    fn skip_whitespace(&mut self) {
        loop {
            if let Some(ch) = self.chars.peek() {
                match ch {
                    ' ' | '\r' | '\t' => {
                        self.advance();
                    }
                    '\n' => {
                        self.line += 1;
                        self.advance();
                    }
                    _ => return,
                }
            } else {
                return;
            }
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token<'src> {
        Token {
            token_type,
            string: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    fn make_error(&self, message: &'static str) -> Error {
        Error {
            message,
            line: self.line,
        }
    }
}
