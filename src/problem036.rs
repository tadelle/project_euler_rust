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
use crate::problems::get_sum_function_parallel;

use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Double-base palindromes")
    }

    fn get_result(&self) -> i64 {
        get_double_base_palindromes() as i64
    }
}

fn get_double_base_palindromes() -> i32 {
    get_sum_function_parallel(get_sum_double_base_palindromes, 1, 1_000_000 - 1)
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

#[cfg(test)]
mod test036 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 872187)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Double-base palindromes")
    }
}
