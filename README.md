# norenfas

Norenfas is a toy project for me to practice basic Rust skills. It is a sudoku solver.

## Benchmarks

The `benches` folder contains a couple [Criterion.rs](https://bheisler.github.io/criterion.rs/book/criterion_rs.html) benchmarks. To run them: `cargo bench`.

Typical results on my machine:

```
solve easy:   ~11µs
solve medium: ~380µs
solve hard:   ~560µs
```

To profile and get a [flamegraph](https://github.com/flamegraph-rs/flamegraph): `cargo flamegraph --root --bench solve_benchmark -- --bench`. Well, it isn't that useful for this project that has about two functions.

## Sudoku solutions online

https://www.sudoku-solutions.com/
