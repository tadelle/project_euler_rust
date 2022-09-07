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
use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Coin sums")
    }

    fn get_result(&self) -> i64 {
        get_coin_sums() as i64
    }
}

fn get_coin_sums() -> i32 {
    let initial_coin = 200u8;
    let number_of_coins = 8;

    count_coins(initial_coin) - number_of_coins

}

fn count_coins(coin: u8) -> i32 {
    if coin == 1u8 {
        1
    } else {

        let vec_coin = match coin {
            200 => vec!(100, 100),
            100 => vec!(50, 50),
            50 => vec!(20, 20, 10),
            20 => vec!(10, 10),
            10 => vec!(5, 5),
            5 => vec!(2, 2, 1),
            2 => vec!(1, 1),
            _ => Vec::new()
        };

        let mut times = 1;
        let len_vec = vec_coin.len() as i32;
        for c in vec_coin {
            times += count_coins(c);
        }

        times * len_vec
    }
}

#[cfg(test)]
mod test031 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 73682)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Coin sums")
    }
}
