//! Quadratic primes
//!
//! Euler discovered the remarkable quadratic formula:
//! n^2 + n + 41
//!
//! It turns out that the formula will produce 40 primes for the
//! consecutive integer values 0 <= n <= 39. However,
//! when n = 40, 40^2 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41,
//! and certainly when n = 41, 41^2 + 41 + 41 is clearly divisible by 41.
//!
//! The incredible formula n^2 - 79n + 1601 was discovered,
//! which produces 80 primes for the consecutive values 0 <= n <= 79.
//! The product of the coefficients, −79 and 1601, is −126479.
//!
//! Considering quadratics of the form:
//!
//! n^2 + an + b, where |a| < 1000 and |b| <= 1000
//!
//! where |n| is the modulus/absolute value of n
//! e.g. |11| = 11 and |-4| = 4
//!
//! Find the product of the coefficients, a and b, for the quadratic
//! expression that produces the maximum number of primes for
//! consecutive values of n, starting with n = 0.
use super::{is_prime, Problem};

crate::base_problem!(-59231, "Quadratic primes");

fn get_result_problem() -> i64 {
    let mut result = 0;
    let mut num_primes = 0;

    for a in -999..1000 {
        for b in -1000..1001 {
            let mut n = 0;

            while is_prime(n * n + a * n + b) {
                n += 1;
            }

            if n > num_primes {
                num_primes = n;
                result = a * b;
            }
        }
    }
    result as i64
}
