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

pub fn get_primes_erastostenes(number: i32) -> Vec<i32> {
    let mut primes: Vec<i32> = Vec::new();
    let mut vec_numbers = vec![true; number as usize];

    if number > 2 {
        primes.push(2);
    }
    if number > 3 {
        primes.push(3);
    }

    let mut index: i32 = 4;

    while index < number {
        vec_numbers[index as usize] = false;
        index += 2;
    }

    let mut prime = 3;
    while prime * prime < number {
        index = prime + prime;
        while index < number {
            vec_numbers[index as usize] = false;
            index += prime;
        }

        let mut i = prime + 1;
        prime = 0;
        while i < number {
            if vec_numbers[i as usize] {
                prime = i;
                break;
            }
            i += 1;
        }

        if prime == 0 {
            break;
        }
    }
    for index in 5..vec_numbers.len() {
        if vec_numbers[index] {
            primes.push(index as i32);
        }
    }

    primes.clone()
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

    #[test]
    fn test_get_primes_erastostenes() {
        let sum = get_primes_erastostenes(2_000_000)
            .into_iter()
            .map(|num| num as i64)
            .sum::<i64>();
        assert_eq!(sum, 142_913_828_922)
    }
}
