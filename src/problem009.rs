//! Special Pythagorean triplet
//!
//! A Pythagorean triplet is a set of three
//! natural numbers, a < b < c, for which,
//!
//! a^2 + b^2 = c^2
//! For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//! Find the product abc.
use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Special Pythagorean triplet")
    }

    fn get_result(&self) -> i64 {
        get_special_pythagorean_triplet() as i64
    }
}

fn get_special_pythagorean_triplet() -> i32 {
    for n1 in 1..1000 {
        for n2 in n1..(1000 - n1) {
            let n3 = 1000 - (n1 + n2);
            if n1 * n1 + n2 * n2 == n3 * n3 {
                return n1 * n2 * n3;
            }
        }
    }
    0
}

#[cfg(test)]
mod test009 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 31875000)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Special Pythagorean triplet")
    }
}
