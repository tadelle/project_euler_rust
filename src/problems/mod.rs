pub trait Problem {
    fn new() -> Self;
    fn get_title(self: &Self) -> String;
    fn get_result(self: &Self) -> i64;
}

pub fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    } else if number == 2 {
        return true;
    } else if number % 2 == 0 {
        return false;
    }

    let limit = (number as f64).sqrt() as i32 + 1;
    let mut counter = 3;

    while counter < limit {
        if number % counter == 0 {
            return false;
        }
        counter += 2;
    }
    true
}

#[cfg(test)]
mod test_problems {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(11));
        assert!(is_prime(104743));
    }
}
