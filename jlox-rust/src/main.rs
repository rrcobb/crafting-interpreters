use std::env;
mod lox;
mod token_type;
mod scanner;
mod token;

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


// notes: 
//  - um, so, Rust doesn't have classes
//  - so, the translation from Java will not be 1:1
//  but, w/e, so far this Java project is mostly namespaced functions anyways
//
//  hokay, so: we'll use Rust modules and structs to stand in for Java Classes
//  and I have to figure out again how the import system works in rust
//
//  For the global variables... I guess I'll just use globals? idk, seems dangerous, so Rust will
//  try to stop me (or at least make it difficult)
