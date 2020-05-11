// This file maps to the jlox-java file jlox/Scanner.java
// The Java Scanner is a class, and uses mutable state and methods.
// The Rust translation of that will be a struct with impls of those methods, mutating the state
// We don't need any inheritance, so it should be a pretty straightforward translation
use std::collections::HashMap;
use crate::token::*;
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;

// class definition, final variables
#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u8,
    current: u8,
    line: u8,
}

const keywords: HashMap<String, TokenType> = vec![
    ("and".to_string(), And),
    ("class".to_string(), Class),
    ("else".to_string(), Else),
    ("false".to_string(), False),
    ("for".to_string(), For),
    ("fun".to_string(), Fun),
    ("if".to_string(), If),
    ("nil".to_string(), Nil),
    ("or".to_string(), Or),
    ("print".to_string(), Print),
    ("return".to_string(), Return),
    ("super".to_string(), Super),
    ("this".to_string(), This),
    ("true".to_string(), True),
    ("var".to_string(), Var),
    ("while".to_string(), While),
].into_iter().collect();

impl Scanner {
    // Scanner.Scanner L38
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    // Scanner.scanTokens L42
    pub fn scan_tokens(&self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token { type_: Eof, lexeme: "".to_string(), line: self.line});

        self.tokens.clone()
    }


}
