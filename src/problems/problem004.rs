//! Largest palindrome product
//!
//! A palindromic number reads the same both ways.
//! The largest palindrome made from the product
//! of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from
//! the product of two 3-digit numbers.
use super::Problem;

crate::base_problem!(906609, "Largest palindrome product");

fn get_result_problem() -> i64 {
    let mut result = 0;

    for factor1 in 100..1000 {
        for factor2 in factor1..1000 {
            let mult = factor1 * factor2;
            if mult > result && is_palindrome(mult) {
                result = mult;
            }
        }
    }

    result as i64
}

fn is_palindrome(number: i32) -> bool {
    let str_number = number.to_string();
    str_number == str_number.chars().rev().collect::<String>()
}
