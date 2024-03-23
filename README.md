# Argon Calculator

[![Rust CI](https://github.com/grqphical/argon/actions/workflows/Rust.yml/badge.svg)](https://github.com/grqphical/argon/actions/workflows/Rust.yml)

A superpowered CLI calculator

## Basic Usage

Enter any mathematical statement in the REPL and Argon will calculate it

### Operators

- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `^` Power (Integer)
- `%` Modulus

Argon follows the BEDMAS (Brackets, Exponents, Division, Mulitplication, Addition, Subtraction)
Order of Operations and whenever there is two terms in the same order (eg. Two multiplication statements),
it goes from left to right

## Variables

In Argon, you can declare variables with this syntax:
`NAME = VALUE`
The only type allowed is numbers so value has to be a real number

You can use variables in equations by just typing it's identifier. For example, the variable X can be referenced by using `X`
in an equation.

NOTE: You cannot declare a variable and run an equation in the same line. Anything after the declaration will be omitted from the interpreter.

There are some built in constants for common mathematical constants.
`PI`: 3.14159265358979
`E`: 2.718281828459045
`TAU`: 6.283185307179586

## Built-in functions

There are many built in functions within Argon such as trignometric functions and logarithimic functions.

- `sqrt(number)`: Returns the square root of the number.
- `rad2deg(angle)`: Requires one angle in radians. Returns the angle in degrees.
- `deg2rad(angle)`: Requires one angle in degrees. Returns the angle in radians.
- `abs(number)`: Returns the absolute value of the number.
- `ceil(number)`: Returns the smallest integer greater than or equal to the number.
- `floor(number)`: Returns the largest integer less than or equal to the number.
- `round(number)`: Returns the number rounded to the nearest integer.
- `recip(number)`: Returns the reciprocal of the number.
- `sin(angle)`: Requires one angle in radians. Returns the sine of the angle.
- `cos(angle)`: Requires one angle in radians. Returns the cosine of the angle.
- `tan(angle)`: Requires one angle in radians. Returns the tangent of the angle.
- `asin(number)`: Returns the arcsine of the number.
- `acos(number)`: Returns the arccosine of the number.
- `atan(number)`: Returns the arctangent of the number.
- `sinh(number)`: Returns the hyperbolic sine of the number.
- `cosh(number)`: Returns the hyperbolic cosine of the number.
- `tanh(number)`: Returns the hyperbolic tangent of the number.
- `asinh(number)`: Returns the inverse hyperbolic sine of the number.
- `acosh(number)`: Returns the inverse hyperbolic cosine of the number.
- `atanh(number)`: Returns the inverse hyperbolic tangent of the number.
- `ln(number)`: Returns the natural logarithm of the number.
- `log10(number)`: Returns the base-10 logarithm of the number.
- `pow(base, exponent)`: Returns the base raised to the power of the exponent.
- `log(base, number)`: Returns the logarithm of the number to the specified base.
- `max(numbers)`: Requires a list of numbers. Returns the largest number in the list.
- `min(numbers)`: Requires a list of numbers. Returns the smallest number in the list.
- `sum(numbers)`: Requires a list of numbers. Returns the sum of all numbers in the list.
- `avg(numbers)`: Requires a list of numbers. Returns the average of all numbers in the list.
- `median(numbers)`: Requires a list of numbers. Returns the median of the numbers in the list.
- `factorial(number)`: Returns the factorial of the given number.
- `root(number, root)`: Roots a number to the nth root given in the argument root
- `exp(power)`: Raises E to the given power

## File as input

If you have lots of equations or a complex math function in a file, you can load it by passing it as an argument and Argon will run the file and print the output to stdout

## Changelog

See [CHANGELOG.md](https://github.com/grqphical/argon/blob/main/CHANGELOG.md)
