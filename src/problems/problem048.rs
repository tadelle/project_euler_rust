//! Self powers
//!
//! The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
//!
//! Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
use super::Problem;

crate::base_problem!(9110846700, "Self powers");

fn get_result_problem() -> i64 {
    let limit: i64 = 10_000_000_000;
    let mut sum: i64 = 0;
    let numbers = (1..1000).filter(|num| num % 10 != 0).collect::<Vec<i64>>();

    for number in numbers {
        let mut mult = 1;
        for _ in 0..number {
            mult *= number;
            mult %= limit;
        }
        sum += mult % limit;
    }

    (sum % limit) as i64
}
