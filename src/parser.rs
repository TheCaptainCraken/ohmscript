use crate::lexer::Token;
use std::slice::Iter;

type Program = Vec<AST>;

#[derive(Debug, Clone)]
pub enum AST {
    Bind {
        identifier: String,
        expression: Expression,
    },
    Eval(Expression),
}
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    Expression {
        operand: Operand,
        args: Vec<Expression>,
    },
}

pub type Args = Vec<Expression>;
#[derive(Debug, Clone)]
pub enum Operand {
    Series,
    Parallel,
}
#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    NumberAndMultiplier { number: f64, multiplier: Multiplier },
}
#[derive(Debug, Clone)]
pub enum Multiplier {
    K,
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut result: Program = Vec::new();
    let mut statement_tokens = Vec::new();

    for token in tokens {
        if token == Token::EndOfLine {
            let parsed_statement = parse_statement(statement_tokens.clone())?;
            result.push(parsed_statement);
            statement_tokens.clear();
        } else {
            statement_tokens.push(token);
        }
    }

    Ok(result)
}

pub fn parse_statement(tokens: Vec<Token>) -> Result<AST, String> {
    let mut position = tokens.iter();

    match position.next() {
        Some(Token::Evaluator) => {
            if let Some(Token::Equal) = position.next() {
                // evaluate expression
                let expression = parse_expression(&mut position)?;

                let result = AST::Eval(expression);

                return Ok(result);
            }
            Err("Expected = after evaluator".to_string())
        }

        Some(Token::Identifier(identifier)) => {
            if let Some(Token::Equal) = position.next() {
                // evaluate expression

                let expression = parse_expression(&mut position)?;

                let result = AST::Bind {
                    identifier: identifier.to_owned(),
                    expression,
                };

                return Ok(result);
            }

            Err("Expected = after evaluator".to_string())
        }

        _ => Err("Expected identifier or ? at statement start.".to_string()),
    }
}

pub fn parse_expression<'a>(position: &mut Iter<'a, Token>) -> Result<Expression, String> {
    let first = position.next();

    match first {
        Some(Token::Identifier(identifier)) => Ok(Expression::Identifier(identifier.to_owned())),
        Some(Token::Number(number)) => {
            let mut position = position.peekable();
            if let Some(Token::Multiplier(m)) = position.peek() {
                if m == "k" {
                    _ = position.next();
                    Ok(Expression::Literal(Literal::NumberAndMultiplier {
                        number: number.clone(),
                        multiplier: Multiplier::K,
                    }))
                } else {
                    Err("Unexpected multiplier after numeric value in expression".to_string())
                }
            } else {
                Ok(Expression::Literal(Literal::Number(number.clone())))
            }
        }
        Some(Token::Series) => {
            if let Some(Token::LeftParenthesis) = position.next() {
                let args = parse_args(position)?;
                if let Some(Token::RightParenthesis) = position.next() {
                    let result = Expression::Expression {
                        operand: Operand::Series,
                        args,
                    };

                    Ok(result)
                } else {
                    Err("Missing ) for series function in expression.".to_owned())
                }
            } else {
                Err("Missing ( in series function in expression.".to_owned())
            }
        }
        Some(Token::Parallel) => {
            if let Some(Token::LeftParenthesis) = position.next() {
                let args = parse_args(position)?;
                if let Some(Token::RightParenthesis) = position.next() {
                    let result = Expression::Expression {
                        operand: Operand::Parallel,
                        args,
                    };

                    Ok(result)
                } else {
                    Err("Missing ) for parallel function in expression.".to_owned())
                }
            } else {
                Err("Missing ( in parallel function in expression.".to_owned())
            }
        }
        _ => return Err("Unexpected token in expression.".to_string()),
    }
}

fn parse_args<'a>(position: &mut Iter<'a, Token>) -> Result<Args, String> {
    let mut result: Args = Vec::new();

    result.push(parse_expression(position)?);

    let mut position_peekable = position.clone().peekable();

    if let Some(Token::Comma) = position_peekable.peek() {
        _ = position.next();

        result.push(parse_expression(position)?);

        position_peekable = position.clone().peekable();
    }

    Ok(result)
}
