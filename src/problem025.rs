//! 1000-digit Fibonacci number
//! 
//! The Fibonacci sequence is defined by the recurrence relation:
//! 
//! Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
//! Hence the first 12 terms will be:
//! 
//! F1 = 1
//! F2 = 1
//! F3 = 2
//! F4 = 3
//! F5 = 5
//! F6 = 8
//! F7 = 13
//! F8 = 21
//! F9 = 34
//! F10 = 55
//! F11 = 89
//! F12 = 144
//! The 12th term, F12, is the first term to contain three digits.
//! 
//! What is the index of the first term in the
//! Fibonacci sequence to contain 1000 digits?
use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("1000-digit Fibonacci number")
    }

    fn get_result(&self) -> i64 {
        get_1000_digit_fibonacci_number() as i64
    }
}

fn get_1000_digit_fibonacci_number() -> i32 {
    
    let limit = 1000;
    let mut index = 2;
    let mut vec1 = vec![1_u8];
    let mut vec2 = vec![1_u8];
    loop {

        let vec_next = add_vec(&vec1, &vec2);
        index += 1;

        if vec_next.len() >= limit {
            break;
        }

        vec1 = vec2;
        vec2 = vec_next;
    }

    index
}

fn add_vec(vec1: &Vec<u8>, vec2: &Vec<u8>) -> Vec<u8> {
    let mut result: u8;
    let mut remainder: u8 = 0;
    let mut vec_res: Vec<u8> = Vec::new();

    let mut index = 0;
    for digit in vec1 {
        result = digit + vec2[index] + remainder;
        vec_res.push(result % 10);
        remainder = result / 10;
        index += 1;
    }
    while remainder > 0 || vec2.len() > index {

        if index < vec2.len() {
            result = vec2[index] + remainder;
            vec_res.push(result % 10);
            remainder = result / 10;
        } else {
            vec_res.push(remainder);
            remainder = 0;
        }
        index += 1;
    }

    vec_res
}

#[cfg(test)]
mod test025 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 4782)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "1000-digit Fibonacci number")
    }
}
