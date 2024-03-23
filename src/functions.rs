use anyhow::Result;
use std::collections::HashMap;

/// Type alias for calculator functions.
pub type CalculatorFunction = fn(Vec<f64>) -> Result<f64>;

/// Loads the calculator functions into a HashMap.
///
/// # Returns
///
/// A HashMap containing the calculator functions.
pub fn load_functions() -> HashMap<String, CalculatorFunction> {
    let mut functions: HashMap<String, CalculatorFunction> = HashMap::new();

    functions.insert("sqrt".to_string(), sqrt);
    functions.insert("sin".to_string(), sin);
    functions.insert("cos".to_string(), cos);
    functions.insert("tan".to_string(), tan);
    functions.insert("asin".to_string(), asin);
    functions.insert("acos".to_string(), acos);
    functions.insert("atan".to_string(), atan);
    functions.insert("sinh".to_string(), sinh);
    functions.insert("cosh".to_string(), cosh);
    functions.insert("tanh".to_string(), tanh);
    functions.insert("asinh".to_string(), asinh);
    functions.insert("acosh".to_string(), acosh);
    functions.insert("atanh".to_string(), atanh);
    functions.insert("floor".to_string(), floor);
    functions.insert("ceil".to_string(), ceil);
    functions.insert("round".to_string(), round);
    functions.insert("recip".to_string(), recip);
    functions.insert("abs".to_string(), abs);
    functions.insert("rad2deg".to_string(), rad2deg);
    functions.insert("deg2rad".to_string(), deg2rad);
    functions.insert("ln".to_string(), ln);
    functions.insert("log10".to_string(), log10);
    functions.insert("log2".to_string(), log2);
    functions.insert("log".to_string(), log);
    functions.insert("factorial".to_string(), factorial);
    functions.insert("pow".to_string(), pow);
    functions.insert("max".to_string(), max);
    functions.insert("min".to_string(), min);
    functions.insert("sum".to_string(), sum);
    functions.insert("avg".to_string(), avg);
    functions.insert("median".to_string(), median);

    functions
}

/// Calculates the square root of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The square root of the number.
pub fn sqrt(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].sqrt())
}

/// Converts an angle from radians to degrees.
///
/// # Arguments
///
/// * `args` - A vector containing a single angle in radians.
///
/// # Returns
///
/// The angle converted to degrees.
pub fn rad2deg(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].to_degrees())
}

/// Converts an angle from degrees to radians.
///
/// # Arguments
///
/// * `args` - A vector containing a single angle in degrees.
///
/// # Returns
///
/// The angle converted to radians.
pub fn deg2rad(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].to_radians())
}

/// Calculates the absolute value of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The absolute value of the number.
pub fn abs(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].abs())
}

/// Rounds a number up to the nearest integer.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The number rounded up to the nearest integer.
pub fn ceil(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].ceil())
}

/// Rounds a number down to the nearest integer.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The number rounded down to the nearest integer.
pub fn floor(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].floor())
}

/// Rounds a number to the nearest integer.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The number rounded to the nearest integer.
pub fn round(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].round())
}

/// Calculates the reciprocal of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The reciprocal of the number.
pub fn recip(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].recip())
}

/// Calculates the sine of an angle.
///
/// # Arguments
///
/// * `args` - A vector containing a single angle.
///
/// # Returns
///
/// The sine of the angle.
pub fn sin(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].sin())
}

/// Calculates the cosine of an angle.
///
/// # Arguments
///
/// * `args` - A vector containing a single angle.
///
/// # Returns
///
/// The cosine of the angle.
pub fn cos(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].cos())
}

/// Calculates the tangent of an angle.
///
/// # Arguments
///
/// * `args` - A vector containing a single angle.
///
/// # Returns
///
/// The tangent of the angle.
pub fn tan(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].tan())
}

/// Calculates the arcsine of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The arcsine of the number.
pub fn asin(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].asin())
}

/// Calculates the arccosine of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The arccosine of the number.
pub fn acos(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].acos())
}

/// Calculates the arctangent of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The arctangent of the number.
pub fn atan(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].atan())
}

/// Calculates the hyperbolic sine of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The hyperbolic sine of the number.
pub fn sinh(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].sinh())
}

/// Calculates the hyperbolic cosine of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The hyperbolic cosine of the number.
pub fn cosh(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].cosh())
}

/// Calculates the hyperbolic tangent of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The hyperbolic tangent of the number.
pub fn tanh(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].tanh())
}

/// Calculates the inverse hyperbolic sine of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The inverse hyperbolic sine of the number.
pub fn asinh(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].asinh())
}

/// Calculates the inverse hyperbolic cosine of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The inverse hyperbolic cosine of the number.
pub fn acosh(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].acosh())
}

/// Calculates the inverse hyperbolic tangent of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The inverse hyperbolic tangent of the number.
pub fn atanh(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].atanh())
}

/// Calculates the natural logarithm of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The natural logarithm of the number.
pub fn ln(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].ln())
}

/// Calculates the base 10 logarithm of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The base 10 logarithm of the number.
pub fn log10(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].log10())
}

/// Calculates the base 2 logarithm of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The base 2 logarithm of the number.
pub fn log2(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].log2())
}

/// Calculates the logarithm of a number with a specified base.
///
/// # Arguments
///
/// * `args` - A vector containing two numbers: the number and the base.
///
/// # Returns
///
/// The logarithm of the number with the specified base.
pub fn log(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].log(args[1]))
}

/// Calculates the factorial of a number.
///
/// # Arguments
///
/// * `args` - A vector containing a single number.
///
/// # Returns
///
/// The factorial of the number.
pub fn factorial(args: Vec<f64>) -> Result<f64> {
    let mut result = 1.0;
    for i in 1..(args[0] as u64 + 1) {
        result *= i as f64;
    }
    Ok(result)
}

/// Calculates the power of a number.
///
/// # Arguments
///
/// * `args` - A vector containing two numbers: the base and the exponent.
///
/// # Returns
///
/// The base raised to the power of the exponent.
pub fn pow(args: Vec<f64>) -> Result<f64> {
    Ok(args[0].powf(args[1]))
}

/// Calculates the maximum value among a list of numbers.
///
/// # Arguments
///
/// * `args` - A vector containing multiple numbers.
///
/// # Returns
///
/// The maximum value among the numbers.
pub fn max(args: Vec<f64>) -> Result<f64> {
    Ok(args.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)))
}

/// Calculates the minimum value among a list of numbers.
///
/// # Arguments
///
/// * `args` - A vector containing multiple numbers.
///
/// # Returns
///
/// The minimum value among the numbers.
pub fn min(args: Vec<f64>) -> Result<f64> {
    Ok(args.iter().fold(f64::INFINITY, |a, &b| a.min(b)))
}

/// Calculates the sum of a list of numbers.
///
/// # Arguments
///
/// * `args` - A vector containing multiple numbers.
///
/// # Returns
///
/// The sum of the numbers.
pub fn sum(args: Vec<f64>) -> Result<f64> {
    Ok(args.iter().sum())
}

/// Calculates the average of a list of numbers.
///
/// # Arguments
///
/// * `args` - A vector containing multiple numbers.
///
/// # Returns
///
/// The average of the numbers.
pub fn avg(args: Vec<f64>) -> Result<f64> {
    Ok(args.iter().sum::<f64>() / args.len() as f64)
}

/// Calculates the median of a list of numbers.
///
/// # Arguments
///
/// * `args` - A vector containing multiple numbers.
///
/// # Returns
///
/// The median of the numbers.
pub fn median(args: Vec<f64>) -> Result<f64> {
    let mut args = args;
    args.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = args.len() / 2;
    if args.len() % 2 == 0 {
        Ok((args[mid - 1] + args[mid]) / 2.0)
    } else {
        Ok(args[mid])
    }
}
