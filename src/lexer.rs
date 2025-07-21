#[derive(Debug, Clone)]
pub enum Token {
    Wort(String),
    Zahl(i64),
    Float(f64),
    StringLiteral(String),
    Typ(String),           // int, fl, string
    Ausgabe,
    Gleich,
    Plus,
    Minus,
    Stern,
    Slash,
    PlusPlus,
    MinusMinus,
    Semikolon,
    KlammerAuf,
    KlammerZu,
}

pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut in_block_comment = false;

    for line_raw in source.lines() {
        let mut line = line_raw.trim().to_string();

       if in_block_comment {
    if line.contains("/>") {
        in_block_comment = false;
    }
    continue;
}
if line.contains('<') {
    in_block_comment = true;
    continue;
}

        // Manuelle Tokenisierung (trennt Klammern und Semikolons korrekt)
        let mut buffer = String::new();
        for ch in line.chars() {
            match ch {
                '(' => {
                    if !buffer.trim().is_empty() {
                        process_word(&mut buffer, &mut tokens);
                    }
                    tokens.push(Token::KlammerAuf);
                }
                ')' => {
                    if !buffer.trim().is_empty() {
                        process_word(&mut buffer, &mut tokens);
                    }
                    tokens.push(Token::KlammerZu);
                }
                ';' => {
                    if !buffer.trim().is_empty() {
                        process_word(&mut buffer, &mut tokens);
                    }
                    tokens.push(Token::Semikolon);
                }
                ' ' => {
                    if !buffer.trim().is_empty() {
                        process_word(&mut buffer, &mut tokens);
                        buffer.clear();
                    }
                }
                _ => buffer.push(ch),
            }
        }
        if !buffer.trim().is_empty() {
            process_word(&mut buffer, &mut tokens);
        }
    }

    tokens
}

fn process_word(word_buf: &mut String, tokens: &mut Vec<Token>) {
    let word = word_buf.clone();
    word_buf.clear();

    match word.as_str() {
        "ausgabe" => tokens.push(Token::Ausgabe),
        "int" | "fl" | "string" => tokens.push(Token::Typ(word.clone())),
        "=" => tokens.push(Token::Gleich),
        "++" => tokens.push(Token::PlusPlus),
        "--" => tokens.push(Token::MinusMinus),
        "+" => tokens.push(Token::Plus),
        "-" => tokens.push(Token::Minus),
        "*" => tokens.push(Token::Stern),
        "/" => tokens.push(Token::Slash),
        "" => {}
        _ => {
            if word.starts_with('"') && word.ends_with('"') {
                tokens.push(Token::StringLiteral(word.trim_matches('"').to_string()));
            } else if let Ok(f) = word.parse::<f64>() {
                if word.contains('.') {
                    tokens.push(Token::Float(f));
                } else {
                    tokens.push(Token::Zahl(f as i64));
                }
            } else {
                tokens.push(Token::Wort(word));
            }
        }
    }
}
