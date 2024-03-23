use anyhow::{format_err, Result};

/// Enum that represents a token in the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Modulus,
    RightParen,
    LeftParen,
    Identifier(String),
    Equals,
}

/// Preprocesses the equation by replacing constants with their values.
///
/// # Example
/// ```
/// let equation = "PI + E".to_string();
/// let processed_equation = lexer::preprocessor(equation);
/// assert_eq!(processed_equation, "3.14159265358979 + 2.718281828459045");
fn preprocessor(equation: String) -> String {
    let mut processed_equation = equation.replace("PI", "3.14159265358979");
    processed_equation = processed_equation.replace("E", "2.718281828459045");
    processed_equation = processed_equation.replace("TAU", "6.283185307179586");
    return processed_equation;
}

/// Generates a number token from the equation.
///
/// # Example
/// ```
/// let equation = "123.456".to_string();
/// let mut index = 0;
/// let token = lexer::make_number(&equation, &mut index).unwrap();
/// assert_eq!(token, lexer::Token::Number(123.456));
fn make_number(equation: &String, index: &mut usize) -> Result<Token> {
    let mut char = equation.chars().nth(*index).unwrap_or(' ');
    let mut dot_count = 0;
    let mut num_string = String::new();

    loop {
        if char == '.' {
            if dot_count == 1 {
                return Err(format_err!("Unexpected token '.'"));
            }
            dot_count += 1;
            num_string.push(char);
        } else if char.is_numeric() {
            num_string.push(char)
        } else {
            break;
        }
        *index += 1;
        char = equation.chars().nth(*index).unwrap_or(' ');
    }

    let num: f64 = num_string.parse().unwrap();
    return Ok(Token::Number(num));
}

fn make_identifier(equation: &String, index: &mut usize) -> Result<Token> {
    let mut char = equation.chars().nth(*index).unwrap_or(' ');
    let mut identifier = String::new();

    loop {
        if char.is_alphanumeric() {
            identifier.push(char);
        } else {
            break;
        }
        *index += 1;
        char = equation.chars().nth(*index).unwrap_or(' ');
    }

    return Ok(Token::Identifier(identifier));
}

/// The main lexer function that generates tokens from the equation.
///
/// # Example
/// ```
/// let equation = "1 + 2".to_string();
/// let tokens = lexer::generate_tokens(equation).unwrap();
/// assert_eq!(tokens, vec![lexer::Token::Number(1.0), lexer::Token::Plus, lexer::Token::Number(2.0)]);
pub fn generate_tokens(equation: String) -> Result<Vec<Token>> {
    let mut result = Vec::new();
    let equation = preprocessor(equation);

    let mut index: usize = 0;
    while index < equation.len() {
        let char = equation.chars().nth(index).unwrap_or(' ');
        if char.is_numeric() {
            let token = make_number(&equation, &mut index);
            match token {
                Ok(token) => result.push(token),
                Err(err) => {
                    return Err(err);
                }
            }
            continue;
        }

        if char.is_alphabetic() {
            let token = make_identifier(&equation, &mut index);
            match token {
                Ok(token) => result.push(token),
                Err(err) => {
                    return Err(err);
                }
            }
            continue;
        }

        match char {
            '+' => result.push(Token::Plus),
            '-' => result.push(Token::Minus),
            '*' => result.push(Token::Multiply),
            '/' => result.push(Token::Divide),
            '(' => result.push(Token::LeftParen),
            ')' => result.push(Token::RightParen),
            '^' => result.push(Token::Power),
            '%' => result.push(Token::Modulus),
            '=' => result.push(Token::Equals),
            ' ' | '\n' | '\t' | '\r' => (),
            _ => return Err(format_err!("Unknown character '{}'", char)),
        }
        index += 1;
    }

    return Ok(result);
}
