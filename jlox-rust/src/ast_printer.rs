// maps to jlox-java AstPrinter.java
mod expr;
mod token_type;
mod token;
use crate::token_type::TokenType;
use crate::token::*;
use crate::expr::*;

fn main() {
    let left = Unary {
	operator: Token {
	    type_: TokenType::Minus,
	    lexeme: "-".to_string(), 
	    line: 1
	},
	right: &Literal { value: "123".to_string() },
    };
    let operator = Token { 
	type_: TokenType::Star,
	lexeme: "*".to_string(),
	line: 1
    };
    let right = Grouping {
	expression: &Literal { value: "45.67".to_string() }
    };
    let expression = Binary { left: &left, operator, right: &right };

    println!("{}", expression.print());
}

pub trait AstPrintable: Expr {
    fn print(&self) -> String;
}

pub fn print<T: AstPrintable>(expr: T) -> String {
    expr.print()
}

impl AstPrintable for Binary<'_> {
    fn print(&self) -> String {
	parenthesize(&self.operator.lexeme, vec![self.left, self.right])
    }
}

impl AstPrintable for Ternary<'_> {
    fn print(&self) -> String {
	parenthesize("ternary", vec![self.first, self.second, self.third])
    }
}

impl AstPrintable for Grouping<'_> {
    fn print(&self) -> String {
	parenthesize("group", vec![self.expression])
    }
}

impl AstPrintable for Literal {
    fn print(&self) -> String {
	(&self.value).to_string()
    }
}

impl AstPrintable for Unary<'_> {
    fn print(&self) -> String {
	parenthesize(&self.operator.lexeme, vec![self.right])
    }
}

fn parenthesize(name: &str, exprs: Vec<&dyn Expr>) -> String {
    let mut s = String::from(format!("({}", name));
    for expr in exprs.iter() {
	s.push(' ');
	s.push_str(&format!("{:?}", expr));
    }
    s.push(')');
    s
}
