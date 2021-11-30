use std::{iter::{Enumerate, Peekable}, str::{CharIndices, Chars}};

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
  
    Error,
    Eof,
}
  
pub struct Token<'src> {
    pub token_type: TokenType,
    pub string: &'src str,
    pub line: i32,
}

pub struct Scanner<'src> {
    source: &'src str,
    chars: Peekable<Enumerate<Chars<'src>>>,
    start: usize,
    line: i32,
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Scanner {
            source,
            chars: source.chars().enumerate().peekable(),
            start: 0,
            line: 1,
         }
    }

    pub fn scan_token(&mut self) -> Option<Token<'src>> {
        if let Some((i, ch)) = self.chars.next() {
            self.start = i;
            match ch {
                '(' => self.make_token(TokenType::LeftParen, i),

                _ => None,
            }

        } else {
            None
        }
        
        /*
        */
    }

    fn make_token(&self, token_type: TokenType, length: usize) -> Option<Token<'src>> {
        Some(Token {
            token_type,
            string: &self.source[self.start .. length],
            line: self.line,
        })
    }
}
