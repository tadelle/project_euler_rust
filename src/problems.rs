use std::thread::{self, JoinHandle};

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

pub fn get_primes_eratostenes(number: i32) -> Vec<i32> {
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


pub fn get_sum_function_parallel(
    fun: fn(usize, usize) -> i32,
    initial_value: usize,
    final_value: usize
) -> i32 {
    let limit = final_value;
    let cores = num_cpus::get();
    let chunck = limit / cores;
    let mut vec_handle: Vec<JoinHandle<i32>> = Vec::new();

    let mut counter = 0;
    while counter < cores {
        let ini = if counter == 0 {
            initial_value
        } else {
            chunck * counter
        };
        let end = if counter == cores - 1 {
            limit
        } else {
            chunck * (counter + 1)
        };

        vec_handle.push(thread::spawn(move || fun(ini, end)));

        counter += 1;
    }

    let mut result = 0;
    for handle in vec_handle {
        result += handle.join().unwrap_or(0);
    }

    result
}

#[allow(dead_code)]
pub fn power_vec(number: i32, power: i32) -> String {
    let mut vec_num: Vec<u8> = number
        .to_string()
        .chars()
        .into_iter()
        .map(|b| b as u8 - '0' as u8)
        .collect();
    vec_num.reverse();

    for _ in 1..power {
        let vec = vec_num.clone();
        for _ in 1..number {
            vec_num = add_vec(&vec, &vec_num)
        }
    }
    vec_num
        .into_iter()
        .map(|n| n.to_string())
        .reduce(|ac, d| format!("{}{}", d, ac))
        .unwrap_or("".to_string())
}

pub fn add_vec(vec1: &Vec<u8>, vec2: &Vec<u8>) -> Vec<u8> {
    let mut result: u8;
    let mut remainder: u8 = 0;
    let mut vec_res: Vec<u8> = Vec::new();

    let mut index = 0;
    for digit in vec1 {
        result = digit + vec2[index] + remainder;
        vec_res.push(result % 10);
        remainder = result / 10;
        index += 1;
    }
    while remainder > 0 || vec2.len() > index {
        if index < vec2.len() {
            result = vec2[index] + remainder;
            vec_res.push(result % 10);
            remainder = result / 10;
        } else {
            vec_res.push(remainder);
            remainder = 0;
        }
        index += 1;
    }

    vec_res
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
    fn test_get_primes_eratostenes() {
        let sum = get_primes_eratostenes(2_000_000)
            .into_iter()
            .map(|num| num as i64)
            .sum::<i64>();
        assert_eq!(sum, 142_913_828_922)
    }

    #[test]
    fn test_power_vec() {
        assert_eq!(power_vec(9, 5), "59049".to_string());
        assert_eq!(power_vec(789, 2), "622521".to_string());
        assert_eq!(power_vec(555, 5), "52658067346875".to_string());
    }
}
