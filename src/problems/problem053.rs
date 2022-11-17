//! Combinatoric selections
//! 
//! There are exactly ten ways of selecting three from five, 12345:
//! 
//! 123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
//! 
//! In combinatorics, we use the notation, 
//! (5) = 10
//! (3)
//! 
//! In general,
//! (n) = n! / r!(n-r)!, where r <= n, n! = n x (n-1) x .. 2 x 1, and 0! = 1.
//! (r)
//! 
//! It is not until n = 23, that a value exceeds one-million: 
//! (23) = 1144066.
//! (10)
//! How many, not necessarily distinct, values of 
//! (n)  for 1 <= n <= 100, are greater than one-million?
//! (r)
use super::Problem;

crate::base_problem!(4075, "Combinatoric selections");

fn get_result_problem() -> i64 {
    const LIMIT: i128 = 1_000_000;
    let mut count = 0;

    for n in 1..=100 {
        for r in 1..=n {
            let res = get_combinatoric(n, r);
            if res > LIMIT {
                count += n - (r * 2 - 1);
                break;
            }
        }
    }
    count as i64
}

fn get_combinatoric(n: i32, r: i32) -> i128 {

    let mut n1: i128 = 1;
    let n_minus_r = n - r;

    if r > n_minus_r {
        for i in (r + 1)..=n {
            n1 *= i as i128;
        }

        n1 / get_factorial(n - r)

    } else {
        for i in (n_minus_r + 1)..=n {
            n1 *= i as i128;
        }

        n1 / get_factorial(r)
    }
}

fn get_factorial(number: i32) -> i128 {

    let mut res = 1;
    for n in 2..=number {
        res *= n as i128;
    }
    res
}
