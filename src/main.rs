#![allow(dead_code)]

mod chunk;
mod compiler;
mod scanner;
mod value;
mod vm;

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process;
use vm::*;

fn main() {
    let argv = env::args();
    let argc = argv.len();
    if argc == 1 {
        repl();
    } else if argc == 2 {
        run_file(&argv.last().unwrap());
    } else {
        eprintln!("Usage: clox [path]");
        process::exit(64);
    }
}

fn repl() {
    let mut vm = VM::new();

    fn prompt() {
        print!("> ");
        io::stdout().flush().unwrap();
    }

    prompt();
    for line in io::stdin().lock().lines() {
        // Swallow any errors when running the repl.
        let _result = vm.interpret(&line.unwrap());
        prompt();
    }
}

fn run_file(file_name: &str) {
    match fs::read_to_string(file_name) {
        Ok(source) => {
            let mut vm = VM::new();
            let result = vm.interpret(&source);
            drop(vm);
            match result {
                Ok(()) => {}
                Err(InterpretError::Compile) => process::exit(65),
                Err(InterpretError::Runtime) => process::exit(70),
            }
        }
        Err(error) => {
            eprintln!("{}", error);
            process::exit(74);
        }
    }
}
