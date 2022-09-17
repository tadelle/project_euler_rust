//! Digit factorials
//!
//! 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
//!
//! Find the sum of all numbers which are equal
//! to the sum of the factorial of their digits.
//!
//! Note: As 1! = 1 and 2! = 2 are not sums they are not included.
use crate::problems::get_sum_function_parallel;

use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Digit factorials")
    }

    fn get_result(&self) -> i64 {
        get_digit_factorials() as i64
    }
}

fn get_digit_factorials() -> i32 {
    let limit = 2_540_160; // (9! * 7)
    let init = 3;

    get_sum_function_parallel(get_digit_factorials_partial, init, limit)
}

fn get_factorial(number: i32) -> i32 {
    if number == 1 || number == 0 {
        1
    } else {
        number * get_factorial(number - 1)
    }
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

#[cfg(test)]
mod test034 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 40730)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Digit factorials")
    }

    #[test]
    fn test_get_factorial() {
        assert_eq!(get_factorial(0), 1);
        assert_eq!(get_factorial(1), 1);
        assert_eq!(get_factorial(2), 2);
        assert_eq!(get_factorial(3), 6);
        assert_eq!(get_factorial(4), 24);
        assert_eq!(get_factorial(5), 120);
        assert_eq!(get_factorial(6), 720);
        assert_eq!(get_factorial(7), 5040);
        assert_eq!(get_factorial(8), 40320);
        assert_eq!(get_factorial(9), 362880);
    }
}
