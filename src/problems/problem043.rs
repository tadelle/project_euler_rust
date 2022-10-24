//! Sub-string divisibility
//!
//! The number, 1406357289, is a 0 to 9 pandigital
//! number because it is made up of each of the
//! digits 0 to 9 in some order, but it also has
//! a rather interesting sub-string divisibility property.
//!
//! Let d1 be the 1st digit, d2 be the 2nd digit,
//! and so on. In this way, we note the following:
//!
//! d2d3d4=406 is divisible by 2
//! d3d4d5=063 is divisible by 3
//! d4d5d6=635 is divisible by 5
//! d5d6d7=357 is divisible by 7
//! d6d7d8=572 is divisible by 11
//! d7d8d9=728 is divisible by 13
//! d8d9d10=289 is divisible by 17
//! Find the sum of all 0 to 9 pandigital numbers with this property.
use super::Problem;

crate::base_problem!(16695334890, "Sub-string divisibility");

fn get_result_problem() -> i64 {
    get_ten_pandigital_numbers()
        .into_iter()
        .filter(|n| *n >= 1023456789_i64 && is_interesting_div_property(*n))
        .sum::<i64>()
}

fn get_ten_pandigital_numbers() -> Vec<i64> {
    let mut vec: Vec<i32> = Vec::new();

    vec.push(12);
    vec.push(21);

    for number in 3..=9_i32 {
        let ini = 10_i32.pow((number - 2) as u32);
        let end = 10_i32.pow((number - 1) as u32);

        for pan_number in vec.clone().into_iter().filter(|n| *n > ini && *n < end) {
            vec.push(pan_number * 10 + number);
            vec.push(pan_number + number * end);

            for pos in 1..(number - 1) {
                let mult10 = 10_i32.pow(pos as u32);
                let n1 =
                    (pan_number / mult10) * mult10 * 10 + number * mult10 + (pan_number % mult10);
                vec.push(n1);
            }
        }
    }

    let mut vec10: Vec<i64> = Vec::new();
    let vec_new: Vec<i64> = vec
        .into_iter()
        .filter(|n| *n > 100_000_000)
        .map(|n| n as i64)
        .collect();

    for pan_number in vec_new {
        for pos in 1..9 {
            let mult10 = 10_i64.pow(pos as u32);
            let n1 = (pan_number / mult10) * mult10 * 10 + (pan_number % mult10);
            vec10.push(n1);
        }
    }
    vec10
}

fn is_interesting_div_property(number: i64) -> bool {
    let primes = [2, 3, 5, 7, 11, 13, 17];
    let mut div = 1000000;

    for prime in primes {
        if ((number / div) % 1000) % prime != 0 {
            return false;
        }
        div /= 10;
    }
    true
}
