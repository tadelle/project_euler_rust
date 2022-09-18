//! Largest prime factor
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143 ?
use super::Problem;

crate::base_problem!(6857, "Largest prime factor");

fn get_result_problem() -> i64 {
    let mut number: i64 = 600851475143;
    let mut divisor = 1;

    while number > 1 {
        divisor += 2;
        while number % divisor == 0 {
            number /= divisor;
        }
    }
    divisor
}
