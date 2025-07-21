mod lexer;
mod parser;
mod compiler;

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum VarValue {
    Int(i64),
    Float(f64),
    Str(String),
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Benutzung: wnrun <datei.wn>");
        return;
    }

    let source = std::fs::read_to_string(&args[1]).expect("Datei konnte nicht gelesen werden");
    let tokens = lexer::lex(&source);
    let ast = parser::parse(tokens);

    let mut env: HashMap<String, VarValue> = HashMap::new();

    run_ast(&ast, &mut env);

    let out_file = args[1].replace(".wn", ".wnt");
    compiler::compile(ast, &out_file);
}

fn run_ast(ast: &[parser::ASTNode], env: &mut HashMap<String, VarValue>) {
    for node in ast {
        match node {
            parser::ASTNode::PrintLiteral(text) => println!("{}", text),
            parser::ASTNode::PrintVar(var) => {
                if let Some(v) = env.get(var) {
                    match v {
                        VarValue::Int(n) => println!("{}", n),
                        VarValue::Float(f) => println!("{}", f),
                        VarValue::Str(s) => println!("{}", s),
                    }
                } else {
                    println!("(undefiniert: {})", var);
                }
            }
            parser::ASTNode::AssignLiteral(var, val, typ) => {
                let value = match typ.as_str() {
                    "int" => VarValue::Int(val.parse::<i64>().unwrap_or(0)),
                    "fl" => VarValue::Float(val.parse::<f64>().unwrap_or(0.0)),
                    "string" => VarValue::Str(val.clone()),
                    _ => VarValue::Str(val.clone()),
                };
                env.insert(var.clone(), value);
            }
            parser::ASTNode::AssignMath(var, expr, typ) => {
                let result = eval_math(expr, env);
                env.insert(var.clone(), match typ.as_str() {
                    "int" => VarValue::Int(result as i64),
                    "fl" => VarValue::Float(result),
                    _ => VarValue::Int(result as i64),
                });
            }
            parser::ASTNode::Increment(var) => {
                if let Some(v) = env.get_mut(var) {
                    match v {
                        VarValue::Int(n) => *n += 1,
                        VarValue::Float(f) => *f += 1.0,
                        _ => {}
                    }
                }
            }
            parser::ASTNode::Decrement(var) => {
                if let Some(v) = env.get_mut(var) {
                    match v {
                        VarValue::Int(n) => *n -= 1,
                        VarValue::Float(f) => *f -= 1.0,
                        _ => {}
                    }
                }
            }
        }
    }
}

fn eval_math(expr: &[lexer::Token], env: &HashMap<String, VarValue>) -> f64 {
    // Shunting-Yard: Infix -> Postfix
    let mut output = Vec::new();
    let mut ops = Vec::new();

    for token in expr {
        match token {
            lexer::Token::Zahl(_) | lexer::Token::Float(_) | lexer::Token::Wort(_) => output.push(token.clone()),
            lexer::Token::Plus | lexer::Token::Minus | lexer::Token::Stern | lexer::Token::Slash => {
                while let Some(op) = ops.last() {
                    if precedence(op) >= precedence(token) {
                        output.push(ops.pop().unwrap());
                    } else {
                        break;
                    }
                }
                ops.push(token.clone());
            }
            lexer::Token::KlammerAuf => ops.push(token.clone()),
            lexer::Token::KlammerZu => {
                while let Some(op) = ops.pop() {
                    if matches!(op, lexer::Token::KlammerAuf) {
                        break;
                    }
                    output.push(op);
                }
            }
            _ => {}
        }
    }

    while let Some(op) = ops.pop() {
        output.push(op);
    }

    // Postfix auswerten
    let mut stack: Vec<f64> = Vec::new();
    for token in output {
        match token {
            lexer::Token::Zahl(n) => stack.push(n as f64),
            lexer::Token::Float(f) => stack.push(f),
            lexer::Token::Wort(var) => {
                match env.get(&var) {
                    Some(VarValue::Int(n)) => stack.push(*n as f64),
                    Some(VarValue::Float(f)) => stack.push(*f),
                    _ => stack.push(0.0),
                }
            }
            lexer::Token::Plus => { let b = stack.pop().unwrap_or(0.0); let a = stack.pop().unwrap_or(0.0); stack.push(a + b); }
            lexer::Token::Minus => { let b = stack.pop().unwrap_or(0.0); let a = stack.pop().unwrap_or(0.0); stack.push(a - b); }
            lexer::Token::Stern => { let b = stack.pop().unwrap_or(0.0); let a = stack.pop().unwrap_or(0.0); stack.push(a * b); }
            lexer::Token::Slash => { let b = stack.pop().unwrap_or(1.0); let a = stack.pop().unwrap_or(0.0); stack.push(if b != 0.0 { a / b } else { 0.0 }); }
            _ => {}
        }
    }

    stack.pop().unwrap_or(0.0)
}

fn precedence(token: &lexer::Token) -> i32 {
    match token {
        lexer::Token::Stern | lexer::Token::Slash => 2,
        lexer::Token::Plus | lexer::Token::Minus => 1,
        _ => 0,
    }
}
