// This file maps to the jlox-java file jlox/Scanner.java
// The Java Scanner is a class, and uses mutable state and methods.
// The Rust translation of that will be a struct with impls of those methods, mutating the state
// We don't need any inheritance, so it should be a pretty straightforward translation
use crate::token::*;
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;

// class definition, final variables
#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

// match instead of a hashmap, since rust is picky about hashmaps...
// note: not clear about perf vs. actual hashmap
// basic redditor suggests match is better https://www.reddit.com/r/rust/comments/5mnj3y/which_has_better_performance_a_hashmap_or_a/
// see also https://users.rust-lang.org/t/match-statement-efficiency/4488/2
fn keyword_get(text: &str) -> Option<TokenType> {
    match text {
        "and" => Some(And),
        "class" => Some(Class),
        "else" => Some(Else),
        "false" => Some(False),
        "for" => Some(For),
        "fun" => Some(Fun),
        "if" => Some(If),
        "nil" => Some(Nil),
        "or" => Some(Or),
        "print" => Some(Print),
        "return" => Some(Return),
        "super" => Some(Super),
        "this" => Some(This),
        "true" => Some(True),
        "var" => Some(Var),
        "while" => Some(While),
        _ => None
    }
}

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
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token { type_: Eof, lexeme: "".to_string(), line: self.line});

        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance().unwrap();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => if self.match_('=') { self.add_token(BangEqual) } else { self.add_token(Bang) },
            '=' => if self.match_('=') { self.add_token(EqualEqual) } else { self.add_token(Equal) },
            '<' => if self.match_('=') { self.add_token(LessEqual) } else { self.add_token(Less) },
            '>' => if self.match_('=') { self.add_token(GreaterEqual) } else { self.add_token(Greater) },
            '/' => { 
                if self.match_('/') { 
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); };
                } else {
                    self.add_token(Slash)
                }
            }
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if Scanner::is_digit(c) {
                    self.number()
                } else if Scanner::is_alpha(c) {
                    self.identifier()
                }
                else {
                    crate::lox::error(self.line, "Unexpected character.".to_string())
                }
            }
        }
    }

    fn cur(&self) -> Option<char> {
        // fyi - this is O(n), and supports utf8
        self.source.chars().nth(self.current)
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.cur().unwrap()
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 > self.source.len() {
            None
        } else {
            self.source.chars().nth(self.current + 1)
        }
    }

    // match is a keyword
    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        if self.peek() != expected { return false }
        self.current += 1;
        true
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn current_substring(&self) -> String {
        self.source[self.start..self.current].to_string()
    }

    fn add_token(&mut self, type_: TokenType) {
        let lexeme = self.current_substring();
        self.tokens.push(Token { type_, lexeme, line: self.line}) 
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
            (c >= 'A' && c <= 'Z') ||
            c == '_'
    }

    fn is_alphanumeric(c: char) -> bool {
        Scanner::is_digit(c) || Scanner::is_alpha(c)
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) { self.advance(); }
        let text = self.current_substring();
        let type_ = keyword_get(&text).unwrap_or(Identifier);
        self.add_token(type_);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1 }
            self.advance();
        }

        if self.is_at_end() {
            crate::lox::error(self.line, "Unterminated string.".to_string());
            return
        }

        self.advance();
        let literal: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(STRING { literal })
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) { self.advance(); }

        // if there's a fractional part (e.g. after a '.')
        if self.peek() == '.' && Scanner::is_digit(self.peek_next().unwrap()) {
            self.advance();
            while Scanner::is_digit(self.peek()) { self.advance(); }
        }

        let literal: f64 = self.current_substring().parse().unwrap();
        self.add_token(Number { literal })
    }
}
