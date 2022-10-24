//! Coin sums
//!
//! In the United Kingdom the currency is
//! made up of pound (£) and pence (p).
//! There are eight coins in general circulation:
//!
//! 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
//! It is possible to make £2 in the following way:
//!
//! 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
//! How many different ways can £2 be made
//! using any number of coins?
use super::Problem;

crate::base_problem!(73682, "Coin sums");

fn get_result_problem() -> i64 {
    count_coins() as i64
}

fn count_coins() -> i32 {
    let coins: [i32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    let mut combo = [0; 201];

    combo[0] = 1;

    for coin in coins {
        for i in coin..201 {
            combo[i as usize] += combo[(i - coin) as usize];
        }
    }

    *combo.last().unwrap_or(&0)
}
