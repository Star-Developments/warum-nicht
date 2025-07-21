
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum VarType {
    Int(i64),
    Float(f64),
    Str(String),
}

#[derive(Debug)]
pub enum ASTNode {
    PrintExpr(String),                       // Variable ausgeben
    Assign(String, VarType),                 // Zuweisung mit festem Wert
    AssignExpr(String, Vec<Token>, bool),    // Mathematische Expression (true = Float)
}

pub fn parse(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut ast = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Ausgabe => {
                if let Some(Token::Wort(var)) = tokens.get(i + 1) {
                    ast.push(ASTNode::PrintExpr(var.clone()));
                    i += 2;
                } else if let Some(Token::StringLiteral(s)) = tokens.get(i + 1) {
                    ast.push(ASTNode::Assign("__tmp_print".to_string(), VarType::Str(s.clone())));
                    ast.push(ASTNode::PrintExpr("__tmp_print".to_string()));
                    i += 2;
                }
                if matches!(tokens.get(i), Some(Token::Semikolon)) {
                    i += 1;
                }
            }
            Token::Typ(t) => {
                let typename = t.clone();
                if let Some(Token::Wort(var)) = tokens.get(i + 1) {
                    if let Some(eq) = tokens.get(i + 2) {
                        if matches!(eq, Token::Gleich) {
                            let mut expr_tokens = Vec::new();
                            let mut j = i + 3;
                            while j < tokens.len() && !matches!(tokens[j], Token::Semikolon) {
                                expr_tokens.push(tokens[j].clone());
                                j += 1;
                            }
                            let is_float = typename == "fl";
                            if typename == "string" && expr_tokens.len() == 1 {
                                if let Token::StringLiteral(s) = &expr_tokens[0] {
                                    ast.push(ASTNode::Assign(var.clone(), VarType::Str(s.clone())));
                                }
                            } else {
                                ast.push(ASTNode::AssignExpr(var.clone(), expr_tokens, is_float));
                            }
                            i = j + 1;
                            continue;
                        }
                    }
                }
                i += 1;
            }
            _ => i += 1,
        }
    }

    ast
}
