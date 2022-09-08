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
use std::collections::HashMap;

use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Pandigital products")
    }

    fn get_result(&self) -> i64 {
        get_pandigital_products() as i64
    }
}

fn get_pandigital_products() -> i32 {
    let mut map: HashMap<i32, String> = HashMap::new();

    for f1 in 1..1000 {
        for f2 in (f1 + 1)..10000 {
            let product = f1 * f2;

            let str = format!("{f1}{f2}{product}");

            if str.len() < 9 {
                continue;
            } else if str.len() > 9 {
                break;
            }

            let mut is_pandigital = true;
            for c in '1'..='9' {
                if !str.contains(c) {
                    is_pandigital = false;
                    break;
                }
            }

            if is_pandigital {
                map.insert(product, format!("{f1}*{f2}={product}"));
            }
        }
    }
    map.keys().sum()
}

#[cfg(test)]
mod test032 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 45228)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Pandigital products")
    }
}
