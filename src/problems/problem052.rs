//! Permuted multiples
//! 
//! It can be seen that the number, 125874,
//! and its double, 251748, contain exactly
//! the same digits, but in a different order.
//! 
//! Find the smallest positive integer, x,
//! such that 2x, 3x, 4x, 5x, and 6x,
//! contain the same digits.
use super::Problem;

crate::base_problem!(142857, "Permuted multiples");

fn get_result_problem() -> i64 {

    let mut number = 10;
    let mut div = 100;
    loop {

        number += 1;
        if number % div != number {
            div *= 10;
        }

        let mut numbers = [number; 7];
        let mut is_valid = true;
        for i in (2..=6).rev() {
            numbers[i - 1] = number * i as i32;
            if numbers[i - 1] >= div {
                number = div;
                is_valid = false;
                break;
            }
        }
        if !is_valid { 
            continue;
        }
        
        let mut vec_digits: Vec<i32> = get_digits(number);
        vec_digits.sort();

        for i in (2..=6).rev() {
            let mut vec: Vec<i32> = get_digits(numbers[i - 1]);
            vec.sort();

            if vec != vec_digits {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            return number as i64;
        }
    }
}

fn get_digits(number: i32) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut n = number;

    while n > 0 {
        vec.push(n % 10);
        n /= 10;
    }
    vec
}
