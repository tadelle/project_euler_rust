//! Smallest multiple
//!
//! 2520 is the smallest number that can be
//! divided by each of the numbers
//! from 1 to 10 without any remainder.
//!
//! What is the smallest positive number
//! that is evenly divisible by all
//! of the numbers from 1 to 20?
use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Smallest multiple")
    }

    fn get_result(&self) -> i64 {
        get_smallest_multiple() as i64
    }
}

fn get_smallest_multiple() -> i32 {
    let mut result = 1;
    let mut divisors = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let mut divisor = 2;
    let mut is_used = false;

    while divisor < 21 {
        for index in 0..20 {
            if divisors[index] % divisor == 0 {
                divisors[index] /= divisor;
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

    result
}

#[cfg(test)]
mod test005 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 232792560)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Smallest multiple")
    }
}
