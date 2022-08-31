//! Reciprocal cycles
//! 
//! A unit fraction contains 1 in the numerator. 
//! The decimal representation of the unit fractions 
//! with denominators 2 to 10 are given:
//! 
//! 1/2	= 0.5
//! 1/3 = 0.(3)
//! 1/4 = 0.25
//! 1/5 = 0.2
//! 1/6 = 0.1(6)
//! 1/7 = 0.(142857)
//! 1/8 = 0.125
//! 1/9 = 0.(1)
//! 1/10= 0.1
//!
//! Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. 
//! It can be seen that 1/7 has a 6-digit recurring cycle.
//! 
//! Find the value of d < 1000 for which 1/d contains the longest 
//! recurring cycle in its decimal fraction part.
use std::collections::HashMap;

use super::problems::Problem;

pub struct Problema {}

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Reciprocal Cycles")
    }

    fn get_result(&self) -> i64 {
        get_reciprocal_cycles() as i64
    }
}

fn get_reciprocal_cycles() -> i32 {
    let mut result = 0;

    for divisor in 2..1000 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut remainder = 10 % divisor;
        let mut times = 0;
        let mut num_cycles = 0;
        loop {
            if remainder == 0 {
                break;
            }
            times += 1;

            if map.contains_key(&remainder) {
                num_cycles = times - (map[&remainder] - 1);
                break;
            }
            map.insert(remainder, times);

            remainder = (remainder * 10) % divisor;
        }

        if num_cycles > result {
            result = num_cycles;
        }
    }

    result
}

#[cfg(test)]
mod test026 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 983)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Reciprocal Cycles")
    }
}
