//! Smallest multiple
//!
//! 2520 is the smallest number that can be
//! divided by each of the numbers
//! from 1 to 10 without any remainder.
//!
//! What is the smallest positive number
//! that is evenly divisible by all
//! of the numbers from 1 to 20?
use super::Problem;

crate::base_problem!(232792560, "Smallest multiple");

fn get_result_problem() -> i64 {
    let mut result = 1;
    let mut divisors = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let mut divisor = 2;
    let mut is_used = false;

    while divisor < 21 {
        for d in divisors.iter_mut() {
            if *d % divisor == 0 {
                *d /= divisor;
                is_used = true;
            }
        }

        if is_used {
            result *= divisor;
            is_used = false;
        } else {
            divisor = if divisor == 2 { 3 } else { divisor + 2 };
        }
    }

    result as i64
}
