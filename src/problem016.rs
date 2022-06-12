//! Power digit sum
//!
//! 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//!
//! What is the sum of the digits of the number 2^1000?
use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Power digit sum")
    }

    fn get_result(&self) -> i64 {
        get_power_digit_sum() as i64
    }
}

fn get_power_digit_sum() -> i32 {
    let mut vector: Vec<u8> = vec![2];

    for _ in 1..1000 {
        let mut rest = 0;
        for index in 0..vector.len() {
            let mult = vector[index] * 2 + rest;
            vector[index] = mult % 10;
            rest = mult / 10;
        }
        if rest > 0 {
            vector.push(rest);
        }
    }

    vector.into_iter().map(|num| num as i32).sum::<i32>()
}

#[cfg(test)]
mod test016 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 1366)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Power digit sum")
    }
}
