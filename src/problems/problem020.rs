//! Factorial digit sum
//!
//! n! means n × (n − 1) × ... × 3 × 2 × 1
//!
//! For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
//! and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//!
//! Find the sum of the digits in the number 100!
use super::Problem;

crate::base_problem!(648, "Factorial digit sum");

fn get_result_problem() -> i64 {
    let mut vec_factorial = vec![0, 0, 1];
    let mut number: i32 = 99;

    while number > 1 {
        let vec_partial = vec_factorial.clone();
        let length = vec_partial.len();
        for _ in 1..number {
            let mut result = 0;
            let mut index = 0;

            while index < length {
                result = vec_factorial[index] + vec_partial[index] + result;
                vec_factorial[index] = result % 10;
                result /= 10;
                index += 1;
            }
            while result > 0 {
                if index < vec_factorial.len() {
                    result = vec_factorial[index] + result;
                    vec_factorial[index] = result % 10;
                    result /= 10;
                } else {
                    vec_factorial.push(result);
                    result = 0;
                }
                index += 1;
            }
        }
        number -= 1;
    }
    vec_factorial.into_iter().sum::<i32>() as i64
}
