//! Largest prime factor
//! 
//! The prime factors of 13195 are 5, 7, 13 and 29.
//! 
//! What is the largest prime factor of the number 600851475143 ?
use super::problems::Problem;

pub struct Problema;

impl Problem for Problema{

    fn new() -> Problema { Problema { }}

    fn get_title(&self) -> String {
        String::from("Largest prime factor")
    }

    fn get_result(&self) -> i64 {
        get_largest_prime_factor()
    }
}

fn get_largest_prime_factor() -> i64 {
    let mut number: i64 = 600851475143;
    let mut divisor = 1;

    while number > 1 {
        divisor += 2;
        while number % divisor == 0{
            number /= divisor;
        }
    }
    divisor
}

#[cfg(test)]
mod test003 {
    use super::*;
    
    #[test]
    fn test_get_result(){
        assert_eq!(Problema::new().get_result(), 6857)
    }

    #[test]
    fn test_get_title(){
        assert_eq!(Problema::new().get_title(), "Largest prime factor")
    }
}