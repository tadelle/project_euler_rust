//! Distinct primes factors
//!
//! The first two consecutive numbers to have
//! two distinct prime factors are:
//!
//! 14 = 2 × 7
//! 15 = 3 × 5
//!
//! The first three consecutive numbers to have
//! three distinct prime factors are:
//!
//! 644 = 2^2 × 7 × 23
//! 645 = 3 × 5 × 43
//! 646 = 2 × 17 × 19.
//!
//! Find the first four consecutive integers to
//! have four distinct prime factors each.
//! What is the first of these numbers?
use std::collections::HashMap;

use super::Problem;

crate::base_problem!(134043, "Distinct primes factors");

fn get_result_problem() -> i64 {
    let mut number = 646;
    let mut map_factor: HashMap<i32, Vec<i32>> = HashMap::new();

    loop {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut count = 0;
        let mut is_ok = true;
        let mut next = 4;
        for n in (number..(number + 4)).rev() {
            let vec = match map_factor.get(&n) {
                Some(v) => v.clone(),
                None => get_prime_factors(n),
            };

            if !map_factor.contains_key(&n) {
                map_factor.insert(n, vec.clone());
            }

            if vec.len() != 4 {
                is_ok = false;
                break;
            } else {
                count += 4;
                for factor in vec {
                    map.insert(factor, n);
                }
                if map.len() != count {
                    is_ok = false;
                    break;
                }
            }
            next -= 1;
        }
        if is_ok {
            return number as i64;
        }
        number += next;
    }
}

fn get_prime_factors(number: i32) -> Vec<i32> {
    let mut denominator = number;
    let mut divisor = 2;
    let mut vec = Vec::new();

    while denominator > 1 {
        let mut factor = 1;
        while denominator % divisor == 0 {
            denominator /= divisor;
            factor *= divisor;
        }
        if factor != 1 {
            vec.push(factor);
        }

        divisor = if divisor == 2 { 3 } else { divisor + 2 };
    }

    vec
}
