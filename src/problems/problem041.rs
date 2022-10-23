//! Pandigital prime
//!
//! We shall say that an n-digit number is pandigital
//! if it makes use of all the digits 1 to n exactly once.
//! For example, 2143 is a 4-digit pandigital and is also prime.
//!
//! What is the largest n-digit pandigital prime that exists?
use super::{is_prime, Problem};

crate::base_problem!(7652413, "Pandigital prime");

fn get_result_problem() -> i64 {
    let mut numbers = get_pandigital_numbers();

    numbers.sort();

    for number in numbers.into_iter().rev() {
        if is_prime(number) {
            return number as i64;
        }
    }
    0
}

fn get_pandigital_numbers() -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();

    vec.push(12);
    vec.push(21);

    for number in 3..9_i32 {
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
    vec
}
