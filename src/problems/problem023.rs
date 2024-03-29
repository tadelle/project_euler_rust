//! Non-abundant sums
//!
//! A perfect number is a number for which the sum of its proper
//! divisors is exactly equal to the number. For example, the sum
//! of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28,
//! which means that 28 is a perfect number.
//!
//! A number n is called deficient if the sum of its proper
//! divisors is less than n and it is called abundant
//! if this sum exceeds n.
//!
//! As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16,
//! the smallest number that can be written as the sum of two
//! abundant numbers is 24. By mathematical analysis, it can be shown
//! that all integers greater than 28123 can be written as the sum
//! of two abundant numbers. However, this upper limit cannot be
//! reduced any further by analysis even though it is known that the
//! greatest number that cannot be expressed as the sum of two abundant
//! numbers is less than this limit.
//!
//! Find the sum of all the positive integers which
//! cannot be written as the sum of two abundant numbers.
use std::collections::HashMap;

use super::Problem;

crate::base_problem!(4179871, "Non-abundant sums");

fn get_result_problem() -> i64 {
    let limit = 28123 + 1;
    let mut result = 0;
    let map = get_abundant_numbers();

    for number in 1..limit {
        let mut is_ok = false;
        for k in map.keys() {
            if k >= &number {
                continue;
            }
            if map.contains_key(&(number - k)) {
                is_ok = true;
                break;
            }
        }
        if !is_ok {
            result += number;
        }
    }

    result as i64
}

fn get_abundant_numbers() -> HashMap<i32, bool> {
    let limit = 28123;

    (12..=limit)
        .filter(|n| get_sum_divisors(*n) > *n)
        .map(|n| (n, true))
        .collect::<HashMap<i32, bool>>()
}

fn get_sum_divisors(number: i32) -> i32 {
    get_proper_divisors(number).into_iter().sum()
}

fn get_proper_divisors(number: i32) -> Vec<i32> {
    let limit = (number as f64).sqrt() as i32;
    let sqrt = limit * limit;
    let mut divisors = vec![1];

    for divisor in 2..limit {
        if number % divisor == 0 {
            divisors.push(divisor);
            divisors.push(number / divisor);
        }
    }

    if sqrt == number {
        divisors.push(limit);
    } else if number % limit == 0 {
        divisors.push(limit);
        divisors.push(number / limit);
    }

    divisors.clone()
}
