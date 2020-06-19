use crate::expr::*;
use crate::expr::Expr::*;
use crate::stmt::*;
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;
use crate::token::Token;
// use crate::ast_printer::*;

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
	Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
	let mut stmts: Vec<Stmt> = vec![];
	while !self.is_at_end() {
	    stmts.push(self.declaration());
	}
	stmts
    }

    fn declaration(&mut self) -> Stmt {
	if self.mtch(vec![TokenType::Var]) {
	    self.var_declaration()
	} else {
	    self.statement()
	}
    }

    fn statement(&mut self) -> Stmt {
	if self.mtch(vec![TokenType::Print]) {
	    self.print_statement()
	} else if self.mtch(vec![TokenType::LeftBrace]) {
	    self.block_statement()
	} else {
	    self.expression_statement()
	}
    }

    fn var_declaration(&mut self) -> Stmt {
	let name = self.consume(&Identifier, "Expect variable name").unwrap();

	let mut initializer = Expr::Literal { value: Value::Nil };
	if self.mtch(vec![Equal]) {
	    initializer = self.expression();
	}

	self.consume(&Semicolon, "Expect ';' after variable declaration");
	Stmt::Var { name, initializer: Box::new(initializer) } 	
    }

    fn print_statement(&mut self) -> Stmt {
	let value = self.expression();
	self.consume(&Semicolon, "Expect ';' after value.");
	Stmt::Print { expr: Box::new(value) }	
    }

    fn block_statement(&mut self) -> Stmt {
	let mut stmts = vec![];
	while !self.check(&RightBrace) && !self.is_at_end() {
	    stmts.push(self.declaration());
	}
	self.consume(&RightBrace, "Expect '}' after block.");
	Stmt::Block { stmts }
    }

    fn expression_statement(&mut self) -> Stmt {
	let value = self.expression();
	self.consume(&Semicolon, "Expect ';' after value.");
	Stmt::Expression { expr: Box::new(value) }	
    }

    fn expression(&mut self) -> Expr {
	self.assignment()
    }

    fn assignment(&mut self) ->  Expr {
	let mut expr = self.equality();

	if self.mtch(vec![Equal]) {
	    // let equals = self.previous(); needed only if we're erroring with a token
	    let value = self.equality();
	    expr = match expr {
		Variable { name } => Expr::Assign { name, value: Box::new(value) },
		_ => { panic!("Invalid assignment target."); }
	    };
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
	let mut advance = true;
	let res = match self.peek().type_ {
	    False => Literal { value: Value::False },
	    True => Literal { value: Value::True },
	    Nil => Literal { value: Value::Nil },
	    Number { literal } => Literal { value: Value::Number(literal) },
	    STRING { literal } => Literal { value: Value::Strng(literal) },
	    // not quite a transliteration, because we're rust match, and we're advancing after
	    Identifier => Variable { name: self.peek() },
	    LeftParen => {
		// move past the left paren
		self.advance();
		// consume the expression
		let expr = self.expression();
		// eat the right paren
		self.consume(&RightParen, "Expect ')' after expression.");
		// don't advance past the right paren
		advance = false;
		Grouping { expression: Box::new(expr) }

	    }
	    _ => {
		println!("failing token {:?}", self.peek());
		panic!("failed in primary on not matching")
	    }
	};
	// hacky skip for grouping
	if advance { self.advance(); }
	res
    }

    // currently does not do the erroring that the Java version does
    fn consume(&mut self, type_: &TokenType, message: &str) -> Option<Token> {
	if self.check(type_) { 
	    Some(self.advance())
	} else {
	    println!("{}", message);
	    None
	}
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
