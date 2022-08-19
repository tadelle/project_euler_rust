//! Number letter counts
//!
//! If the numbers 1 to 5 are written out in words:
//! one, two, three, four, five, then there
//! are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
//!
//! If all the numbers from 1 to 1000 (one thousand)
//! inclusive were written out in words,
//! how many letters would be used?
//!
//!
//! NOTE: Do not count spaces or hyphens. For example,
//! 342 (three hundred and forty-two) contains 23 letters
//! and 115 (one hundred and fifteen) contains 20 letters.
//! The use of "and" when writing out numbers is in
//! compliance with British usage.
use super::problems::Problem;
use std::collections::HashMap;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Number letter counts")
    }

    fn get_result(&self) -> i64 {
        get_number_letter_counts() as i64
    }
}

fn get_number_letter_counts() -> i32 {
    let map = get_map_numbers();
    let mut sum = 0;

    for number in 1..=1000 {
        sum += map[&number].len();
    }

    sum as i32
}

fn get_map_numbers() -> HashMap<i32, String> {
    let mut map: HashMap<i32, String> = HashMap::new();

    map.insert(0, "".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    map.insert(4, "four".to_string());
    map.insert(5, "five".to_string());
    map.insert(6, "six".to_string());
    map.insert(7, "seven".to_string());
    map.insert(8, "eight".to_string());
    map.insert(9, "nine".to_string());
    map.insert(10, "ten".to_string());
    map.insert(11, "eleven".to_string());
    map.insert(12, "twelve".to_string());
    map.insert(13, "thirteen".to_string());
    map.insert(14, "fourteen".to_string());
    map.insert(15, "fifteen".to_string());
    map.insert(16, "sixteen".to_string());
    map.insert(17, "seventeen".to_string());
    map.insert(18, "eighteen".to_string());
    map.insert(19, "nineteen".to_string());
    map.insert(20, "twenty".to_string());
    map.insert(30, "thirty".to_string());
    map.insert(40, "forty".to_string());
    map.insert(50, "fifty".to_string());
    map.insert(60, "sixty".to_string());
    map.insert(70, "seventy".to_string());
    map.insert(80, "eighty".to_string());
    map.insert(90, "ninety".to_string());
    map.insert(1000, "onethousand".to_string());

    for dozens in (20..=90).step_by(10) {
        for unit in 1..10 {
            map.insert(dozens + unit, format!("{}{}", map[&dozens], map[&unit]));
        }
    }

    for hundreds in 1..10 {
        for counter in 0..100 {
            let and = if counter > 0 {
                "and".to_string()
            } else {
                "".to_string()
            };
            map.insert(
                hundreds * 100 + counter,
                format!("{}hundred{}{}", map[&hundreds], and, map[&counter]),
            );
        }
    }

    map.clone()
}

#[cfg(test)]
mod test017 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 21124)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Number letter counts")
    }
}
