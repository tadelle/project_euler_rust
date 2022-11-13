//! Goldbach's other conjecture
//!
//! It was proposed by Christian Goldbach that every odd composite
//! number can be written as the sum of a prime and twice a square.
//!
//! 9 = 7 + 2×1^2
//! 15 = 7 + 2×2^2
//! 21 = 3 + 2×3^2
//! 25 = 7 + 2×3^2
//! 27 = 19 + 2×2^2
//! 33 = 31 + 2×1^2
//!
//! It turns out that the conjecture was false.
//!
//! What is the smallest odd composite that cannot be
//! written as the sum of a prime and twice a square?
use super::{get_primes_eratostenes, is_prime, Problem};

crate::base_problem!(5777, "Goldbach's other conjecture");

fn get_result_problem() -> i64 {
    let mut odd = 33;
    let mut primes = get_primes_eratostenes(odd);

    loop {
        odd += 2;

        if is_prime(odd) {
            primes.push(odd);
        } else {
            let mut is_goldbach = false;
            for prime in &primes {
                let sqr = (odd - prime) / 2;
                if sqr <= 0 {
                    break;
                }
                let sqrt = (sqr as f64).sqrt() as i32;
                if sqrt * sqrt == sqr {
                    is_goldbach = true;
                    break;
                }
            }
            if !is_goldbach {
                return odd as i64;
            }
        }
    }
}
