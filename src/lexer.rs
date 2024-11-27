#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Equal,
    Parallel,
    Series,
    Identifier(String),
    Number(f64),
    Multiplier(String),
    Comma,
    Evaluator,
    LeftParenthesis,
    RightParenthesis,
    EndOfLine,
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();

    let mut current_position = input.chars().peekable();

    let mut line_number = 1;

    while let Some(&current_character) = current_position.peek() {
        match current_character {
            ' ' | '\t' | '\r' => _ = current_position.next(),
            '\n' => {
                line_number += 1;
                _ = current_position.next();
                result.push(Token::EndOfLine);
            }
            '=' => {
                result.push(Token::Equal);
                _ = current_position.next();
            }

            '?' => {
                result.push(Token::Evaluator);
                _ = current_position.next();
            }
            ')' => {
                result.push(Token::RightParenthesis);
                _ = current_position.next();
            }
            '(' => {
                result.push(Token::LeftParenthesis);
                _ = current_position.next();
            }

            ',' => {
                result.push(Token::Comma);
                _ = current_position.next();
            }

            '-' => {
                _ = current_position.next();

                let ch = current_position.peek();

                if let Some('>') = ch {
                    _ = current_position.next();
                    result.push(Token::Series);
                } else {
                    return Err(format!("Error on line {}", line_number));
                }
            }

            '/' => {
                _ = current_position.next();

                let ch = current_position.peek();

                if let Some('/') = ch {
                    _ = current_position.next();
                    result.push(Token::Parallel);
                } else {
                    return Err(format!("Error on line {}", line_number));
                }
            }

            '0'..'9' => {
                let mut n = current_character
                    .to_string()
                    .parse::<f64>()
                    .expect("Already tested to be a digit.");

                _ = current_position.next();
                let mut next_digit = current_position.peek();
                let mut add_k = false;

                while let Some(&i) = next_digit {
                    if !i.is_numeric() {
                        if i == '.' {
                            let mut d = 10.0;
                            current_position.next();
                            next_digit = current_position.peek();

                            while let Some(&j) = next_digit {
                                if !j.is_numeric() {
                                    next_digit = None;
                                } else {
                                    let f = j
                                        .to_string()
                                        .parse::<f64>()
                                        .expect("Character not a digit.");
                                    n = n + f / d;
                                    d = d * 10.0;
                                    current_position.next();
                                    next_digit = current_position.peek();
                                }
                            }
                        } else if i == 'k' {
                            _ = current_position.next();
                            next_digit = None;
                            add_k = true;
                        } else {
                            next_digit = None;
                        }
                    } else {
                        let digit = i
                            .to_string()
                            .parse::<f64>()
                            .expect("Character not a digit.");
                        n = n * 10.0 + digit;
                        current_position.next();
                        next_digit = current_position.peek();
                    }
                }

                result.push(Token::Number(n));
                if add_k {
                    result.push(Token::Multiplier("k".to_string()));
                }
            }

            ('a'..'z') | ('A'..'Z') => {
                let mut identifier = String::from(current_character);

                _ = current_position.next();
                let mut next_character = current_position.peek();

                while let Some(&character) = next_character {
                    if character.is_alphanumeric() || character == '_' {
                        identifier.push(character);
                        _ = current_position.next();
                        next_character = current_position.peek();
                    } else {
                        next_character = None;
                    }
                }

                result.push(Token::Identifier(identifier));
            }

            _ => return Err(format!("Error on line {}", line_number)),
        }
    }

    Ok(result)
}
