//! 10001st prime
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//!
//! What is the 10 001st prime number?
use super::{problems::get_primes_erastostenes, problems::is_prime, problems::Problem};

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

    let vec_primes = get_primes_erastostenes(limit * 10);

    if vec_primes.len() >= limit as usize {
        vec_primes[(limit - 1) as usize]
    } else {
        get_primes(
            limit,
            vec_primes.len() as i32,
            vec_primes[vec_primes.len() - 1],
        )
    }
}

fn get_primes(prime_position: i32, prime_position_ini: i32, prime_ini: i32) -> i32 {
    let limit = prime_position;
    let mut counter = prime_position_ini;
    let mut number = prime_ini;

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
