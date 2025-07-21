use crate::parser::{ASTNode, VarType};
use crate::lexer::Token;
use std::collections::HashMap;

pub fn run(ast: Vec<ASTNode>) {
    let mut vars: HashMap<String, VarType> = HashMap::new();

    for node in ast {
        match node {
            ASTNode::Assign(name, value) => {
                vars.insert(name, value);
            }
            ASTNode::AssignExpr(name, expr_tokens, is_float) => {
                let val = eval_expr(&expr_tokens, &vars, is_float);
                vars.insert(name, val);
            }
            ASTNode::PrintExpr(name) => {
                if let Some(value) = vars.get(&name) {
                    match value {
                        VarType::Int(i) => println!("{}", i),
                        VarType::Float(f) => {
                            if (f.fract() - 0.0).abs() < std::f64::EPSILON {
                                println!("{}", *f as i64);
                            } else {
                                println!("{}", f);
                            }
                        }
                        VarType::Str(s) => println!("{}", s),
                    }
                }
            }
        }
    }
}

fn eval_expr(tokens: &Vec<Token>, vars: &HashMap<String, VarType>, force_float: bool) -> VarType {
    fn parse_expression(tokens: &[Token], vars: &HashMap<String, VarType>, pos: &mut usize) -> f64 {
        let mut values: Vec<f64> = Vec::new();
        let mut ops: Vec<char> = Vec::new();

        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Zahl(n) => {
                    values.push(*n as f64);
                    *pos += 1;
                }
                Token::Float(n) => {
                    values.push(*n);
                    *pos += 1;
                }
                Token::Wort(name) => {
                    if let Some(v) = vars.get(name) {
                        match v {
                            VarType::Int(i) => values.push(*i as f64),
                            VarType::Float(f) => values.push(*f),
                            _ => {}
                        }
                    }
                    *pos += 1;
                }
                Token::KlammerAuf => {
                    *pos += 1;
                    let inner_val = parse_expression(tokens, vars, pos);
                    values.push(inner_val);
                }
                Token::KlammerZu => {
                    *pos += 1;
                    break;
                }
                Token::Plus => {
                    ops.push('+');
                    *pos += 1;
                }
                Token::Minus => {
                    ops.push('-');
                    *pos += 1;
                }
                Token::Stern => {
                    ops.push('*');
                    *pos += 1;
                }
                Token::Slash => {
                    ops.push('/');
                    *pos += 1;
                }
                _ => {
                    *pos += 1;
                }
            }
        }

        // Multiplikation/Division zuerst
        let mut i = 0;
        while i < ops.len() {
            if ops[i] == '*' || ops[i] == '/' {
                let (a, b) = (values[i], values[i + 1]);
                let res = if ops[i] == '*' { a * b } else { a / b };
                values[i] = res;
                values.remove(i + 1);
                ops.remove(i);
            } else {
                i += 1;
            }
        }

        // Dann Addition/Subtraktion
        i = 0;
        while i < ops.len() {
            let (a, b) = (values[i], values[i + 1]);
            let res = if ops[i] == '+' { a + b } else { a - b };
            values[i] = res;
            values.remove(i + 1);
            ops.remove(i);
        }

        values.get(0).cloned().unwrap_or(0.0)
    }

    let mut pos = 0;
    let result = parse_expression(tokens, vars, &mut pos);
    if force_float {
        VarType::Float(result)
    } else if (result.fract() - 0.0).abs() < std::f64::EPSILON {
        VarType::Int(result as i64)
    } else {
        VarType::Float(result)
    }
}
