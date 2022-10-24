//! Pandigital multiples
//!
//! Take the number 192 and multiply it by each of 1, 2, and 3:
//!
//! 192 × 1 = 192
//! 192 × 2 = 384
//! 192 × 3 = 576
//! By concatenating each product we get the 1 to 9 pandigital,
//! 192384576. We will call 192384576 the concatenated
//! product of 192 and (1,2,3)
//!
//! The same can be achieved by starting with 9 and multiplying
//! by 1, 2, 3, 4, and 5, giving the pandigital, 918273645,
//! which is the concatenated product of 9 and (1,2,3,4,5).
//!
//! What is the largest 1 to 9 pandigital 9-digit number that
//! can be formed as the concatenated product of an integer
//! with (1,2, ... , n) where n > 1?
use super::Problem;

crate::base_problem!(932718654, "Pandigital multiples");

fn get_result_problem() -> i64 {
    let base_number = "123456789".to_string();
    let mut vec_numbers: Vec<String> = Vec::new();

    for n in 1..10_000 {
        let mut count = 1;
        let mut product = String::from("");
        while product.len() < 9 {
            product = format!("{}{}", product, n * count);
            count += 1;
        }
        let mut vec_prod = product.chars().collect::<Vec<_>>();
        vec_prod.sort();
        if vec_prod.into_iter().collect::<String>() == base_number {
            vec_numbers.push(product);
        }
    }

    let mut res_vec = vec_numbers
        .into_iter()
        .map(|n| n.parse::<i32>().unwrap_or(0))
        .collect::<Vec<_>>();
    res_vec.sort();

    *res_vec.last().unwrap_or(&0) as i64
}
