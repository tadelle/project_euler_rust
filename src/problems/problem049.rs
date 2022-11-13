//! Prime permutations
//! 
//! The arithmetic sequence, 1487, 4817, 8147, 
//! in which each of the terms increases by 3330, 
//! is unusual in two ways: (i) each of the 
//! three terms are prime, and, (ii) each of the 
//! 4-digit numbers are permutations of one another.
//! 
//! There are no arithmetic sequences made up of 
//! three 1-, 2-, or 3-digit primes, exhibiting 
//! this property, but there is one other 4-digit 
//! increasing sequence.
//! 
//! What 12-digit number do you form by 
//! concatenating the three terms in this sequence?
use super::{Problem, is_prime};

crate::base_problem!(296962999629, "Prime permutations");

fn get_result_problem() -> i64 {

    let mut number = 999;
    let increment = 3330;

    let mut result = 0;

    while number < 9999 {
        number += 2;

        if !is_prime(number) || number == 1487 {
            continue;
        }

        let number2 = number + increment;
        let number3 = number +  2 * increment;

        if is_prime(number2) &&
           is_prime(number3) {

            let digs1 = get_ordered_digits(number);
            let digs2 = get_ordered_digits(number2);
            let digs3 = get_ordered_digits(number3);

            if digs1 == digs2 && digs2 == digs3 {
                result = format!("{}{}{}", number, number2, number3).parse::<i64>().unwrap_or(0);
                break;
            }
        }
    }
    result
}

fn get_ordered_digits(number: i32) -> i32 {
    
    let mut vec: Vec<char> = number
        .to_string()
        .chars()
        .collect();
    vec.sort();

    vec
        .into_iter()
        .collect::<String>()
        .parse::<i32>()
        .unwrap_or(0)
}