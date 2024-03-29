use std::thread::{self, JoinHandle};
mod problem001;
mod problem002;
mod problem003;
mod problem004;
mod problem005;
mod problem006;
mod problem007;
mod problem008;
mod problem009;
mod problem010;
mod problem011;
mod problem012;
mod problem013;
mod problem014;
mod problem015;
mod problem016;
mod problem017;
mod problem018;
mod problem019;
mod problem020;
mod problem021;
mod problem022;
mod problem023;
mod problem024;
mod problem025;
mod problem026;
mod problem027;
mod problem028;
mod problem029;
mod problem030;
mod problem031;
mod problem032;
mod problem033;
mod problem034;
mod problem035;
mod problem036;
mod problem037;
mod problem038;
mod problem039;
mod problem040;
mod problem041;
mod problem042;
mod problem043;
mod problem044;
mod problem045;
mod problem046;
mod problem047;
mod problem048;
mod problem049;
mod problem050;
mod problem051;
mod problem052;
mod problem053;
mod problem054;
mod problem055;
mod problem056;
mod problem057;
mod problem058;
mod problem067;
mod problem059;
mod problem060;

pub trait Problem {
    fn new() -> Self;
    fn get_title(&self) -> String;
    fn get_result(&self) -> i64;
}

#[macro_export]
macro_rules! base_problem {
    ($result: expr, $title: expr) => {
        pub struct Problema;

        impl Problem for Problema {
            fn new() -> Problema {
                Problema {}
            }

            fn get_title(&self) -> String {
                String::from($title)
            }

            fn get_result(&self) -> i64 {
                get_result_problem()
            }
        }

        #[test]
        fn test_get_result() {
            assert_eq!(Problema::new().get_result(), $result);
        }
    };
}

pub fn get_result(number: i32) -> i64 {
    match number {
        1 => problem001::Problema::new().get_result(),
        2 => problem002::Problema::new().get_result(),
        3 => problem003::Problema::new().get_result(),
        4 => problem004::Problema::new().get_result(),
        5 => problem005::Problema::new().get_result(),
        6 => problem006::Problema::new().get_result(),
        7 => problem007::Problema::new().get_result(),
        8 => problem008::Problema::new().get_result(),
        9 => problem009::Problema::new().get_result(),
        10 => problem010::Problema::new().get_result(),
        11 => problem011::Problema::new().get_result(),
        12 => problem012::Problema::new().get_result(),
        13 => problem013::Problema::new().get_result(),
        14 => problem014::Problema::new().get_result(),
        15 => problem015::Problema::new().get_result(),
        16 => problem016::Problema::new().get_result(),
        17 => problem017::Problema::new().get_result(),
        18 => problem018::Problema::new().get_result(),
        19 => problem019::Problema::new().get_result(),
        20 => problem020::Problema::new().get_result(),
        21 => problem021::Problema::new().get_result(),
        22 => problem022::Problema::new().get_result(),
        23 => problem023::Problema::new().get_result(),
        24 => problem024::Problema::new().get_result(),
        25 => problem025::Problema::new().get_result(),
        26 => problem026::Problema::new().get_result(),
        27 => problem027::Problema::new().get_result(),
        28 => problem028::Problema::new().get_result(),
        29 => problem029::Problema::new().get_result(),
        30 => problem030::Problema::new().get_result(),
        31 => problem031::Problema::new().get_result(),
        32 => problem032::Problema::new().get_result(),
        33 => problem033::Problema::new().get_result(),
        34 => problem034::Problema::new().get_result(),
        35 => problem035::Problema::new().get_result(),
        36 => problem036::Problema::new().get_result(),
        37 => problem037::Problema::new().get_result(),
        38 => problem038::Problema::new().get_result(),
        39 => problem039::Problema::new().get_result(),
        40 => problem040::Problema::new().get_result(),
        41 => problem041::Problema::new().get_result(),
        42 => problem042::Problema::new().get_result(),
        43 => problem043::Problema::new().get_result(),
        44 => problem044::Problema::new().get_result(),
        45 => problem045::Problema::new().get_result(),
        46 => problem046::Problema::new().get_result(),
        47 => problem047::Problema::new().get_result(),
        48 => problem048::Problema::new().get_result(),
        49 => problem049::Problema::new().get_result(),
        50 => problem050::Problema::new().get_result(),
        51 => problem051::Problema::new().get_result(),
        52 => problem052::Problema::new().get_result(),
        53 => problem053::Problema::new().get_result(),
        54 => problem054::Problema::new().get_result(),
        55 => problem055::Problema::new().get_result(),
        56 => problem056::Problema::new().get_result(),
        57 => problem057::Problema::new().get_result(),
        58 => problem058::Problema::new().get_result(),
        59 => problem059::Problema::new().get_result(),
        60 => problem060::Problema::new().get_result(),
        67 => problem067::Problema::new().get_result(),
        _ => 0,
    }
}

pub fn get_title(number: i32) -> String {
    match number {
        1 => problem001::Problema::new().get_title(),
        2 => problem002::Problema::new().get_title(),
        3 => problem003::Problema::new().get_title(),
        4 => problem004::Problema::new().get_title(),
        5 => problem005::Problema::new().get_title(),
        6 => problem006::Problema::new().get_title(),
        7 => problem007::Problema::new().get_title(),
        8 => problem008::Problema::new().get_title(),
        9 => problem009::Problema::new().get_title(),
        10 => problem010::Problema::new().get_title(),
        11 => problem011::Problema::new().get_title(),
        12 => problem012::Problema::new().get_title(),
        13 => problem013::Problema::new().get_title(),
        14 => problem014::Problema::new().get_title(),
        15 => problem015::Problema::new().get_title(),
        16 => problem016::Problema::new().get_title(),
        17 => problem017::Problema::new().get_title(),
        18 => problem018::Problema::new().get_title(),
        19 => problem019::Problema::new().get_title(),
        20 => problem020::Problema::new().get_title(),
        21 => problem021::Problema::new().get_title(),
        22 => problem022::Problema::new().get_title(),
        23 => problem023::Problema::new().get_title(),
        24 => problem024::Problema::new().get_title(),
        25 => problem025::Problema::new().get_title(),
        26 => problem026::Problema::new().get_title(),
        27 => problem027::Problema::new().get_title(),
        28 => problem028::Problema::new().get_title(),
        29 => problem029::Problema::new().get_title(),
        30 => problem030::Problema::new().get_title(),
        31 => problem031::Problema::new().get_title(),
        32 => problem032::Problema::new().get_title(),
        33 => problem033::Problema::new().get_title(),
        34 => problem034::Problema::new().get_title(),
        35 => problem035::Problema::new().get_title(),
        36 => problem036::Problema::new().get_title(),
        37 => problem037::Problema::new().get_title(),
        38 => problem038::Problema::new().get_title(),
        39 => problem039::Problema::new().get_title(),
        40 => problem040::Problema::new().get_title(),
        41 => problem041::Problema::new().get_title(),
        42 => problem042::Problema::new().get_title(),
        43 => problem043::Problema::new().get_title(),
        44 => problem044::Problema::new().get_title(),
        45 => problem045::Problema::new().get_title(),
        46 => problem046::Problema::new().get_title(),
        47 => problem047::Problema::new().get_title(),
        48 => problem048::Problema::new().get_title(),
        49 => problem049::Problema::new().get_title(),
        50 => problem050::Problema::new().get_title(),
        51 => problem051::Problema::new().get_title(),
        52 => problem052::Problema::new().get_title(),
        53 => problem053::Problema::new().get_title(),
        54 => problem054::Problema::new().get_title(),
        55 => problem055::Problema::new().get_title(),
        56 => problem056::Problema::new().get_title(),
        57 => problem057::Problema::new().get_title(),
        58 => problem058::Problema::new().get_title(),
        59 => problem059::Problema::new().get_title(),
        60 => problem060::Problema::new().get_title(),
        67 => problem067::Problema::new().get_title(),
        _ => String::from("Not implemented yet!"),
    }
}

pub fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    }
    if number == 2 {
        return true;
    }
    if number % 2 == 0 {
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

    for (index, item) in vec_numbers.iter().enumerate().skip(5) {
        if *item {
            primes.push(index as i32);
        }
    }

    primes.clone()
}

pub fn get_number_of_divisors(number: i32) -> i32 {
    let limit = (number as f64).sqrt() as i32;
    let mut counter = 2;

    if limit * limit == number {
        counter -= 1
    }

    for index in 2..(limit + 1) {
        if number % index == 0 {
            counter += 2;
        }
    }
    counter
}

pub fn get_sum_function_parallel(
    fun: fn(usize, usize) -> i32,
    initial_value: usize,
    final_value: usize,
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

pub fn get_factorial(number: i32) -> i32 {
    if number == 1 || number == 0 {
        1
    } else {
        number * get_factorial(number - 1)
    }
}

pub fn power_vec(number: i32, power: i32) -> Vec<u8> {
    let mut vec_num: Vec<u8> = number.to_string().chars().map(|b| b as u8 - b'0').collect();
    vec_num.reverse();

    for _ in 1..power {
        let vec = vec_num.clone();
        for _ in 1..number {
            vec_num = add_vec(&vec, &vec_num)
        }
    }

    vec_num.reverse();

    vec_num
}

pub fn add_vec(vec1: &Vec<u8>, vec2: &Vec<u8>) -> Vec<u8> {
    let mut result: u8;
    let mut remainder: u8 = 0;
    let mut vec_res: Vec<u8> = Vec::new();

    let v1 = if vec1.len() > vec2.len() { vec1 } else { vec2 };
    let v2 = if vec1.len() > vec2.len() { vec2 } else { vec1 };

    let mut index = 0;
    for digit in v1 {
        let vec2_digit = if v2.len() > index { v2[index] } else { 0 };
        result = digit + vec2_digit + remainder;
        vec_res.push(result % 10);
        remainder = result / 10;
        index += 1;
    }
    while remainder > 0 || v2.len() > index {
        if index < v2.len() {
            result = v2[index] + remainder;
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
    assert_eq!(power_vec(9, 5), vec!(5, 9, 0, 4, 9));
    assert_eq!(power_vec(789, 2), vec!(6, 2, 2, 5, 2, 1));
    assert_eq!(
        power_vec(555, 5),
        vec!(5, 2, 6, 5, 8, 0, 6, 7, 3, 4, 6, 8, 7, 5)
    );
}

#[test]
fn test_get_number_of_divisors() {
    assert_eq!(get_number_of_divisors(6), 4);
    assert_eq!(get_number_of_divisors(2), 2);
    assert_eq!(get_number_of_divisors(3), 2);
    assert_eq!(get_number_of_divisors(4), 3);
    assert_eq!(get_number_of_divisors(28), 6);
}
