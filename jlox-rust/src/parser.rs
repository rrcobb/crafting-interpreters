use crate::expr::*;
use crate::expr::Expr::*;
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;
use crate::token::Token;

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
	Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
	self.expression()
    }

    fn expression(&mut self) ->  Expr {
	let mut expr = self.equality();
	while self.mtch(vec![Comma]) {
	    let operator = self.previous();
	    let right = self.expression();
	    expr = Binary { left: Box::new(expr), operator, right: Box::new(right) };
	}

	expr
    }

    fn equality(&mut self) -> Expr {
	let mut expr = self.comparison();

	while self.mtch(vec![BangEqual, EqualEqual]) {
	    let operator = self.previous();
	    let right = self.comparison();
	    expr = Binary { left: Box::new(expr), operator, right: Box::new(right) };
	}

	expr
    }

    fn comparison(&mut self) -> Expr {
	let mut expr = self.addition();

	while self.mtch(vec![Greater, GreaterEqual, Less, LessEqual]) {
	    let operator: Token = self.previous();
	    let right = self.addition();
	    expr = Binary { left: Box::new(expr), operator, right: Box::new(right) };
	}

	expr
    }

    fn addition(&mut self) -> Expr {
	let mut expr = self.multiplication();

	while self.mtch(vec![Minus, Plus]) {
	    let operator = self.previous();
	    let right = self.multiplication();
	    expr = Binary { left: Box::new(expr), operator, right: Box::new(right) };
	}

	expr
    }

    fn multiplication(&mut self) -> Expr {
	let mut expr = self.unary();

	while self.mtch(vec![Slash, Star]) {
	    let operator = self.previous();
	    let right = self.unary();
	    expr = Binary { left: Box::new(expr), operator, right: Box::new(right) };
	}

	expr
    }

    fn unary(&mut self) -> Expr {
	if self.mtch(vec![Bang, Minus]) {
	    let operator = self.previous();
	    let right = self.unary();
	    Unary { operator, right: Box::new(right) }
	} else {
	    self.primary()
	}
    }

    fn primary(&mut self) -> Expr {
	match self.peek().type_ {
	    False => Literal { value: Lit::False },
	    True => Literal { value: Lit::True },
	    Nil => Literal { value: Lit::Nil },
	    Number { literal } => Literal { value: Lit::Number(literal) },
	    STRING { literal } => Literal { value: Lit::Strng(literal) },
	    LeftParen => {
		self.advance();
		let expr = self.expression();
		self.consume(&RightParen, "Expect ')' after expression.");
		Grouping { expression: Box::new(expr) }
	    }
	    _ => {
		panic!("failed in primary on not matching")
	    }
	}
    }

    fn consume(&mut self, type_: &TokenType, message: &str) {
	if self.check(type_) { self.advance(); }
    }

    fn mtch(&mut self, types: Vec<TokenType>) -> bool {
	for type_ in types.iter() {
	    if self.check(type_) {
		self.advance();
		return true;
	    }
	}
	false
    }

    fn check(&self, token_type: &TokenType) -> bool {
	if self.is_at_end() { return false; }
	let result = token_type == &self.peek().type_;
	result
    }

    fn previous(&self) -> Token {
	self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
	self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
	if !self.is_at_end() { self.current += 1 }
	self.previous()
    }

    fn is_at_end(&self) -> bool {
	match self.peek().type_ {
	    Eof => true,
	    _ => false
	}
    }

}
