//! Double-base palindromes
//!
//! The decimal number, 585 = 1001001001
//! (binary), is palindromic in both bases.
//!
//! Find the sum of all numbers, less than
//! one million, which are palindromic in base 10 and base 2.
//!
//! (Please note that the palindromic number,
//! in either base, may not include leading zeros.)
use super::{get_sum_function_parallel, Problem};

crate::base_problem!(872187, "Double-base palindromes");

fn get_result_problem() -> i64 {
    get_sum_function_parallel(get_sum_double_base_palindromes, 1, 1_000_000 - 1) as i64
}

fn get_sum_double_base_palindromes(ini: usize, end: usize) -> i32 {
    (ini..end)
        .map(|number| {
            if is_double_base_palindrome(number) {
                number as i32
            } else {
                0
            }
        })
        .sum()
}

fn is_double_base_palindrome(number: usize) -> bool {
    let base10 = number.to_string();
    let base2 = format!("{:b}", number);

    base10 == base10.chars().rev().collect::<String>()
        && base2 == base2.chars().rev().collect::<String>()
}
