// maps to jlox-java file jlox/Token.java
use std::fmt;
use crate::token_type::*;

// final members of Token
// also - Rust implements a constructor for free, so we don't need Token.Token
#[derive(Debug, Clone)]
pub struct Token {
    // 'type' in Java (reserved keyword in Rust)
    type_: TokenType,
    lexeme: String,
    // literal: ???, needs to be generic?
    line: u8,
}

// Token.toString in jlox/Token.java L16
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.type_, self.lexeme)
    }
}
