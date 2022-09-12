//! Circular primes
//!
//! The number, 197, is called a circular prime because 
//! all rotations of the digits: 197, 971, and 719, 
//! are themselves prime.
//! 
//! There are thirteen such primes below 100: 
//! 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
//! 
//! How many circular primes are there below one million?
use std::collections::HashMap;

use crate::problems::get_primes_eratostenes;

use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Circular primes")
    }

    fn get_result(&self) -> i64 {
        get_circular_primes() as i64
    }
}

fn get_circular_primes() -> i32 {
    let limit = 1_000_000;
    let map_primes =
        get_primes_eratostenes(limit)
        .into_iter()
        .map(|n| (n, true))
        .collect::<HashMap<i32, bool>>();
    let mut map_circular_primes: HashMap<i32, bool> = HashMap::new();

    for prime in map_primes.keys() {
        if map_circular_primes.contains_key(prime) {
            continue;
        }
        let vec = get_rotations(*prime);
        let mut is_all_primes = true;
        for n in &vec {
            if !map_primes.contains_key(&n){
                is_all_primes = false;
                break;
            }
        }
        if is_all_primes {
            for p in vec {
                map_circular_primes.insert(p, true);
            }
        }
    }
    map_circular_primes.len() as i32
}

fn get_rotations(number: i32) -> Vec<i32> {
    let mut vec = Vec::new();
    let str_number = number.to_string();
    vec.push(number);

    for i in 1..str_number.len() {
        vec.push(
            format!("{}{}", str_number[i..].to_string(), str_number[..i].to_string())
                .parse::<i32>().unwrap_or(0)
        );
    }

    vec
}

#[cfg(test)]
mod test035 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 55)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Circular primes")
    }
}
