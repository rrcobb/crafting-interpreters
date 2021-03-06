use crate::expr;
use expr::{Expr, Value};
use crate::stmt;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::environment::Environment;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>
}

impl Interpreter {
    pub fn new() -> Interpreter {
	Interpreter {
	    environment: Rc::new(RefCell::new(Environment::new()))
	}
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
	for stmt in stmts {
	    self.execute(&stmt);
	}
    }

    fn execute(&mut self, stmt: &Stmt) {
	stmt.accept(self);
    }

    fn evaluate(&mut self, expr: &Expr) -> Value {
	expr.accept(self)
    }

    fn is_truthy(&self, val: Value) -> bool {
	use Value::*;
	match val {
	    False => false,
	    True => true,
	    Nil => false,
	    Number(_) => true,
	    Strng(_) => true,
	}
    }

    fn is_equal(&self, a: Value, b: Value) -> bool {
	a == b
    }

    fn numeric(&self, operand: Value) -> f64 {
	match operand {
	    Value::Number(n) => n,
	    _ => panic!("operand must be a number")
	}
    }

}

impl stmt::Visitor<()> for Interpreter {
    fn visit_block(&mut self, stmts: &Vec<Stmt>) {
	let prev = self.environment.clone();
	let new = Rc::new(RefCell::new(Environment::from(&self.environment)));
	self.environment = new;
	for stmt in stmts.iter() {
	    self.execute(stmt);
	}
	self.environment = prev;
    }

    fn visit_print(&mut self, expr: &Expr) {
	let val = self.evaluate(expr);
	println!("{}", val);	
    }

    fn visit_expression(&mut self, expr: &Expr) {
	self.evaluate(expr);
    }

    fn visit_var(&mut self, name: &Token, initializer: &Expr) {
	let value = self.evaluate(initializer);
	self.environment.borrow_mut().define(&name.lexeme, value);	
    }
}

impl expr::Visitor<Value> for Interpreter {
    fn visit_assignment(&mut self, name: &Token, value: &Expr) -> Value {
	let val = self.evaluate(value);
	self.environment.borrow_mut().assign(name, val.clone());
	val
    }

    fn visit_literal(&self, val: &Value) -> Value {
	val.clone()
    }

    fn visit_grouping(&mut self, expression: &Expr) -> Value {
	self.evaluate(expression)
    }

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Value {
	let lt = self.evaluate(left);
	let rt = self.evaluate(right);
	
	use TokenType::*;
	// note: would be less verbose to implement
	// https://doc.rust-lang.org/std/ops/index.html
	// for the Value enum
	// then just apply them
	match operator.type_ {
	    Greater => { 
		Value::from(
		    self.numeric(lt) > self.numeric(rt)
		)
	    } ,
	    GreaterEqual => {
		Value::from(
		    self.numeric(lt) >= self.numeric(rt)
		)
	    },
	    Less => {
		Value::from(
		    self.numeric(lt) < self.numeric(rt)
		)
	    },
	    LessEqual => {
		Value::from(
		    self.numeric(lt) <= self.numeric(rt)
		)
	    },
	    Minus => {
		Value::from(
		    self.numeric(lt) - self.numeric(rt)
		)
	    },
	    Plus => {
		// handle string and number cases
		match (lt, rt) {
		    (Value::Strng(l), Value::Strng(r)) => Value::from(l + &r),
		    (Value::Number(l), Value::Number(r)) => Value::from(l + r),
		    (_, _) => panic!("operands to plus must both be numbers or both be strings"),
		}
	    },
	    Slash => {
		Value::from(
		    self.numeric(lt) / self.numeric(rt)
		)
	    },
	    Star => {
		Value::from(
		    self.numeric(lt) * self.numeric(rt)
		)
	    },
	    BangEqual => { Value::from(!self.is_equal(lt, rt)) },
	    EqualEqual => { Value::from(self.is_equal(lt, rt)) },
	    _ => Value::Nil
	}
    }

    fn visit_unary(&mut self, operator: &Token, expr: &Expr) -> Value {
	let right = self.evaluate(expr);
	match operator.type_ {
	    TokenType::Minus => {
		Value::from(-self.numeric(right))
	    },
	    TokenType::Bang => Value::from(!self.is_truthy(right)),
	    _ => panic!("unary operator should only be minus or bang")
	}
    }

    fn visit_variable(&self, name: &Token) -> Value {
	self.environment.borrow().get(name)
    }
}
