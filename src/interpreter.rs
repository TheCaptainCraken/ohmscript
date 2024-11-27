use std::collections::HashMap;

use crate::parser::{Expression, Literal, Multiplier, Operand, Program, AST};

pub struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, program: Program) {
        for statement in program {
            self.execute_statement(&statement);
        }
    }

    fn execute_statement(&mut self, statement: &AST) {
        match statement {
            AST::Bind {
                identifier,
                expression,
            } => {
                let evaluated_expression = self.evaluate_expression(expression);
                self.variables
                    .insert(identifier.to_string(), evaluated_expression);
            }
            AST::Eval(expression) => {
                let evaluated_expression = self.evaluate_expression(expression);
                println!("{}", evaluated_expression);
            }
        }
    }

    fn evaluate_expression(&self, expression: &Expression) -> f64 {
        match expression {
            Expression::Literal(literal) => self.evaluate_literal(literal),
            Expression::Identifier(id) => self.evaluate_identifier(id),
            Expression::Expression { operand, args } => match operand {
                Operand::Series => self.evaluate_series(args),
                Operand::Parallel => self.evaluate_parallel(args),
            },
        }
    }

    fn evaluate_literal(&self, literal: &Literal) -> f64 {
        match literal {
            Literal::Number(n) => n.clone(),
            Literal::NumberAndMultiplier { number, multiplier } => {
                let factor = match multiplier {
                    Multiplier::K => 1000.0,
                };

                number * factor
            }
        }
    }

    fn evaluate_identifier(&self, identifier: &String) -> f64 {
        self.variables[identifier]
    }

    fn evaluate_series(&self, args: &Vec<Expression>) -> f64 {
        args.iter().map(|exp| self.evaluate_expression(exp)).sum()
    }

    fn evaluate_parallel(&self, args: &Vec<Expression>) -> f64 {
        let divider: f64 = args
            .iter()
            .map(|exp| 1.0 / self.evaluate_expression(exp))
            .sum();

        1.0 / divider
    }
}
