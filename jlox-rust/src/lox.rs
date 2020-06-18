use std::fs;
use std::io;
use std::io::prelude::*;
use crate::scanner::*;
use crate::parser::*;
// use crate::ast_printer::*;
use crate::interpreter::*;

pub struct Lox {
    interpreter: Interpreter
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            interpreter: Interpreter::new()
        }
    }

    // Lox.runPrompt: jlox/Lox.java L30
    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();
        print!("> ");
        io::stdout().flush().unwrap();
        for line in stdin.lock().lines() {
            self.run(line.unwrap());
            print!("> ");
            io::stdout().flush().unwrap();
        }
    }

    // Lox.runFile: jlox/Lox.java L24
    pub fn run_file(&mut self, path: &str) {
        let contents = fs::read_to_string(path)
            .expect(&format!("an error while reading {}", path));
        self.run(contents);
    }

    // Lox.run: jlox/Lox.java L42
    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse();

        // println!("{}", (AstPrinter {}).print(expression));
        self.interpreter.interpret(stmts);
    }
}

    // Lox.error: jlox/Lox.java L51
    pub fn error(line: usize, message: String) {
        report(line, "".to_string(), message);
    }

    // Lox.report jlox/Lox.java L51
    fn report(line: usize, location: String, message: String) {
        eprintln!("[line {} ] Error {}: {}", line, location, message);    
    }

