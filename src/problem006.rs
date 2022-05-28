//! Sum square difference
//! 
//! The sum of the squares of the first ten natural numbers is,
//! 
//! 1^2 + 2^2 + ... + 10^2 = 385
//! 
//! The square of the sum of the first ten natural numbers is,
//! 
//! (1 + 2 + ... + 10)^2 = 3025
//! 
//! Hence the difference between the sum of the squares of the
//! first ten natural numbers and the square of the sum is .
//! 
//! Find the difference between the sum of the squares of the 
//! first one hundred natural numbers and the square of the sum.
use super::problems::Problem;

pub struct Problema;

impl Problem for Problema{

    fn new() -> Problema { Problema { }}

    fn get_title(&self) -> String {
        String::from("Sum square difference")
    }

    fn get_result(&self) -> i64 {
        get_sum_square_difference()
    }
}

fn get_sum_square_difference() -> i64 {
    
    let sum_of_squares = (1..101)
        .into_iter()
        .map(|n| n * n)
        .sum::<i64>();

    let sum = (1..101)
        .into_iter()
        .sum::<i64>();

    (sum * sum) - sum_of_squares
}

#[cfg(test)]
mod test006 {
    use super::*;
    
    #[test]
    fn test_get_result(){
        assert_eq!(Problema::new().get_result(), 25164150)
    }

    #[test]
    fn test_get_title(){
        assert_eq!(Problema::new().get_title(), "Sum square difference")
    }
}