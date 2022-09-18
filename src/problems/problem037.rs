//! Truncatable primes
//!
//! The number 3797 has an interesting property.
//! Being prime itself, it is possible to continuously
//! remove digits from left to right, and remain prime
//! at each stage: 3797, 797, 97, and 7.
//! Similarly we can work from right to left: 3797, 379, 37, and 3.
//!
//! Find the sum of the only eleven primes that are
//! both truncatable from left to right and right to left.
//!
//! NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
use super::{get_primes_eratostenes, is_prime, Problem};

crate::base_problem!(748317, "Truncatable primes");

fn get_result_problem() -> i64 {
    let primes = get_primes_eratostenes(1_000_000);
    let mut result = 0;

    let mut count = 0;
    for p in primes.clone() {
        if is_truncable(p) {
            result += p;
            count += 1;
            if count >= 11 {
                break;
            }
        }
    }

    if count < 11 {
        let mut number = primes.last().unwrap() + 2;
        loop {
            if is_prime(number) && is_truncable(number) {
                result += number;
                count += 1;
                if count >= 11 {
                    break;
                }
            }
            number += 2
        }
    }

    result as i64
}

fn is_truncable(prime: i32) -> bool {
    if prime <= 7 {
        return false;
    }

    let mut number = prime / 10;
    while number > 0 {
        if !is_prime(number) {
            return false;
        }
        number /= 10;
    }

    let mut divisor = 10_i32.pow(prime.to_string().len() as u32);
    let mut number = prime % divisor;
    while number % divisor > 0 {
        number %= divisor;
        if !is_prime(number) {
            return false;
        }
        divisor /= 10;
    }

    true
}
