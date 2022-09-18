//! Summation of primes
//!
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! Find the sum of all the primes below two million.
use super::{get_primes_eratostenes, Problem};

crate::base_problem!(142_913_828_922, "Summation of primes");

fn get_result_problem() -> i64 {
    get_primes_eratostenes(2_000_000)
        .into_iter()
        .map(|prime| prime as i64)
        .sum::<_>()
}
