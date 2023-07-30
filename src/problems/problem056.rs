//! Powerful Digit Sum
//!
//! A googol (10^100) is a massive number: one followed
//! by one-hundred zeros; 100^100 is almost unimaginably
//! large: one followed by two-hundred zeros. Despite
//! their size, the sum of the digits in each number is only 1.
//!
//! Considering natural numbers of the form, a^b,
//! where a, b < 100, what is the maximum digital sum?
use super::{power_vec, Problem};

crate::base_problem!(972, "Powerful Digit Sum");

// Há um padrão onde a cada seis números há uma repetição
// da soma anterior mais um incremento, ou no mínino mantendo
// o valor anterior. Nesse caso, lendo as seis últimas potências
// é suficiente para verificar a maior soma.

fn get_result_problem() -> i64 {
    (1..100)
        .map(|a| {
            (94..100)
                .map(|b| power_vec(a, b).into_iter().map(|n| n as i32).sum::<i32>())
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0) as i64
}
