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
use super::{add_vec, Problem};

crate::base_problem!(4782, "1000-digit Fibonacci number");

fn get_result_problem() -> i64 {
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

    index as i64
}
