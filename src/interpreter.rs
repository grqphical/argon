use std::collections::HashMap;

use crate::{functions::CalculatorFunction, lexer::Token, parser::Expr};
use anyhow::{format_err, Result};

/// Interprets the AST and returns the result. If an unexpected operator is found, it returns an error.
pub fn interpret(
    expr: &Expr,
    variables: &mut HashMap<String, f64>,
    functions: &mut HashMap<String, CalculatorFunction>,
) -> Result<f64> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::UnaryOp { op, rhs } => {
            let rhs = interpret(rhs, variables, functions)?;
            match op {
                Token::Minus => Ok(-rhs),
                _ => Err(format_err!("Unexpected unary operator")),
            }
        }
        Expr::BinaryOp { lhs, op, rhs } => {
            let lhs = interpret(lhs, variables, functions)?;
            let rhs = interpret(rhs, variables, functions)?;
            match op {
                Token::Plus => Ok(lhs + rhs),
                Token::Minus => Ok(lhs - rhs),
                Token::Multiply => Ok(lhs * rhs),
                Token::Divide => Ok(lhs / rhs),
                Token::Power => Ok(lhs.powf(rhs)),
                Token::Modulus => Ok(lhs % rhs),
                _ => Err(format_err!("Unexpected binary operator")),
            }
        }
        Expr::VariableDeclaration { name, value } => {
            variables.insert(name.to_string(), *value);
            Ok(*value)
        }
        Expr::Function { name, args } => {
            let args = args
                .iter()
                .map(|arg| interpret(arg, variables, functions))
                .collect::<Result<Vec<f64>>>()?;

            functions
                .get(name)
                .ok_or(format_err!("Function not found"))?(args)
        }
    }
}
