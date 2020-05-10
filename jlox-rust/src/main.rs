use std::env;
mod lox;
use crate::lox::*;

// Lox.main: jlox/Lox.java L14
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.len() {
       1 => run_prompt(),
       2 => run_file(&args[1]),
       _ => println!("Usage: jlox [script]")
    }
}
