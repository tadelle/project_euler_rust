//! Power digit sum
//!
//! 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//!
//! What is the sum of the digits of the number 2^1000?
use super::Problem;

crate::base_problem!(1366, "Power digit sum");

fn get_result_problem() -> i64 {
    let mut vector: Vec<u8> = vec![2];

    for _ in 1..1000 {
        let mut rest = 0;
        for index in 0..vector.len() {
            let mult = vector[index] * 2 + rest;
            vector[index] = mult % 10;
            rest = mult / 10;
        }
        if rest > 0 {
            vector.push(rest);
        }
    }

    vector.into_iter().map(|num| num as i64).sum::<i64>()
}
