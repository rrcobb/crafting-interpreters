// transliteration of tool/GenerateAst.java
// not idiomatic Rust
use std::env;
use std::fs::{File};
use std::io::prelude::*;
use std::io::LineWriter;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => define_ast(&args[1], "Expr", vec![
          "Binary   - left: <T: Expr>, operator: Token, right: <S: Expr>",
          "Ternary  - first: <T: Expr>, second: <U: Expr>, third: <V: Expr>",
          "Grouping - expression: <T: Expr>",
          // will have to do something else for literals here
          // maybe an enum
          "Literal  - value: String",
          "Unary    - operator: Token, right: <T: Expr>"
        ]),
        _ => {
            println!("Usage: generate_ast <output directory>");
            Ok(())
        },
    }
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> std::io::Result<()> {
    let path = output_dir.to_owned() + "/" + base_name + ".rs";
    let f = File::create(path)?;
    let mut file = LineWriter::new(f);
    writeln!(file, "// Generated by bin/generate_ast.rs")?;
    writeln!(file, "")?;
    writeln!(file, "use crate::token::Token;")?;
    writeln!(file, "pub trait Expr {{}}")?;
    writeln!(file, "")?;

    // AST types
    for type_ in types.iter() {
        let typename = type_.split('-').collect::<Vec<&str>>()[0].trim();
        let fields = type_.split('-').collect::<Vec<&str>>()[1].trim();
        define_type(&mut file, base_name, typename, fields)?;
    }
    Ok(())
}

fn define_type(file: &mut LineWriter<File>, base_name: &str, typename: &str, fields: &str) -> std::io::Result<()> {
    writeln!(file, "#[derive(Debug, Clone)]")?;
    writeln!(file, "pub struct {} {{", typename)?;
    let fs = fields.split(',').collect::<Vec<&str>>();
    for field in fs.iter() {
        writeln!(file, "\tpub {},", field.trim())?;
    }
    writeln!(file, "}}")?;
    writeln!(file, "impl {} for {} {{}}", base_name, typename)?;
    Ok(())
}
