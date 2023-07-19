//! Pandigital products
//!
//! We shall say that an n-digit number is pandigital
//! if it makes use of all the digits 1 to n exactly once;
//! for example, the 5-digit number, 15234, is 1
//! through 5 pandigital.
//!
//! The product 7254 is unusual, as the identity,
//! 39 × 186 = 7254, containing multiplicand,
//! multiplier, and product is 1 through 9 pandigital.
//!
//! Find the sum of all products whose
//! multiplicand/multiplier/product identity
//! can be written as a 1 through 9 pandigital.
//!
//! HINT: Some products can be obtained in more
//! than one way so be sure to only include
//! it once in your sum.
use std::{cmp::Ordering, collections::HashMap};

use super::Problem;

crate::base_problem!(45228, "Pandigital products");

fn get_result_problem() -> i64 {
    let mut map: HashMap<i32, String> = HashMap::new();

    for f1 in 1..1000 {
        for f2 in (f1 + 1)..10000 {
            let product = f1 * f2;

            let str = format!("{f1}{f2}{product}");

            let mut is_pandigital = true;
            match str.len().cmp(&9) {
                Ordering::Less => continue,
                Ordering::Greater => break,
                Ordering::Equal => {
                    for c in '1'..='9' {
                        if !str.contains(c) {
                            is_pandigital = false;
                            break;
                        }
                    }
                }
            }

            if is_pandigital {
                map.insert(product, format!("{f1}*{f2}={product}"));
            }
        }
    }
    map.keys().sum::<i32>() as i64
}
