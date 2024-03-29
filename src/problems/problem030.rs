//! Digit fifth powers
//!
//! Surprisingly there are only three numbers that
//! can be written as the sum of fourth powers of their digits:
//!
//! 1634 = 1^4 + 6^4 + 3^4 + 4^4
//! 8208 = 8^4 + 2^4 + 0^4 + 8^4
//! 9474 = 9^4 + 4^4 + 7^4 + 4^4
//! As 1 = 1^4 is not a sum it is not included.
//!
//! The sum of these numbers is 1634 + 8208 + 9474 = 19316.
//!
//! Find the sum of all the numbers that can be
//! written as the sum of fifth powers of their digits.
use std::thread::{self, JoinHandle};

use super::Problem;

crate::base_problem!(443839, "Digit fifth powers");

fn get_result_problem() -> i64 {
    let initial = 2;
    let limit = 354295; // (9^5 * 6 + 1)
    let cores = num_cpus::get() as i32 * 2;
    let chunck = limit / cores;
    let mut vec_handle: Vec<JoinHandle<i32>> = Vec::new();

    let mut counter = 0;
    while counter < cores {
        let ini = if counter == 0 {
            initial
        } else {
            chunck * counter
        };
        let end = if counter == cores - 1 {
            limit
        } else {
            chunck * (counter + 1)
        };

        vec_handle.push(thread::spawn(move || get_sum_digit_fifth_powers(ini, end)));

        counter += 1;
    }

    let mut result = 0;
    for handle in vec_handle {
        result += handle.join().unwrap_or(0);
    }

    result as i64
}

fn get_sum_digit_fifth_powers(ini: i32, end: i32) -> i32 {
    let mut sum = 0;
    for n in ini..=end {
        let mut number = n;
        let mut remainder = number % 10;
        let mut partial_sum = 0;
        while number > 0 {
            partial_sum += remainder.pow(5);
            number /= 10;
            remainder = number % 10;
        }
        if partial_sum == n {
            sum += partial_sum;
        }
    }
    sum
}

// Mesma solução com iterators
#[allow(dead_code)]
fn get_result_problem_iter() -> i64 {
    (2..354295)
        .map(|n| {
            (
                n,
                n.to_string()
                    .chars()
                    .map(|c| ((c as u8 - b'0') as i32).pow(5))
                    .reduce(|ac, v| ac + v)
                    .unwrap_or(0),
            )
        })
        .filter(|(x, y)| x == y)
        .map(|(x, _)| x)
        .sum::<i32>() as i64
}
