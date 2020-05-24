use crate::expr::*;
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn parse<T: Expr>(&self) -> &dyn Expr {
	self.expression()
    }

    fn expression(&self) ->  &dyn Expr {
	let mut expr = self.equality();
	while self.mtch(vec![Comma]) {
	    let operator = self.previous();
	    let right = self.expression();
	    expr = &Binary { left: expr, operator, right };
	}

	expr
    }

    fn equality(&self) -> &dyn Expr {
	let mut expr = self.comparison();

	while self.mtch(vec![BangEqual, EqualEqual]) {
	    let operator = self.previous();
	    let right = self.comparison();
	    expr = &Binary { left: expr, operator, right: right };
	}

	expr
    }

    fn comparison(&self) -> &dyn Expr {
	let mut expr = self.addition();

	while self.mtch(vec![Greater, GreaterEqual, Less, LessEqual]) {
	    let operator: Token = self.previous();
	    let right = self.addition();
	    expr = &Binary { left: expr, operator, right };
	}

	expr
    }

    fn addition(&self) -> &dyn Expr {
	let mut expr = self.multiplication();

	while self.mtch(vec![Minus, Plus]) {
	    let operator = self.previous();
	    let right = self.multiplication();
	    expr = &Binary { left: expr, operator, right };
	}

	expr
    }

    fn multiplication(&self) -> &dyn Expr {
	let mut expr = self.unary();

	while self.mtch(vec![Slash, Star]) {
	    let operator = self.previous();
	    let right = self.unary();
	    expr = &Binary { left: expr, operator, right };
	}

	expr
    }

    fn unary(&self) -> &dyn Expr {
	if self.mtch(vec![Bang, Minus]) {
	    let operator = self.previous();
	    let right = self.unary();
	    &Unary { operator, right }
	} else {
	    self.primary()
	}
    }

    fn primary(&self) -> &dyn Expr {
	match self.peek().type_ {
	    False => &Literal::False(false),
	    True => &Literal::True(true),
	    Nil => &Literal::Nil,
	    Number { literal } => &Literal::Number(literal),
	    STRING { literal } => &Literal::Strng(literal),
	    LeftParen => {
		let expr = self.expression();
		self.consume(RightParen, "Expect ')' after expression.");
		&Grouping { expression: expr }
	    }
	}
    }

    fn consume(&self, type_: TokenType, message: &str) {
	if self.check(type_) { self.advance(); }
			
    }

    fn mtch(&self, types: Vec<TokenType>) -> bool {
	for type_ in types.iter() {
	    if self.check(*type_) {
		self.advance();
		return true;
	    }
	}
	false
    }

    fn check(&self, type_: TokenType) -> bool {
	if self.is_at_end() { return false; }
	match self.peek().type_ {
	    type_ => true,
	    _ => false
	}
    }

    fn previous(&self) -> Token {
	self.tokens[self.current - 1]
    }

    fn peek(&self) -> Token {
	self.tokens[self.current]
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
