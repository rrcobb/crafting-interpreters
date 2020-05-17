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

    println!("{}", expression.accept(&AstPrinter {}));
}

pub fn print(expr: impl Expr) -> String {
    let printer = AstPrinter {};
    expr.accept(&printer)
}

pub struct AstPrinter;

impl Visitor for AstPrinter {
    fn visit_binary(&self, expr: &Binary) -> String {
	parenthesize(&expr.operator.lexeme, vec![expr.left, expr.right], self)
    }

    fn visit_grouping(&self, expr: &Grouping) -> String {
	parenthesize("group", vec![expr.expression], self)
    }

    fn visit_literal(&self, expr: &Literal) -> String {
	(expr.value).to_string()
    }
    fn visit_unary(&self, expr: &Unary) -> String {
	parenthesize(&expr.operator.lexeme, vec![expr.right], self)
    }
}

fn parenthesize(name: &str, exprs: Vec<&dyn Expr>, printer: &AstPrinter) -> String {
    let mut s = String::from(format!("({}", name));
    for expr in exprs.iter() {
	s.push(' ');
	s.push_str(&expr.accept(printer));
    }
    s.push(')');
    s
}
