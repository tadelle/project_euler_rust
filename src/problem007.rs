//! 10001st prime
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//!
//! What is the 10 001st prime number?
//use super::{problems::Problem, problems::is_prime} ;
use super::{problems::is_prime, problems::Problem};

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("10001st prime")
    }

    fn get_result(&self) -> i64 {
        get_10001st_prime() as i64
    }
}

fn get_10001st_prime() -> i32 {
    let limit = 10_001;
    let mut counter = 2;
    let mut number = 3;

    while counter < limit {
        number += 2;
        if is_prime(number) {
            counter += 1;
        }
    }
    number
}

#[cfg(test)]
mod test007 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 104743)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "10001st prime")
    }
}
