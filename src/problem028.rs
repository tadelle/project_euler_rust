//! Number spiral diagonals
//!
//! Starting with the number 1 and moving to
//! the right in a clockwise direction a 5 by 5
//! spiral is formed as follows:
//!
//! 21 22 23 24 25
//! 20  7  8  9 10
//! 19  6  1  2 11
//! 18  5  4  3 12
//! 17 16 15 14 13
//!
//! It can be verified that the sum of the
//! numbers on the diagonals is 101.
//!
//! What is the sum of the numbers on the diagonals
//! in a 1001 by 1001 spiral formed in the same way?
use super::problems::Problem;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Number spiral diagonals")
    }

    fn get_result(&self) -> i64 {
        get_number_spiral_diagonals() as i64
    }
}

fn get_number_spiral_diagonals() -> i32 {
    let matrix_length = 1001;
    let mut counter = 3;
    let mut soma = 0;
    while counter <= matrix_length {
        let quadrado = counter * counter;
        let distancia = counter - 1;

        soma += 4 * quadrado - 6 * distancia;

        counter += 2;
    }
    soma + 1
}

#[cfg(test)]
mod test028 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 669_171_001)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Number spiral diagonals")
    }
}
