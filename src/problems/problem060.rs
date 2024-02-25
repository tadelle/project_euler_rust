//! Prime Pair Sets
//!
//! The primes 3, 7, 109, and 673, are quite remarkable.
//! By taking any two primes and concatenating them in any order the result will
//! always be prime. For example, taking 7 and 109, both 7109 and 1097 are prime.
//! The sum of these four primes, 792, represents
//! the lowest sum for a set of four primes with this property.
//!
//! Find the lowest sum for a set of five primes for which any
//! two primes concatenate to produce another prime.
use crate::problems::get_primes_eratostenes;
use super::{is_prime, Problem};

crate::base_problem!(26033, "Prime Pair Sets");

fn get_result_problem() -> i64 {

    let mut primes = get_primes_eratostenes(10_000);
    let mut combo: Vec<(i32, i32, i32, i32, i32)> = Vec::new();

    primes.remove(0); // Remove prime '2'
    primes.remove(1); // Remove prime '5'

    let len = primes.len();

    for idx1 in 0..len {
        for idx2 in (idx1 + 1)..len {
            if !is_special_prime(primes[idx1], primes[idx2]) {
                continue;
            }

            for idx3 in (idx2 + 1)..len {
                if !is_special_prime(primes[idx1], primes[idx3]) {
                    continue;
                }
                if !is_special_prime(primes[idx2], primes[idx3]) {
                    continue;
                }

                for idx4 in (idx3 + 1)..len {
                    if !is_special_prime(primes[idx1], primes[idx4]) {
                        continue;
                    }
                    if !is_special_prime(primes[idx2], primes[idx4]) {
                        continue;
                    }
                    if !is_special_prime(primes[idx3], primes[idx4]) {
                        continue;
                    }

                    for idx5 in (idx4 + 1)..len {

                        if !is_special_prime(primes[idx1], primes[idx5]) {
                            continue;
                        }
                        if !is_special_prime(primes[idx2], primes[idx5]) {
                            continue;
                        }
                        if !is_special_prime(primes[idx3], primes[idx5]) {
                            continue;
                        }
                        if !is_special_prime(primes[idx4], primes[idx5]) {
                            continue;
                        }

                        let prime_tuple: (i32, i32, i32, i32, i32) = (primes[idx1], primes[idx2], primes[idx3], primes[idx4], primes[idx5]);
                        combo.push(prime_tuple);
                    }
                }
            }
        }
    }

    combo
        .iter()
        .map(|(n1, n2, n3, n4, n5)| n1 + n2 + n3 + n4 + n5)
        .min().unwrap_or(0) as i64

}

fn is_special_prime(prime1: i32, prime2: i32) -> bool {
    let num1 = prime1 * 10i32.pow((prime2 as f64).log10() as u32 + 1) + prime2;
    let num2 = prime2 * 10i32.pow((prime1 as f64).log10() as u32 + 1) + prime1;

    is_prime(num1) && is_prime(num2)

}
