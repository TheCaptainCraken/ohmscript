use crate::lexer::Token;
pub type Program = Vec<AST>;

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

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    fn next(&mut self) -> Option<Token> {
        if self.cursor + 1 == self.tokens.len() {
            None
        } else {
            let result = Some(self.tokens[self.cursor].clone());
            self.cursor += 1;

            result
        }
    }

    fn peek(&self) -> Option<Token> {
        if self.cursor + 1 == self.tokens.len() {
            None
        } else {
            Some(self.tokens[self.cursor].clone())
        }
    }

    // consumes the value
    fn expect(&mut self, token: Token) -> Result<(), String> {
        let next = self.next();

        if next.is_some_and(|t| t == token) {
            Ok(())
        } else {
            Err(format!("Expected {:?}.", token))
        }
    }

    // does not consume the value
    fn check(&self, token: Token) -> bool {
        let peek = self.peek();

        peek.is_some_and(|t| t == token)
    }

    fn is_empty(&self) -> bool {
        self.peek().is_none()
    }

    fn init_parser(tokens: Vec<Token>) -> Parser {
        Parser { tokens, cursor: 0 }
    }

    pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
        let mut parser = Self::init_parser(tokens);

        let result = parser.parse_statements()?;

        Ok(result)
    }

    fn parse_statements(&mut self) -> Result<Program, String> {
        while self.check(Token::EndOfLine) {
            _ = self.next();
        }

        let mut result = Vec::new();

        loop {
            let parsed_statement = self.parse_statement()?;
            result.push(parsed_statement);

            if self.is_empty() {
                break;
            }

            _ = self.expect(Token::EndOfLine)?
        }

        Ok(result)
    }

    fn parse_statement(&mut self) -> Result<AST, String> {
        match self.next() {
            Some(Token::Identifier(identifier)) => {
                _ = self.expect(Token::Equal)?;

                let expression = self.parse_expression()?;

                let ast = AST::Bind {
                    identifier,
                    expression,
                };

                Ok(ast)
            }

            Some(Token::Evaluator) => {
                _ = self.expect(Token::Equal)?;

                let expression = self.parse_expression()?;

                let ast = AST::Eval(expression);

                Ok(ast)
            }

            _ => Err("Expected evaluator or identifier, found none.".to_string()),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        match self.peek() {
            Some(Token::Identifier(identifier)) => {
                _ = self.next();
                Ok(Expression::Identifier(identifier))
            }
            Some(Token::Number(_)) => {
                let number_literal = self.parse_number()?;
                Ok(Expression::Literal(number_literal))
            }

            Some(Token::Parallel) => {
                _ = self.next();
                _ = self.expect(Token::LeftParenthesis)?;
                let args = self.parse_args()?;
                _ = self.expect(Token::RightParenthesis)?;

                Ok(Expression::Expression {
                    operand: Operand::Parallel,
                    args,
                })
            }

            Some(Token::Series) => {
                _ = self.next();
                _ = self.expect(Token::LeftParenthesis)?;
                let args = self.parse_args()?;
                _ = self.expect(Token::RightParenthesis)?;

                Ok(Expression::Expression {
                    operand: Operand::Series,
                    args,
                })
            }

            _ => Err("Expected expression".to_string()),
        }
    }

    fn parse_number(&mut self) -> Result<Literal, String> {
        if let Some(Token::Number(n)) = self.next() {
            if let Some(Token::Multiplier(m)) = self.peek() {
                if m == "k" {
                    _ = self.next();
                    let number = Literal::NumberAndMultiplier {
                        number: n,
                        multiplier: Multiplier::K,
                    };

                    Ok(number)
                } else {
                    Err("Unknown multiplier".to_string())
                }
            } else {
                Ok(Literal::Number(n))
            }
        } else {
            Err("Expected number.".to_string())
        }
    }

    fn parse_args(&mut self) -> Result<Args, String> {
        let mut result = Vec::new();

        let expression = self.parse_expression()?;

        result.push(expression);

        while self.check(Token::Comma) {
            _ = self.next();
            let expression = self.parse_expression()?;
            result.push(expression);
        }

        Ok(result)
    }
}
