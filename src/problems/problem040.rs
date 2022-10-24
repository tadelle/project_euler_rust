//! Champernowne's constant
//!
//! An irrational decimal fraction is created
//! by concatenating the positive integers:
//!
//! 0.123456789101112131415161718192021...
//!              ^
//! It can be seen that the 12th digit of the fractional part is 1.
//!
//! If dn represents the nth digit of the fractional part,
//! find the value of the following expression.
//!
//! d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000
use super::Problem;

crate::base_problem!(210, "Champernowne's constant");

fn get_result_problem() -> i64 {
    let limit = 1_000_000;
    let indexes: [usize; 7] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];
    let mut number = String::with_capacity(limit);

    let mut counter = 1;
    while number.len() < limit {
        number.push_str(&counter.to_string());
        counter += 1;
    }

    let vec = number
        .chars()
        .map(|c| (c as u8 - '0' as u8) as i32)
        .collect::<Vec<_>>();

    let mut result = 1;
    for index in indexes {
        result *= vec[index - 1];
    }

    result as i64
}
