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
use super::Problem;

crate::base_problem!(31875000, "Special Pythagorean triplet");

fn get_result_problem() -> i64 {
    for n1 in 1..1000 {
        for n2 in n1..(1000 - n1) {
            let n3 = 1000 - (n1 + n2);
            if n1 * n1 + n2 * n2 == n3 * n3 {
                return (n1 * n2 * n3) as i64;
            }
        }
    }
    0
}
