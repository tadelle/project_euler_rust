//! Self powers
//! 
//! The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
//! 
//! Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Self powers")
    }

    fn get_result(&self) -> i64 {
        get_self_powers() as i64
    }
}

fn get_self_powers() -> i64 {
    
    let limit: i64 = 10_000_000_000;

    let mut sum: i64 = 0;

    for number in 1..1_001 {
        if number % 10 == 0 {
            continue;
        }
        let mut mult = 1;
        for _ in 0..number {
            mult *= number;
            mult %= limit;
        }
        sum += mult % limit;
    }

    (sum % limit) as i64
}

#[cfg(test)]
mod test048 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 9110846700)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Self powers")
    }
}
