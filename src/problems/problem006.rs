//! Sum square difference
//!
//! The sum of the squares of the first ten natural numbers is,
//!
//! 1^2 + 2^2 + ... + 10^2 = 385
//!
//! The square of the sum of the first ten natural numbers is,
//!
//! (1 + 2 + ... + 10)^2 = 3025
//!
//! Hence the difference between the sum of the squares of the
//! first ten natural numbers and the square of the sum is .
//!
//! Find the difference between the sum of the squares of the
//! first one hundred natural numbers and the square of the sum.
use super::Problem;

crate::base_problem!(25164150, "Sum square difference");

fn get_result_problem() -> i64 {
    let sum_of_squares = (1..101).map(|n| n * n).sum::<i32>();

    let sum = (1..101).sum::<i32>();

    ((sum * sum) - sum_of_squares) as i64
}
