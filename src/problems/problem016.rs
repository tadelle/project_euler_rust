//! Power digit sum
//!
//! 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//!
//! What is the sum of the digits of the number 2^1000?
use super::{power_vec, Problem};

crate::base_problem!(1366, "Power digit sum");

fn get_result_problem() -> i64 {
    power_vec(2, 1000)
        .into_iter()
        .map(|num| num as i64)
        .sum::<i64>()
}
