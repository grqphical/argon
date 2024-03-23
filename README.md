# Argon Calculator

A superpowered CLI calculator

## LICENSE

MIT License

Copyright (c) 2024 grqphical

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## TABLE OF CONTENTS

1. Basic Usage
2. Variables
3. Built-in Functions

## BASIC USAGE

Enter any mathematical statement in the REPL and Argon will calculate it

### OPERATORS

- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `^` Power (Integer)
- `%` Modulus

Argon follows the BEDMAS (Brackets, Exponents, Division, Mulitplication, Addition, Subtraction)
Order of Operations and whenever there is two terms in the same order (eg. Two multiplication statements),
it goes from left to right

## VARIABLES

In Argon, you can declare variables with this syntax:
NAME = VALUE
The only type allowed is numbers so value has to be a real number

You can use variables in equations by just typing it's identifier. For example, the variable X can be referenced by using X
in an equation.

There are some built in constants for common mathematical constants.
PI - 3.14159265358979
E - 2.718281828459045
TAU - 6.283185307179586

# BUILT IN FUNCTIONS

There are many built in functions within Argon such as trignometric functions and logarithimic functions.

- `sqrt(number)`: Requires one number. Returns the square root of the number.
- `rad2deg(angle)`: Requires one angle in radians. Returns the angle in degrees.
- `deg2rad(angle)`: Requires one angle in degrees. Returns the angle in radians.
- `abs(number)`: Requires one number. Returns the absolute value of the number.
- `ceil(number)`: Requires one number. Returns the smallest integer greater than or equal to the number.
- `floor(number)`: Requires one number. Returns the largest integer less than or equal to the number.
- `round(number)`: Requires one number. Returns the number rounded to the nearest integer.
- `recip(number)`: Requires one number. Returns the reciprocal of the number.
- `sin(angle)`: Requires one angle in radians. Returns the sine of the angle.
- `cos(angle)`: Requires one angle in radians. Returns the cosine of the angle.
- `tan(angle)`: Requires one angle in radians. Returns the tangent of the angle.
- `asin(number)`: Requires one number. Returns the arcsine of the number.
- `acos(number)`: Requires one number. Returns the arccosine of the number.
- `atan(number)`: Requires one number. Returns the arctangent of the number.
- `sinh(number)`: Requires one number. Returns the hyperbolic sine of the number.
- `cosh(number)`: Requires one number. Returns the hyperbolic cosine of the number.
- `tanh(number)`: Requires one number. Returns the hyperbolic tangent of the number.
- `asinh(number)`: Requires one number. Returns the inverse hyperbolic sine of the number.
- `acosh(number)`: Requires one number. Returns the inverse hyperbolic cosine of the number.
- `atanh(number)`: Requires one number. Returns the inverse hyperbolic tangent of the number.
- `ln(number)`: Requires one number. Returns the natural logarithm of the number.
- `log10(number)`: Requires one number. Returns the base-10 logarithm of the number.
- `pow(base, exponent)`: Requires two numbers. Returns the base raised to the power of the exponent.
- `log(base, number)`: Requires two numbers. Returns the logarithm of the number to the specified base.
- `max(numbers)`: Requires a list of numbers. Returns the largest number in the list.
- `min(numbers)`: Requires a list of numbers. Returns the smallest number in the list.
- `sum(numbers)`: Requires a list of numbers. Returns the sum of all numbers in the list.
- `avg(numbers)`: Requires a list of numbers. Returns the average of all numbers in the list.
- `median(numbers)`: Requires a list of numbers. Returns the median of the numbers in the list.
- `factorial(number)`: Requires one number. Returns the factorial of the given number.
