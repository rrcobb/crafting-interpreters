use std::fs;
use std::io;
use std::io::prelude::*;
use crate::scanner::*;
use crate::parser::*;
use crate::ast_printer::*;

// Lox.runPrompt: jlox/Lox.java L30
pub fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    io::stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        run(line.unwrap());
        print!("> ");
        io::stdout().flush().unwrap();
    }
}

// Lox.runFile: jlox/Lox.java L24
pub fn run_file(path: &str) {
    let contents = fs::read_to_string(path)
        .expect(&format!("an error while reading {}", path));
    run(contents);
}

// Lox.run: jlox/Lox.java L42
fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();

    println!("{}", (AstPrinter {}).print(expression));
}

// Lox.error: jlox/Lox.java L51
pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

// Lox.report jlox/Lox.java L51
fn report(line: usize, location: String, message: String) {
    eprintln!("[line {} ] Error {}: {}", line, location, message);    
}

