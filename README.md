# norenfas

Norenfas is a toy project for me to practice basic Rust skills. It is a sudoku solver.

## Rust toolchain

The solver leverages the `portable_simd` feature of the nightly channel, so it requires nightly Rust. To install it: `rustup install nightly`.

## Benchmarks

The `benches` folder contains a couple [Criterion.rs](https://bheisler.github.io/criterion.rs/book/criterion_rs.html) benchmarks. To run them: `cargo bench`.

Typical results on my machine:

```
solve easy:   ~4.7µs
solve medium: ~120µs
solve hard:   ~200µs
```

To profile and get a [flamegraph](https://github.com/flamegraph-rs/flamegraph): `cargo flamegraph --root --bench solve_benchmark -- --bench`. Well, it isn't that useful for this project that has about two functions.

## Resources

Sudoku solutions online: https://www.sudoku-solutions.com/
