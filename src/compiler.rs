use super::scanner::{Scanner};

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);
    let mut line = -1;
    while let Some(token) = scanner.scan_token() {
        if token.line != line {
            print!("{:>4}", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }

        println!("{:>2} '{}'", token.token_type as i32, token.string);
    }
}
