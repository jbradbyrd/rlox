use super::scanner::Scanner;

pub fn compile(source: &str) {
    fn print_header(curr_line: &mut i32, line: i32) {
        if *curr_line != line {
            print!("{:>4} ", line);
            *curr_line = line;
        } else {
            print!("   | ");
        }
    }

    let mut line = -1;
    for item in Scanner::new(source) {
        match item {
            Ok(token) => {
                print_header(&mut line, token.line);
                println!("{:>2} '{}'", token.token_type as i32, token.string);
            }
            Err(error) => {
                print_header(&mut line, error.line);
                println!(" E '{}'", error.message);
                return;
            }
        }
    }
}
