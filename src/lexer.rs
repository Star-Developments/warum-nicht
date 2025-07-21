#[derive(Debug, Clone)]
pub enum Token {
    Wort(String),
    Zahl(i64),
    Float(f64),
    StringLiteral(String),
    Typ(String),
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
    BlockStart,
    BlockEnd,
    Wenn,
    Sonst,
    Vergleich(String),
}

pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut in_block_comment = false;

    for line_raw in source.lines() {
        let mut line = line_raw.trim().to_string();

        if line.starts_with('<') {
            in_block_comment = true;
            continue;
        }
        if line.ends_with("/>") {
            in_block_comment = false;
            continue;
        }
        if in_block_comment || line.starts_with("</>") || line.is_empty() {
            continue;
        }

        let mut buffer = String::new();
        for ch in line.chars() {
            match ch {
                '{' => {
                    if !buffer.trim().is_empty() {
                        process_word(&mut buffer, &mut tokens);
                    }
                    tokens.push(Token::BlockStart);
                }
                '}' => {
                    if !buffer.trim().is_empty() {
                        process_word(&mut buffer, &mut tokens);
                    }
                    tokens.push(Token::BlockEnd);
                }
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
        "wenn" => tokens.push(Token::Wenn),
        "sonst" => tokens.push(Token::Sonst),
        "int" | "fl" | "string" => tokens.push(Token::Typ(word.clone())),
        "=" => tokens.push(Token::Gleich),
        "++" => tokens.push(Token::PlusPlus),
        "--" => tokens.push(Token::MinusMinus),
        "+" => tokens.push(Token::Plus),
        "-" => tokens.push(Token::Minus),
        "*" => tokens.push(Token::Stern),
        "/" => tokens.push(Token::Slash),
        ">" | "<" | ">=" | "<=" | "==" | "!=" => tokens.push(Token::Vergleich(word.clone())),
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
