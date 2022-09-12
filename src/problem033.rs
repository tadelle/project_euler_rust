//! Digit cancelling fractions
//! 
//! The fraction 49/98 is a curious fraction, as an 
//! inexperienced mathematician in attempting to 
//! simplify it may incorrectly believe that 49/98 = 4/8, 
//! which is correct, is obtained by cancelling the 9s.
//! 
//! We shall consider fractions like, 30/50 = 3/5, 
//! to be trivial examples.
//! 
//! There are exactly four non-trivial examples of 
//! this type of fraction, less than one in value, 
//! and containing two digits in the numerator and denominator.
//! 
//! If the product of these four fractions is given in its 
//! lowest common terms, find the value of the denominator.
use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Digit cancelling fractions")
    }

    fn get_result(&self) -> i64 {
        get_digit_cancelling_fractions() as i64
    }
}

fn get_digit_cancelling_fractions() -> i64 {
    let mut num_acc = 1;
    let mut den_acc = 1;

    for numerator in 11..100 {
        let n1 = numerator / 10;
        let n2 = numerator % 10;
        for denominator in (numerator + 1)..100 {
            if numerator % 10 == 0 || denominator % 10 == 0 || numerator == denominator {
                continue;
            }
            let d1 = denominator / 10;
            let d2 = denominator % 10;

            let res = numerator as f64 / denominator as f64;
            if  (n1 == d1 && res == n2 as f64 / d2 as f64) || 
                (n1 == d2 && res == n2 as f64 / d1 as f64) ||
                (n2 == d1 && res == n1 as f64 / d2 as f64) || 
                (n2 == d2 && res == n1 as f64 / d1 as f64) {
                    num_acc *= numerator;
                    den_acc *= denominator;
            }
        }
    }

    let numerator = num_acc;

    let mut count = 2;
    while num_acc % count == 0 && den_acc % count == 0 {
        num_acc /= count;
        den_acc /= count;
    }

    count = 3;
    while count * count < numerator {
        while num_acc % count == 0 && den_acc % count == 0 {
            num_acc /= count;
            den_acc /= count;
        }
        count += 2;
    }

    den_acc
}

#[cfg(test)]
mod test033 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 100)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Digit cancelling fractions")
    }
}

