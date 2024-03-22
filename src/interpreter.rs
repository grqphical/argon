use crate::{lexer::Token, parser::Expr};
use anyhow::{format_err, Result};

/// Interprets the AST and returns the result. If an unexpected operator is found, it returns an error.
pub fn interpret(expr: &Expr) -> Result<f64> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::UnaryOp { op, rhs } => {
            let rhs = interpret(rhs)?;
            match op {
                Token::Minus => Ok(-rhs),
                _ => Err(format_err!("Unexpected unary operator")),
            }
        }
        Expr::BinaryOp { lhs, op, rhs } => {
            let lhs = interpret(lhs)?;
            let rhs = interpret(rhs)?;
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
    }
}
