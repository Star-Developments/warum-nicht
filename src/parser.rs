use crate::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    PrintLiteral(String),
    PrintVar(String),
    AssignLiteral(String, String, String),      // var, value, type
    AssignMath(String, Vec<Token>, String),     // var, expr, type
    Increment(String),
    Decrement(String),
}

pub fn parse(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut ast = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Ausgabe => {
                if let Some(Token::StringLiteral(s)) = tokens.get(i + 1) {
                    ast.push(ASTNode::PrintLiteral(s.clone()));
                    i += 2;
                } else if let Some(Token::Wort(var)) = tokens.get(i + 1) {
                    ast.push(ASTNode::PrintVar(var.clone()));
                    i += 2;
                }
                if matches!(tokens.get(i), Some(Token::Semikolon)) {
                    i += 1;
                }
            }
            Token::Typ(t) => {
                if let Some(Token::Wort(var)) = tokens.get(i + 1) {
                    if matches!(tokens.get(i + 2), Some(Token::Gleich)) {
                        let mut expr_tokens = Vec::new();
                        let mut j = i + 3;
                        while j < tokens.len() && !matches!(tokens[j], Token::Semikolon) {
                            expr_tokens.push(tokens[j].clone());
                            j += 1;
                        }

                        if expr_tokens.len() == 1 {
                            match &expr_tokens[0] {
                                Token::StringLiteral(s) => ast.push(ASTNode::AssignLiteral(var.clone(), s.clone(), t.clone())),
                                Token::Float(f) => ast.push(ASTNode::AssignLiteral(var.clone(), f.to_string(), t.clone())),
                                Token::Zahl(z) => ast.push(ASTNode::AssignLiteral(var.clone(), z.to_string(), t.clone())),
                                _ => ast.push(ASTNode::AssignMath(var.clone(), expr_tokens, t.clone())),
                            }
                        } else {
                            ast.push(ASTNode::AssignMath(var.clone(), expr_tokens, t.clone()));
                        }

                        i = j;
                        if matches!(tokens.get(i), Some(Token::Semikolon)) {
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }
            Token::Wort(var) => {
                if matches!(tokens.get(i + 1), Some(Token::PlusPlus)) {
                    ast.push(ASTNode::Increment(var.clone()));
                    i += 2;
                } else if matches!(tokens.get(i + 1), Some(Token::MinusMinus)) {
                    ast.push(ASTNode::Decrement(var.clone()));
                    i += 2;
                } else {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }

    ast
}
