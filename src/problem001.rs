//! Multiples of 3 and 5
//!
//! If we list all the natural numbers below 10 that
//! are multiples of 3 or 5, we get 3, 5, 6 and 9.
//! The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.
use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Multiples of 3 and 5")
    }

    fn get_result(&self) -> i64 {
        get_multiples_of_3_and_5() as i64
    }
}

fn get_multiples_of_3_and_5() -> i32 {
    (1..1000)
        .into_iter()
        .filter(|number| number % 3 == 0 || number % 5 == 0)
        .sum::<i32>()
}

#[cfg(test)]
mod test001 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 233168)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Multiples of 3 and 5")
    }
}
