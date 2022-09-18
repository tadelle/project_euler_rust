//! Multiples of 3 and 5
//!
//! If we list all the natural numbers below 10 that
//! are multiples of 3 or 5, we get 3, 5, 6 and 9.
//! The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.
use super::Problem;

crate::base_problem!(233168, "Multiples of 3 and 5");

fn get_result_problem() -> i64 {
    (1..1000)
        .filter(|number| number % 3 == 0 || number % 5 == 0)
        .sum::<i32>() as i64
}
