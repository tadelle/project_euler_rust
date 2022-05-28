use criterion::{black_box, criterion_group, criterion_main, Criterion};
use problem006::Problema;


pub fn get_sum_square_difference_benchmark(c: &mut Criterion) {
    c.bench_function("sum_square into_iter", |b| b.iter(|| Problema::get_result()));
}

criterion_group!(benches, get_sum_square_difference_benchmark);
criterion_main!(benches);