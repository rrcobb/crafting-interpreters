// NOT ACTUALLY GENERATED
// but theoretically, generated by bin/generate_ast.rs
use std::fmt;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
	name: Token,
	value: Box<Expr>
    },
    Binary {
	left: Box<Expr>,
	// there's actually a smaller subset of tokens that can be binary operators
	// it may be worth defining a type just for those
	operator: Token,
	right: Box<Expr>,
    },
    Grouping {
	expression: Box<Expr>,
    },
    Literal {
	value: Value,
    },
    Unary {
	// there's a small of tokens that can be unary operators - just Minus and Bang
	operator: Token,
	right: Box<Expr>,
    },
    Variable {
	name: Token,
    }
}

pub trait Visitor<T> {
    fn visit_assignment(&mut self, name: &Token, value: &Expr) -> T;
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping(&mut self, expression: &Expr) -> T;
    fn visit_literal(&self, value: &Value) -> T;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_variable(&self, name: &Token) -> T;
}

impl Expr {
    pub fn accept<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
	use crate::expr::Expr::*;
	match self {
	    Assign { name, value } => visitor.visit_assignment(name, value),
	    Binary {left, operator, right} => visitor.visit_binary(left, operator, right),
	    Grouping { expression } => visitor.visit_grouping(expression),
	    Literal { value }=> visitor.visit_literal(value),
	    Unary { operator, right } => visitor.visit_unary(operator, right),
	    Variable { name } => visitor.visit_variable(name),
	}
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    False,
    True,
    Nil,
    Number(f64),
    Strng(String)
}

impl From<bool> for Value {
    fn from(b: bool) -> Value {
	match b {
	    false => Value::False,
	    true => Value::True
	}
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Value {
	Value::Number(n)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
	Value::Strng(s)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	use Value::*;
	match self {
	    Nil => write!(f, "nil"),
	    True => write!(f, "true"),
	    False => write!(f, "false"),
	    Number(n) => write!(f, "{}", n),
	    Strng(s) => write!(f, "{}", s)
	}
    }
}
