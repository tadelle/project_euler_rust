//! Largest palindrome product
//!
//! A palindromic number reads the same both ways.
//! The largest palindrome made from the product
//! of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from
//! the product of two 3-digit numbers.
use super::problems::Problem;

pub struct Problema {}

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Largest palindrome product")
    }

    fn get_result(&self) -> i64 {
        get_largest_palindrome_product() as i64
    }
}

fn get_largest_palindrome_product() -> i32 {
    let mut result = 0;

    for factor1 in 100..1000 {
        for factor2 in factor1..1000 {
            let mult = factor1 * factor2;
            if mult > result && is_palindrome(mult) {
                result = mult;
            }
        }
    }

    result
}

fn is_palindrome(number: i32) -> bool {
    let str_number = number.to_string();
    let str_inv_number = str_number.chars().rev().collect::<String>();

    str_number == str_inv_number
}

#[cfg(test)]
mod test004 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 906609)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Largest palindrome product")
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(9009));
        assert!(is_palindrome(12321));
        assert!(is_palindrome(909));
        assert!(is_palindrome(11));
        assert!(!is_palindrome(123));
    }
}
