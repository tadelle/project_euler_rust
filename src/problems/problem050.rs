//! Consecutive prime sum
//! 
//! Problem 50
//! The prime 41, can be written as the
//! sum of six consecutive primes:
//! 
//! 41 = 2 + 3 + 5 + 7 + 11 + 13
//! This is the longest sum of consecutive
//! primes that adds to a prime below one-hundred.
//! 
//! The longest sum of consecutive primes below
//! one-thousand that adds to a prime,
//! contains 21 terms, and is equal to 953.
//! 
//! Which prime, below one-million, can be written
//! as the sum of the most consecutive primes?
use std::collections::HashMap;

use super::{Problem, get_primes_eratostenes};

crate::base_problem!(997651, "Consecutive prime sum");

fn get_result_problem() -> i64 {
    let limit = 1_000_000;
    let primes: HashMap<i32, i32> = 
        get_primes_eratostenes(limit)
        .into_iter()
        .map(|p| (p, 0))
        .collect();

    let mut vec_primes = get_primes_eratostenes(limit);
    vec_primes.sort();

    let mut bigger = 0;
    let mut result = 0;
    let length = vec_primes.len();
    for index in 0..length {
        let prime = vec_primes[index];
        let mut sum = prime;
        let mut count = 1;

        for p in vec_primes.iter().take(length).skip(index + 1) {
            sum += p;
            if sum > limit {
                break;
            }
            count += 1;
            if primes.contains_key(&sum) && count > bigger {
                bigger = count;
                result = sum;
            }
        }
    }
    result as i64
}
