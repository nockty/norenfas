use criterion::{black_box, criterion_group, criterion_main, Criterion};
use norenfas::solve;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve easy", |b| {
        b.iter(|| {
            solve(black_box(&mut [
                8, 0, 6, 0, 2, 0, 5, 0, 7, //
                0, 0, 2, 0, 0, 0, 4, 0, 0, //
                3, 7, 0, 0, 0, 0, 0, 9, 1, //
                //
                0, 0, 0, 4, 5, 6, 0, 0, 0, //
                5, 0, 0, 1, 0, 3, 0, 0, 6, //
                0, 0, 0, 8, 7, 2, 0, 0, 0, //
                //
                4, 3, 0, 0, 0, 0, 0, 7, 5, //
                0, 0, 5, 0, 0, 0, 9, 0, 0, //
                7, 0, 1, 0, 4, 0, 6, 0, 3, //
            ]))
        })
    });

    c.bench_function("solve medium", |b| {
        b.iter(|| {
            solve(black_box(&mut [
                0, 4, 6, 0, 1, 2, 0, 0, 0, //
                0, 1, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 4, 6, 0, 0, 0, 0, //
                //
                0, 5, 0, 9, 0, 8, 1, 4, 0, //
                0, 0, 3, 0, 0, 0, 0, 0, 8, //
                7, 0, 0, 0, 0, 0, 9, 0, 0, //
                //
                3, 6, 8, 1, 0, 0, 0, 0, 4, //
                0, 0, 0, 0, 7, 0, 5, 0, 0, //
                0, 0, 0, 0, 0, 3, 0, 6, 1, //
            ]))
        })
    });

    c.bench_function("solve hard", |b| {
        b.iter(|| {
            solve(black_box(&mut [
                5, 0, 0, 0, 2, 0, 0, 0, 0, //
                0, 0, 0, 1, 0, 7, 0, 5, 0, //
                0, 8, 0, 0, 0, 9, 0, 0, 7, //
                //
                0, 0, 0, 0, 4, 0, 0, 6, 2, //
                0, 0, 5, 0, 8, 0, 4, 0, 0, //
                2, 7, 0, 0, 9, 0, 0, 0, 0, //
                //
                7, 0, 0, 5, 0, 0, 0, 8, 0, //
                0, 1, 0, 4, 0, 3, 0, 0, 0, //
                0, 0, 0, 0, 1, 0, 0, 0, 4, //
            ]))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
