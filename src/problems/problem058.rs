//! Spiral Primes
//!
//! Starting with 1 and spiralling anticlockwise in the
//! following way, a square spiral with side length 7 is formed.
//!
//! 37 36 35 34 33 32 31
//! 38 17 16 15 14 13 30
//! 39 18  5  4  3 12 29
//! 40 19  6  1  2 11 28
//! 41 20  7  8  9 10 27
//! 42 21 22 23 24 25 26
//! 43 44 45 46 47 48 49
//!
//! It is interesting to note that the odd squares lie along
//! the bottom right diagonal, but what is more interesting is
//! that 8 out of the 13 numbers lying along both diagonals
//! are prime; that is, a ratio of 8/13 ≈ 62%.
//!
//! If one complete new layer is wrapped around the spiral above,
//! a square spiral with side length 9 will be formed.
//! If this process is continued, what is the side length of the
//! square spiral for which the ratio of primes along both
//! diagonals first falls below 10%?
use super::{is_prime, Problem};

crate::base_problem!(26241, "Spiral Primes");

fn get_result_problem() -> i64 {
    let mut side_length = 3;
    let mut diagonal_numbers = 1;
    let mut primes = 0;
    loop {
        diagonal_numbers += 4;
        primes += get_amount_primes_diagonal(side_length);

        if (primes as f64) / (diagonal_numbers as f64) < 0.1 {
            break;
        }

        side_length += 2;
    }

    side_length as i64
}

fn get_amount_primes_diagonal(matrix_len: i32) -> i32 {
    (1..=3)
        .map(|i| {
            if is_prime(matrix_len * matrix_len - (matrix_len - 1) * i) {
                1
            } else {
                0
            }
        })
        .sum()
}
