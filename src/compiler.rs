use crate::parser::ASTNode;
use std::fs::File;
use std::io::Write;

pub fn compile(ast: Vec<ASTNode>, out: &str) {
    let mut compiled = String::new();
    for node in ast {
        compiled.push_str(&format!("{:?}\n", node));
    }
    let mut file = File::create(out).expect("Kann Datei nicht erstellen");
    file.write_all(compiled.as_bytes()).unwrap();
}
