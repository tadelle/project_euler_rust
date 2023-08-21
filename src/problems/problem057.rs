//! Square Root Convergents
//! https://projecteuler.net/problem=57
//!
//! It is possible to show that the square root of 
//! two can be expressed as an infinite continued fraction.
//! 
//! SQRT(2) = 1 + 1 / (2 + (1 / 2 + (1 / 2 + (...
//! 
//! By expanding this for the first four iterations, we get:
//! 
//! 3/2 = 1.5
//! 7/5 = 1.4
//! 17/12 = 1.41666...
//! 41/29 = 1.41379...
//! 
//! The next three expansions are
//! 99/70, 239/169, and 577/408, but the eighth expansion, 1393/985,
//! is the first example where the number of digits in the numerator 
//! exceeds the number of digits in the denominator.
//! 
//! In the first one-thousand expansions, how many fractions contain 
//! a numerator with more digits than the denominator?
use super::{add_vec, Problem};

crate::base_problem!(153, "Square Root Convergents");

// Há um padrão onde a cada seis números há uma repetição
// da soma anterior mais um incremento, ou no mínino mantendo
// o valor anterior. Nesse caso, lendo as seis últimas potências
// é suficiente para verificar a maior soma.

fn get_result_problem() -> i64 {
    // let mut numerator_previus: Vec<u8> = vec![b'1'];
    // let mut denominator_previus: Vec<u8> = vec![b'1'];
    // let mut numerator: Vec<u8> = vec![b'3'];
    // let mut denominator: Vec<u8> = vec![b'2'];

    let mut numerator_previus: Vec<u8> = vec![1];
    let mut denominator_previus: Vec<u8> = vec![1];
    let mut numerator: Vec<u8> = vec![3];
    let mut denominator: Vec<u8> = vec![2];

    let mut counter: i32 = 0;

    for _ in 2..=1000 {
        let num1 = get_next_expansion(&numerator_previus, &numerator);
        let den = get_next_expansion(&denominator_previus, &denominator);
        numerator_previus = numerator;
        denominator_previus = denominator;

        numerator =  num1;
        denominator = den;

        if numerator.len() > denominator.len() {
            counter += 1;
        }

    }

    counter as i64

}

fn get_next_expansion(previus: &Vec<u8>, actual: &Vec<u8>) -> Vec<u8> {

    let new_vec = add_vec(actual, actual);
       
    add_vec(&new_vec, previus)
}
