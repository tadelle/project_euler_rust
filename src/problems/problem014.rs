//! Longest Collatz sequence
//!
//! The following iterative sequence is defined for the set of positive integers:
//!
//! n → n/2 (n is even)
//! n → 3n + 1 (n is odd)
//!
//! Using the rule above and starting with 13, we generate the following sequence:
//!
//! 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
//! It can be seen that this sequence (starting at 13 and finishing at 1)
//! contains 10 terms. Although it has not been proved yet (Collatz Problem),
//! it is thought that all starting numbers finish at 1.
//!
//! Which starting number, under one million, produces the longest chain?
//!
//! NOTE: Once the chain starts the terms are allowed to go above one million.
use super::Problem;

crate::base_problem!(837799, "Longest Collatz sequence");

fn get_result_problem() -> i64 {
    let mut result = 0;
    let mut length = 0;

    for number in 1..1_000_000 {
        let seq_len = get_collatz_sequence_len(number);
        if seq_len > length {
            length = seq_len;
            result = number;
        }
    }
    result as i64
}

fn get_collatz_sequence_len(number: i32) -> i32 {
    let mut length: i32 = 0;

    let mut term: i64 = number as i64;
    while term > 1 {
        term = match term % 2 {
            0 => term / 2,
            _ => 3 * term + 1,
        };
        length += 1;
    }

    length
}
