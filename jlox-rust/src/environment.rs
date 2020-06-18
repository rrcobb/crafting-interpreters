// environment where variables and their values will live

use crate::expr::Value;
use crate::token::Token;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>
}
    
impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None
        }
    }

    pub fn within(enclosing: Environment) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing))
        }
    }

    pub fn get(&self, name: &Token) -> Value {
        match self.values.get(&name.lexeme) {
            Some(value) => value.clone(),
            None => {
               match &self.enclosing {
                   Some(enclosing) => enclosing.get(name),
                   None => {
                       panic!("Undefined variable '{}'.", name.lexeme);
                   }
               }
            }
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    pub fn assign(&mut self, name: &Token, value: Value) {
        match self.values.get(&name.lexeme) { 
            Some(_) => { self.values.insert(name.lexeme.to_owned(), value); },
            None => {
                if self.enclosing.is_some() {
                    self.enclosing.as_mut().unwrap().assign(&name, value.clone());
                } else {
                    panic!("Undefined variable '{}'.", name.lexeme);
                }
            }
        }
    }
}
