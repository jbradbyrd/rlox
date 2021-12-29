use super::scanner::Scanner;

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);
    let mut line = -1;

    loop {
        match scanner.scan_token() {
            Ok(Some(token)) => {
                if token.line != line {
                    print!("{:>4} ", token.line);
                    line = token.line;
                } else {
                    print!("   | ");
                }

                println!("{:>2} '{}'", token.token_type as i32, token.string);
            }
            Ok(None) => {
                return;
            }
            Err(_error) => {
                return;
            }
        }
    }
}
