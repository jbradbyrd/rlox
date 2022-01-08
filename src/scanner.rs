use std::iter::Iterator;
use std::str::Chars;

#[derive(Debug)]
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
    start: usize,
    current: usize,
    peek: Option<char>,
    peek_next: Option<char>,
    chars: Chars<'src>,
    line: i32,
}

trait CharChecker {
    fn is_alpha(&self) -> bool;
    fn is_dec_digit(&self) -> bool;
}

impl CharChecker for char {
    fn is_alpha(&self) -> bool {
        self.is_ascii_alphabetic() || *self == '_'
    }

    fn is_dec_digit(&self) -> bool {
        self.is_ascii_digit()
    }
}

impl CharChecker for Option<char> {
    fn is_alpha(&self) -> bool {
        match self {
            Some(ch) => ch.is_alpha(),
            None => false,
        }
    }

    fn is_dec_digit(&self) -> bool {
        match self {
            Some(ch) => ch.is_dec_digit(),
            None => false,
        }
    }
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        let mut chars = source.chars();

        Scanner {
            source,
            start: 0,
            current: 0,
            peek: chars.next(),
            peek_next: chars.next(),
            chars,
            line: 1,
        }
    }

    fn scan_token(&mut self) -> Option<Result<Token<'src>, Error>> {
        self.skip_whitespace();
        self.start = self.current;

        self.advance().map(|ch| {
            if ch.is_alpha() {
                return self.make_identifier();
            }

            if ch.is_dec_digit() {
                return self.make_number();
            }

            match ch {
                '(' => self.make_token(TokenType::LeftParen),
                ')' => self.make_token(TokenType::RightParen),
                '{' => self.make_token(TokenType::LeftBrace),
                '}' => self.make_token(TokenType::RightBrace),
                ';' => self.make_token(TokenType::Semicolon),
                ',' => self.make_token(TokenType::Comma),
                '.' => self.make_token(TokenType::Dot),
                '-' => self.make_token(TokenType::Minus),
                '+' => self.make_token(TokenType::Plus),
                '/' => self.make_token(TokenType::Slash),
                '*' => self.make_token(TokenType::Star),
                '!' => {
                    if self.matches('=') {
                        self.make_token(TokenType::BangEqual)
                    } else {
                        self.make_token(TokenType::Bang)
                    }
                }
                '=' => {
                    if self.matches('=') {
                        self.make_token(TokenType::EqualEqual)
                    } else {
                        self.make_token(TokenType::Equal)
                    }
                }
                '<' => {
                    if self.matches('=') {
                        self.make_token(TokenType::LessEqual)
                    } else {
                        self.make_token(TokenType::Less)
                    }
                }
                '>' => {
                    if self.matches('=') {
                        self.make_token(TokenType::GreaterEqual)
                    } else {
                        self.make_token(TokenType::Greater)
                    }
                }
                '"' => self.make_string(),
                _ => self.make_error("Unexpected character."),
            }
        })
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.peek;
        self.peek = self.peek_next;
        self.peek_next = self.chars.next();

        if let Some(ch) = next {
            self.current += ch.len_utf8();
        }

        next
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.peek == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek {
            match ch {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next == Some('/') {
                        while self.peek != Some('\n') && self.peek.is_some() {
                            self.advance();
                        }
                    }
                }
                _ => return,
            }
        }
    }

    fn make_identifier(&mut self) -> Result<Token<'src>, Error> {
        while self.peek.is_alpha() || self.peek.is_dec_digit() {
            self.advance();
        }

        self.make_token(match &self.source[self.start..self.current] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        })
    }

    fn make_number(&mut self) -> Result<Token<'src>, Error> {
        while self.peek.is_dec_digit() {
            self.advance();
        }

        if self.peek == Some('.') && self.peek_next.is_dec_digit() {
            self.advance(); // consume decimal
            while self.peek.is_dec_digit() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn make_string(&mut self) -> Result<Token<'src>, Error> {
        while self.peek != Some('"') && self.peek.is_some() {
            if self.peek == Some('\n') {
                self.line += 1;
            }

            self.advance();
        }

        if self.peek.is_none() {
            self.make_error("Unterminated string.")
        } else {
            self.advance(); // closing quote
            self.make_token(TokenType::String)
        }
    }

    fn make_token(&self, token_type: TokenType) -> Result<Token<'src>, Error> {
        Ok(Token {
            token_type,
            string: &self.source[self.start..self.current],
            line: self.line,
        })
    }

    fn make_error(&self, message: &'static str) -> Result<Token<'src>, Error> {
        Err(Error {
            message,
            line: self.line,
        })
    }
}

impl<'src> Iterator for Scanner<'src> {
    type Item = Result<Token<'src>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}
