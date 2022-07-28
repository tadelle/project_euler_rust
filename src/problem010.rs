//! Summation of primes
//!
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! Find the sum of all the primes below two million.
use super::problems::{get_primes_erastostenes, Problem};

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Summation of primes")
    }

    fn get_result(&self) -> i64 {
        get_summation_of_primes()
    }
}

fn get_summation_of_primes() -> i64 {
    get_primes_erastostenes(2_000_000)
        .into_iter()
        .map(|prime| prime as i64)
        .sum::<_>()
}

#[cfg(test)]
mod test010 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 142_913_828_922)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Summation of primes")
    }
}
