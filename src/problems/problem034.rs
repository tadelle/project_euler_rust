//! Digit factorials
//!
//! 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
//!
//! Find the sum of all numbers which are equal
//! to the sum of the factorial of their digits.
//!
//! Note: As 1! = 1 and 2! = 2 are not sums they are not included.
use super::{get_factorial, get_sum_function_parallel, Problem};

crate::base_problem!(40730, "Digit factorials");

fn get_result_problem() -> i64 {
    let limit = 2_540_160; // (9! * 7)
    let init = 3;

    get_sum_function_parallel(get_digit_factorials_partial, init, limit) as i64
}

fn get_digit_factorials_partial(ini: usize, end: usize) -> i32 {
    let limit = end;
    let mut counter = ini;
    let mut result = 0;
    let vec_factorial = vec![
        1,
        1,
        get_factorial(2),
        get_factorial(3),
        get_factorial(4),
        get_factorial(5),
        get_factorial(6),
        get_factorial(7),
        get_factorial(8),
        get_factorial(9),
    ];

    while counter < limit {
        let sum = get_sum_factorial_digit(counter, &vec_factorial);

        if counter == sum {
            result += counter;
        }

        counter += 1;
    }
    result as i32
}

fn get_sum_factorial_digit(number: usize, vector: &Vec<i32>) -> usize {
    let mut number1 = number;
    let mut sum: usize = 0;
    while number1 > 0 {
        let digit = number1 % 10;
        sum += vector[digit] as usize;
        number1 /= 10;
    }
    sum
}
