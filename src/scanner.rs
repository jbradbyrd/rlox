use std::str::Chars;

pub struct Token<'src> {
    string: &'src str,
    line: i32,
}

pub struct Scanner<'src> {
    chars: Chars<'src>,
    line: i32,
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Scanner {
            chars: source.chars(),
            line: 1,
         }
    }

    pub fn scan_token() -> Token<'src> {
        
    }
}
