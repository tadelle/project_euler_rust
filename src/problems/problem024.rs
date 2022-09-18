//! Lexicographic permutations
//!
//! A permutation is an ordered arrangement of objects.
//! For example, 3124 is one possible permutation of the
//! digits 1, 2, 3 and 4. If all of the permutations
//! are listed numerically or alphabetically, we call it
//! lexicographic order.
//! The lexicographic permutations of 0, 1 and 2 are:
//!
//! 012   021   102   120   201   210
//!
//! What is the millionth lexicographic permutation
//! of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
use super::Problem;

crate::base_problem!(2783915460, "Lexicographic permutations");

fn get_result_problem() -> i64 {
    let mut vec = get_permutations(String::from(""), String::from("0123456789"));

    vec.sort();

    vec[999999].parse::<i64>().unwrap_or(0)
}

fn get_permutations(value: String, digits: String) -> Vec<String> {
    let mut vec_res: Vec<String> = Vec::new();

    for digit in digits.chars() {
        let new_value = format!("{value}{digit}");
        let new_digits = digits.replace(digit, "");
        if new_digits.len() == 0 {
            vec_res.push(new_value);
        } else {
            let vec_partial = get_permutations(new_value, new_digits);
            for v in vec_partial {
                vec_res.push(v);
            }
        }
    }

    vec_res.clone()
}
