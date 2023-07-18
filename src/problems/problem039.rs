//! Integer right triangles
//!
//! If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.
//!
//! {20,48,52}, {24,45,51}, {30,40,50}
//!
//! For which value of p ≤ 1000, is the number of solutions maximised?
use super::Problem;

crate::base_problem!(840, "Integer right triangles");

fn get_result_problem() -> i64 {
    let limit: i32 = 1000;
    let mut counter = 12;
    let mut num_max_solution = 0;
    let mut result = 0;

    while counter < limit {
        let num_solutions = get_solutions(counter);
        if num_solutions > num_max_solution {
            num_max_solution = num_solutions;
            result = counter;
        }
        counter += 1;
    }
    result as i64
}

fn get_solutions(perimeter: i32) -> i32 {
    let mut count = 0;
    let limit = (perimeter / 2) - 1;

    let mut a: i32 = 1;
    while a < limit {
        let mut b: i32 = 1;
        while b < (perimeter - a) {
            let c = perimeter - (a + b);
            if c > 0 && a * a == b * b + c * c {
                count += 1;
                break;
            }
            b += 1;
        }
        a += 1;
    }
    count
}
