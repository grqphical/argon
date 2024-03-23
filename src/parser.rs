use std::collections::HashMap;

use crate::lexer::Token;
use anyhow::{format_err, Result};

/// Enum that represents an expression in the AST.
///
/// `lhs` stands for left hand side, `rhs` stands for right hand side.
///
/// `UnaryOp` represents unary operations such as negation.
///
/// `BinaryOp` represents binary operations such as addition and subtraction.
///
/// `Number` represents a number.
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    UnaryOp {
        op: Token,
        rhs: Box<Expr>,
    },
    BinaryOp {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
    },
    VariableDeclaration {
        name: String,
        value: f64,
    },
}

/// Parses a list of tokens into an AST.
///
/// # Example
/// ```
/// let tokens = vec![
///   lexer::Token::Number(1.0),
///   lexer::Token::Plus,
///   lexer::Token::Number(2.0),
/// ];
/// let ast = parser::parse_expr(&tokens);
/// assert_eq!(ast, parser::Expr::BinaryOp {
///     lhs: Box::new(parser::Expr::Number(1.0)),
///     op: lexer::Token::Plus,
///     rhs: Box::new(parser::Expr::Number(2.0)),
/// });
/// ```
pub fn parse_expr(tokens: &[Token], variables: &mut HashMap<String, f64>) -> Result<Expr> {
    let (_, expr) = parse_variable_declaration(tokens, 0, variables)?;
    return Ok(expr);
}

/// Parses the addition and subtraction operations. Returning the index and the AST.
///
/// # Example
/// ```
/// let tokens = vec![
///     lexer::Token::Number(1.0),
///     lexer::Token::Plus,
///     lexer::Token::Number(2.0),
///     lexer::Token::Minus,
///     lexer::Token::Number(3.0),
/// ];
/// let (index, ast) = parser::parse_addition_subtraction(&tokens, 0);
/// assert_eq!(index, 5);
/// assert_eq!(ast, parser::Expr::BinaryOp {
///     lhs: Box::new(parser::Expr::BinaryOp {
///     lhs: Box::new(parser::Expr::Number(1.0)),
///     op: lexer::Token::Plus,
///     rhs: Box::new(parser::Expr::Number(2.0)),
///     }),
///     op: lexer::Token::Minus,
///     rhs: Box::new(parser::Expr::Number(3.0)),
/// });
/// ```
fn parse_addition_subtraction(
    tokens: &[Token],
    i: usize,
    variables: &mut HashMap<String, f64>,
) -> Result<(usize, Expr)> {
    let (mut i, mut lhs) = parse_multiplication_division(tokens, i, variables)?;

    while i < tokens.len() {
        match tokens[i] {
            Token::Plus | Token::Minus => {
                let op = tokens[i].clone();
                let (new_index, rhs) = parse_multiplication_division(tokens, i + 1, variables)?;
                i = new_index;
                lhs = Expr::BinaryOp {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                };
            }
            _ => break,
        }
    }

    Ok((i, lhs))
}
/// Parses the multiplication and division operations. Returning the index and the AST.
///
/// # Example
/// ```
/// let tokens = vec![
///     lexer::Token::Number(1.0),
///     lexer::Token::Multiply,
///     lexer::Token::Number(2.0),
///     lexer::Token::Divide,
///     lexer::Token::Number(3.0),
/// ];
/// let (index, ast) = parser::parse_multiplication_division(&tokens, 0);
/// assert_eq!(index, 5);
/// assert_eq!(ast, parser::Expr::BinaryOp {
///     lhs: Box::new(parser::Expr::BinaryOp {
///     lhs: Box::new(parser::Expr::Number(1.0)),
///     op: lexer::Token::Multiply,
///     rhs: Box::new(parser::Expr::Number(2.0)),
///     }),
///     op: lexer::Token::Divide,
///     rhs: Box::new(parser::Expr::Number(3.0)),
/// });
/// ```
fn parse_multiplication_division(
    tokens: &[Token],
    i: usize,
    variables: &mut HashMap<String, f64>,
) -> Result<(usize, Expr)> {
    let (mut i, mut lhs) = parse_exponents(tokens, i, variables)?;

    while i < tokens.len() {
        match tokens[i] {
            Token::Multiply | Token::Divide | Token::Modulus => {
                let op = tokens[i].clone();
                let (new_index, rhs) = parse_exponents(tokens, i + 1, variables)?;
                i = new_index;
                lhs = Expr::BinaryOp {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                };
            }
            _ => break,
        }
    }

    Ok((i, lhs))
}

/// Parses the exponentiation operations. Returning the index and the AST.
///
/// # Example
/// ```
/// let tokens = vec![
///     lexer::Token::Number(2.0),
///     lexer::Token::Power,
///     lexer::Token::Number(3.0),
/// ];
/// let (index, ast) = parser::parse_exponents(&tokens, 0);
/// assert_eq!(index, 3);
/// assert_eq!(ast, parser::Expr::BinaryOp {
///     lhs: Box::new(parser::Expr::Number(2.0)),
///     op: lexer::Token::Power,
///     rhs: Box::new(parser::Expr::Number(3.0)),
/// });
/// ```
fn parse_exponents(
    tokens: &[Token],
    i: usize,
    variables: &mut HashMap<String, f64>,
) -> Result<(usize, Expr)> {
    let (mut i, mut lhs) = parse_unary(tokens, i, variables)?;

    while i < tokens.len() {
        match tokens[i] {
            Token::Power => {
                let op = tokens[i].clone();
                let (new_index, rhs) = parse_unary(tokens, i + 1, variables)?;
                i = new_index;
                lhs = Expr::BinaryOp {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                };
            }
            _ => break,
        }
    }

    Ok((i, lhs))
}

/// Parses the unary operations such as negation. Returning the index and the AST.
///
/// # Example
/// ```
/// let tokens = vec![
///     lexer::Token::Minus,
///     lexer::Token::Number(1.0),
/// ];
/// let (index, ast) = parser::parse_unary(&tokens, 0);
/// assert_eq!(index, 2);
/// assert_eq!(ast, parser::Expr::UnaryOp {
///     op: lexer::Token::Minus,
///     rhs: Box::new(parser::Expr::Number(1.0)),
/// });
/// ```
fn parse_unary(
    tokens: &[Token],
    i: usize,
    variables: &mut HashMap<String, f64>,
) -> Result<(usize, Expr)> {
    match &tokens[i] {
        Token::Number(n) => Ok((i + 1, Expr::Number(*n))),
        Token::Identifier(name) => match variables.get(name) {
            Some(value) => Ok((i + 1, Expr::Number(*value))),
            None => Err(format_err!("Variable not found")),
        },
        Token::Minus => {
            let (i, rhs) = parse_addition_subtraction(tokens, i + 1, variables)?;
            Ok((
                i,
                Expr::UnaryOp {
                    op: Token::Minus,
                    rhs: Box::new(rhs),
                },
            ))
        }
        Token::LeftParen => {
            let (index, expr) = parse_addition_subtraction(tokens, i + 1, variables)?;
            if tokens[index] == Token::RightParen {
                Ok((index + 1, expr))
            } else {
                Err(format_err!("Expected right parenthesis"))
            }
        }
        _ => Err(format_err!("Unexpected token: {:?}", tokens[i])),
    }
}

/// Parses the variable declaration. Returning the index and the AST.
///
/// # Example
/// ```
/// let tokens = vec![
///     lexer::Token::Identifier("x".to_string()),
///     lexer::Token::Equals,
///     lexer::Token::Number(1.0),
/// ];
/// let (index, ast) = parser::parse_variable_declaration(&tokens, 0);
/// assert_eq!(index, 3);
/// assert_eq!(ast, parser::Expr::Variable {
///     name: "x".to_string(),
///     value: 1.0,
/// });
/// ```
fn parse_variable_declaration(
    tokens: &[Token],
    i: usize,
    variables: &mut HashMap<String, f64>,
) -> Result<(usize, Expr)> {
    match &tokens[i] {
        Token::Identifier(name) => {
            if tokens.get(i + 1) == Some(&Token::Equals) {
                let (index, expr) = parse_addition_subtraction(tokens, i + 2, variables)?;
                Ok((
                    index,
                    Expr::VariableDeclaration {
                        name: name.clone(),
                        value: match expr {
                            Expr::Number(n) => n,
                            _ => return Err(format_err!("Expected number")),
                        },
                    },
                ))
            } else {
                parse_addition_subtraction(tokens, i, variables)
            }
        }
        _ => parse_addition_subtraction(tokens, i, variables),
    }
}
