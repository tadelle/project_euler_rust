//! Coded triangle numbers
//!
//! The nth term of the sequence of triangle numbers
//! is given by, tn = 1/2n(n+1); so the first ten
//! triangle numbers are:
//!
//! 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
//!
//! By converting each letter in a word to a number
//! corresponding to its alphabetical position and
//! adding these values we form a word value.
//! For example, the word value for SKY is 19 + 11 + 25 = 55 = t10.
//! If the word value is a triangle number then we
//! shall call the word a triangle word.
//!
//! Using words.txt (right click and 'Save Link/Target As...'),
//! a 16K text file containing nearly two-thousand common
//! English words, how many are triangle words?
use super::Problem;
use std::{collections::HashMap, fs};

crate::base_problem!(162, "Coded triangle numbers");

fn get_result_problem() -> i64 {
    let value_words: Vec<i32> = get_words()
        .into_iter()
        .map(|w| {
            w.chars()
                .map(|c| (c as u8 - (b'A' - 1_u8)) as i32)
                .sum::<i32>()
        })
        .collect();

    let max = value_words.clone().into_iter().max().unwrap_or(0);
    let mut map: HashMap<i32, bool> = HashMap::new();

    let mut number = 0;
    let mut index = 1;
    while number < max {
        number = get_triangle_number(index);
        map.insert(number, true);
        index += 1;
    }

    value_words
        .into_iter()
        .filter(|n| map.contains_key(n))
        .count() as i64
}

fn get_triangle_number(index: i32) -> i32 {
    ((1.0 / 2.0) * index as f64 * (index as f64 + 1.0)) as i32
}

fn get_words() -> Vec<String> {
    fs::read_to_string("src/data/p042_words.txt")
        .unwrap_or(String::from(""))
        .split(',')
        .map(|n| n.replace('\"', ""))
        .collect::<Vec<String>>()
}
