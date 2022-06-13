//! Summation of primes
//!
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! Find the sum of all the primes below two million.
use super::problems::{Problem, is_prime};
use num_cpus;
use std::thread::{self, JoinHandle};

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

fn get_parcial_sum(ini: i32, end: i32) -> i64 {
    (ini..end)
        .filter(|num| is_prime(*num))
        .map(|num| num as i64)
        .sum::<i64>()
}

fn get_summation_of_primes() -> i64 {
    let limit: i32 = 2_000_000;
    let cores = num_cpus::get() as i32 * 4;
    let chunck = limit / cores;
    let mut vec_handle: Vec<JoinHandle<i64>> = Vec::new();

    let mut counter = 0;
    while counter < cores {
        let ini: i32 = if counter == 0 { 2 } else { chunck * counter };
        let end: i32 = if counter == cores - 1 {
            limit
        } else {
            chunck * (counter + 1)
        };

        vec_handle.push(thread::spawn(move || {
            get_parcial_sum(ini, end)
        }));

        counter += 1;
    }

    let mut result = 0;
    for handle in vec_handle {
        result += handle.join().unwrap_or(0);
    }

    result
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
