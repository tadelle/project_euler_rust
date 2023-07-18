//! Distinct powers
//!
//! Consider all integer combinations of ab
//! for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:
//!
//! 2^2=4, 2^3=8, 2^4=16, 2^5=32
//! 3^2=9, 3^3=27, 3^4=81, 3^5=243
//! 4^2=16, 4^3=64, 4^4=256, 4^5=1024
//! 5^2=25, 5^3=125, 5^4=625, 5^5=3125
//! If they are then placed in numerical order,
//! with any repeats removed, we get the following
//! sequence of 15 distinct terms:
//!
//! 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125
//!
//! How many distinct terms are in the sequence
//! generated by a^b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?
use std::collections::HashMap;

use super::{add_vec, Problem};

crate::base_problem!(9183, "Distinct powers");

fn get_result_problem() -> i64 {
    let mut map: HashMap<String, i32> = HashMap::new();

    for number in 2..101 {
        power_vect(number, 100, &mut map);
    }

    map.len() as i64
}

fn power_vect(number: i32, power: i32, map: &mut HashMap<String, i32>) {
    let mut vec_num: Vec<u8> = number
        .to_string()
        .chars()
        .map(|b| b as u8 - b'0')
        .collect();
    vec_num.reverse();

    for _ in 1..power {
        let vec = vec_num.clone();
        for _ in 1..number {
            vec_num = add_vec(&vec, &vec_num)
        }

        let result = vec_num
            .clone()
            .into_iter()
            .map(|n| n.to_string())
            .reduce(|ac, d| format!("{}{}", d, ac))
            .unwrap_or("".to_string());

        map.insert(result.clone(), 0);
    }
}
