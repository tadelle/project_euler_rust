//! Maximum path sum II
//! 
//! By starting at the top of the triangle below and moving to
//! adjacent numbers on the row below, the maximum total 
//! from top to bottom is 23.
//! 
//! 3
//! 7 4
//! 2 4 6
//! 8 5 9 3
//! 
//! That is, 3 + 7 + 4 + 9 = 23.
//! 
//! Find the maximum total from top to bottom in triangle.txt
//! (right click and 'Save Link/Target As...'),
//! a 15K text file containing a triangle with one-hundred rows.
//! 
//! NOTE: This is a much more difficult version of Problem 18.
//! It is not possible to try every route to solve this problem,
//! as there are 2^99 altogether! If you could check one trillion (1012)
//! routes every second it would take over twenty billion years to
//!  check them all. There is an efficient algorithm to solve it. ;o)
use super::problems::Problem;
use std::fs;

pub struct Problema;

impl Problem for Problema {
    fn new() -> Problema {
        Problema {}
    }

    fn get_title(&self) -> String {
        String::from("Maximum path sum II")
    }

    fn get_result(&self) -> i64 {
        get_maximum_path_sum_2() as i64
    }
}

fn get_maximum_path_sum_2() -> i32 {
    let mut triangle = get_triangle();

    for linha in (1..triangle.len()).rev() {
        for coluna in 0..(triangle[linha].len() - 1) {
            if triangle[linha][coluna] > triangle[linha][coluna + 1] {
                triangle[linha - 1][coluna] += triangle[linha][coluna];
            } else {
                triangle[linha - 1][coluna] += triangle[linha][coluna + 1];
            }
        }
    }

    triangle[0][0]
}

fn get_triangle() -> Vec<Vec<i32>> {

    let text = fs::read_to_string("src/data/p067_triangle.txt").unwrap_or(String::from(""));
    let lines = text.lines();

    let mut triangle: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let numbers = line.split(' ')
            .map(|n| n.parse::<i32>().unwrap_or(0))
            .collect::<Vec<_>>();

        triangle.push(numbers);
    }
    triangle.clone()
}

#[cfg(test)]
mod test067 {
    use super::*;

    #[test]
    fn test_get_result() {
        assert_eq!(Problema::new().get_result(), 7273)
    }

    #[test]
    fn test_get_title() {
        assert_eq!(Problema::new().get_title(), "Maximum path sum II")
    }
}

