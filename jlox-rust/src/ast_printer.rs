// maps to jlox-java AstPrinter.java
use crate::token::*;
use crate::expr::*;
use crate::expr::Expr::*;
use crate::token_type::TokenType;

fn main() {
    let left = Unary {
	operator: Token {
	    type_: TokenType::Minus,
	    lexeme: "-".to_string(), 
	    line: 1
	},
	right: Box::new(Literal { value: Value::Number(123.0) })
    };
    let operator = Token { 
	type_: TokenType::Star,
	lexeme: "*".to_string(),
	line: 1
    };
    let right = Grouping {
	expression: Box::new(Literal { value: Value::Number(45.67) })
    };
    let expression = Binary { left: Box::new(left), operator, right: Box::new(right) };

    println!("{}", (AstPrinter {}).print(expression));
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> String {
	expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
	let mut s = String::from(format!("({}", name));
	for expr in exprs.iter() {
	    s.push(' ');
	    s.push_str(&expr.accept(self));
	}
	s.push(')');
	s
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
	self.parenthesize(&operator.lexeme, vec![left, right])
    }

    fn visit_grouping(&self, expression: &Expr) -> String {
	self.parenthesize("group", vec![expression])
    }

    fn visit_literal(&self, val: &Value) -> String {
	match val {
	    Value::False => "false".to_string(),
	    Value::True => "true".to_string(),
	    Value::Nil => "nil".to_string(),
	    Value::Number(n) => format!("{}", n),
	    Value::Strng(s) => s.to_string(),
	}
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> String {
	self.parenthesize(&operator.lexeme, vec![right])
    }
}
