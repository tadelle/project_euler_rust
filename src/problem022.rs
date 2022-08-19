//! Names scores
//!
//! Using names.txt, a 46K text file containing over
//! five-thousand first names, begin by sorting it into
//! alphabetical order. Then working out the alphabetical
//! value for each name, multiply this value by its
//! alphabetical position in the list to obtain a name score.
//!
//! For example, when the list is sorted into alphabetical
//! order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53,
//! is the 938th name in the list. So,
//! COLIN would obtain a score of 938 × 53 = 49714.
//!
//! What is the total of all the name scores in the file?
use std::fs;

use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Names scores")
    }

    fn get_result(&self) -> i64 {
        get_names_scores() as i64
    }
}

fn get_names_scores() -> i32 {
    let mut names = fs::read_to_string("src/data/p022_names.txt")
        .unwrap_or(String::from(""))
        .split(",")
        .map(|n| n.replace("\"", "").to_lowercase())
        .collect::<Vec<String>>();

    names.sort();

    let mut result = 0;
    for (index, name) in names.into_iter().enumerate() {
        let mut sum = 0;
        for digit in name.chars() {
            sum += ((digit as u8 - 'a' as u8) + 1) as i32;
        }
        result += sum * (index as i32 + 1);
    }

    result
}

#[cfg(test)]
mod test022 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 871198282)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Names scores")
    }
}
