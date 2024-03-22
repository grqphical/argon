use crate::lexer::Token;

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
pub fn parse_expr(tokens: &[Token]) -> Expr {
    parse_addition_subtraction(tokens, 0).1
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
fn parse_addition_subtraction(tokens: &[Token], i: usize) -> (usize, Expr) {
    let (mut i, mut lhs) = parse_multiplication_division(tokens, i);

    while i < tokens.len() {
        match tokens[i] {
            Token::Plus | Token::Minus => {
                let op = tokens[i].clone();
                let (new_index, rhs) = parse_multiplication_division(tokens, i + 1);
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

    (i, lhs)
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
fn parse_multiplication_division(tokens: &[Token], i: usize) -> (usize, Expr) {
    let (mut i, mut lhs) = parse_exponents(tokens, i);

    while i < tokens.len() {
        match tokens[i] {
            Token::Multiply | Token::Divide | Token::Modulus => {
                let op = tokens[i].clone();
                let (new_index, rhs) = parse_exponents(tokens, i + 1);
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

    (i, lhs)
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
fn parse_exponents(tokens: &[Token], i: usize) -> (usize, Expr) {
    let (mut i, mut lhs) = parse_unary(tokens, i);

    while i < tokens.len() {
        match tokens[i] {
            Token::Power => {
                let op = tokens[i].clone();
                let (new_index, rhs) = parse_unary(tokens, i + 1);
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

    (i, lhs)
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
fn parse_unary(tokens: &[Token], i: usize) -> (usize, Expr) {
    match tokens[i] {
        Token::Number(n) => (i + 1, Expr::Number(n)),
        Token::Minus => {
            let (i, rhs) = parse_unary(tokens, i + 1);
            (
                i,
                Expr::UnaryOp {
                    op: Token::Minus,
                    rhs: Box::new(rhs),
                },
            )
        }
        Token::LeftParen => {
            let (i, expr) = parse_addition_subtraction(tokens, i + 1);
            assert_eq!(tokens[i], Token::RightParen);
            (i + 1, expr)
        }
        _ => panic!("Unexpected token: {:?}", tokens[i]),
    }
}
