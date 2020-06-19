// environment where variables and their values will live

use crate::expr::Value;
use crate::token::Token;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>
}
    
impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None
        }
    }

    pub fn from(enclosing: &Rc<RefCell<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Rc::clone(enclosing)),
        }
    }

    pub fn get(&self, name: &Token) -> Value {
        if let Some(value) = self.values.get(&name.lexeme) {
            value.clone()
        } else {
            match &self.enclosing {
                Some(enclosing) => enclosing.borrow().get(name),
                None => panic!("Undefined variable '{}'.", name.lexeme)
            }
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn assign(&mut self, name: &Token, value: Value) {
        if self.values.contains_key(&name.lexeme) {
            // have to copy the string for the hashmap
            self.values.insert(name.lexeme.to_owned(), value);
            return;
        } else {
            if let Some(enclosing) = &self.enclosing {
                enclosing.borrow_mut().assign(&name, value);
            } else {
                panic!("Undefined variable '{}'.", name.lexeme); 
            }
        }
    }
}
