// notes: 
//  - um, so, Rust doesn't have classes
//  - so, the translation from Java will not be 1:1
//  but, w/e, so far this Java project is mostly namespaced functions anyways
//
//  hokay, so: we'll use Rust modules and structs to stand in for Java Classes
//  and I have to figure out again how the import system works in rust

// Lox.runPrompt: jlox/Lox.java L30
pub fn run_prompt() {
    println!("run from the prompt > ")
}

// Lox.runFile: jlox/Lox.java L24
pub fn run_file(path: &str) {
    println!("run a file {}", path);
}

