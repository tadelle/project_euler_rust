//! Amicable numbers
//!
//! Let d(n) be defined as the sum of proper divisors
//! of n (numbers less than n which divide evenly into n).
//! If d(a) = b and d(b) = a, where a ≠ b, then a and b are
//! an amicable pair and each of a and b are called amicable numbers.
//!
//! For example, the proper divisors of 220 are
//! 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284.
//! The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
//!
//! Evaluate the sum of all the amicable numbers under 10000.
use std::collections::HashMap;

use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Amicable numbers")
    }

    fn get_result(&self) -> i64 {
        get_amicable_numbers() as i64
    }
}

fn get_amicable_numbers() -> i32 {
    let limit = 10_000;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;

    for number in 4..limit {
        if !map.contains_key(&number) {
            let sum1: i32 = get_proper_divisors(number).into_iter().sum();

            if sum1 >= limit || sum1 == number{
                continue;
            }

            let sum2: i32 = get_proper_divisors(sum1).into_iter().sum();

            map.insert(number, sum1);
            map.insert(sum1, sum2);
            if sum2 == number {
                
                result += number;
                result += sum1;
            }
        }
    }

    result
}

fn get_proper_divisors(number: i32) -> Vec<i32> {
    let limit = (number as f64).sqrt() as i32;
    let sqrt = limit * limit;
    let mut divisors = vec![1];

    for divisor in 2..limit {
        if number % divisor == 0 {
            divisors.push(divisor);
            divisors.push(number / divisor);
        }
    }

    if sqrt == number {
        divisors.push(limit);
    } else {
        if number % limit == 0 {
            divisors.push(limit);
            divisors.push(number / limit);
        }
    }

    divisors.clone()
}

#[cfg(test)]
mod test021 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 648)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Amicable numbers")
    }
}
