//! Prime digit replacements
//!
//! By replacing the 1st digit of the 2-digit number *3,
//! it turns out that six of the nine possible values:
//! 13, 23, 43, 53, 73, and 83, are all prime.
//!
//! By replacing the 3rd and 4th digits of 56**3 with the same digit,
//! this 5-digit number is the first example having seven primes
//! among the ten generated numbers, yielding the family:
//! 56003, 56113, 56333, 56443, 56663, 56773, and 56993. Consequently
//! 56003, being the first member of this family, is the smallest
//! prime with this property.
//!
//! Find the smallest prime which, by replacing part of the number
//! (not necessarily adjacent digits) with the same digit,
//! is part of an eight prime value family.
use super::problems::{Problem, is_prime};
use std::collections::HashMap;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Prime digit replacements")
    }

    fn get_result(&self) -> i64 {
        get_prime_digit_replacements() as i64
    }
}

fn get_prime_digit_replacements() -> i32 {
    let mut num_digits = 5;
    let mut ini_number = 56003;
    let mut end_number = 100_000;
    let mut primes: HashMap<i32, bool> = HashMap::new();

    loop {
        let patterns = get_pattern(num_digits);

        for number in (ini_number..end_number).step_by(2) {
            let vec_number = get_array_number(number);

            for num_replaced_digits in 1..num_digits {

                let list_patterns = patterns.clone()
                    .into_iter()
                    .filter(|vec| vec.into_iter().sum::<i32>() == num_replaced_digits as i32)
                    .collect::<Vec<Vec<i32>>>();

                for pattern in list_patterns {
                    let result = get_smallest_primes_of_eight(pattern.clone(), vec_number.clone(), &mut primes);
                    if result > 0 {
                        return result;
                    }
                }
            }
        }
        ini_number = end_number + 1;
        end_number *= 10;
        num_digits += 1;
    }
}

fn get_pattern(num_digits: usize) -> Vec<Vec<i32>> {
    let mut limit: i32 = 1;
    for _ in 0..num_digits {
        limit *= 2;
    }
    limit -= 1;

    let mut vec_bin: Vec<Vec<i32>> = Vec::new();
    for number in 1..limit {
        let str_bin = format!("{number:010b}")[(10 - num_digits)..].to_string();
        vec_bin.push(
            str_bin
            .chars()
            .into_iter()
            .map(|digit| (digit as i32) - ('0' as i32))
            .collect::<Vec<_>>()
        );
    }

    vec_bin.clone()
}

fn get_array_number(number: i32) -> Vec<i32>{
    number
        .to_string()
        .chars()
        .into_iter()
        .map(|digit| (digit as i32) - ('0' as i32))
        .collect::<Vec<_>>()
}

fn get_smallest_primes_of_eight(pattern: Vec<i32>, number: Vec<i32>, map: &mut HashMap<i32, bool>) -> i32 {
    let first_digit = pattern[0];
    let mut counter = 0;
    let mut smallest = 0;

    for digit in first_digit..10 {
        let mut new_number = 0;
        for index in 0..number.len() {
            new_number *= 10;
            new_number += if pattern[index] == 1 { digit } else { number[index] };
        }
        if map.contains_key(&new_number) || is_prime(new_number) {
            counter += 1;
            if smallest == 0 {
                smallest = new_number;
            }
            if !map.contains_key(&new_number) {
                map.insert(new_number, true);
            }
        }
        if digit > 2 && smallest == 0 {
            return 0;
        }
    }
    if counter >= 8 { smallest } else { 0 }
}

#[cfg(test)]
mod test051 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 121313)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Prime digit replacements")
    }
}
