
mod lexer;
mod parser;
mod compiler;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <sourcefile>", args[0]);
        return;
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Could not read file");

    let tokens = lexer::lex(&source);
    let ast = parser::parse(tokens);
    compiler::run(ast);
}
