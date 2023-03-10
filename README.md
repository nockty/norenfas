# norenfas

Norenfas is a **sudoku solver** written in Rust -- a small project for me to practice basic Rust skills. It is a straightforward solver in that it uses **brute force**: it puts every possible digit in a given tile and recursively tries to solve the sudoku.

Even though the algorithm is simple, I optimized computations by keeping track of 3 different representations of the grid in order to leverage **SIMD vectors** for the 3 different checks that need to be performed -- line, column, and box. The SIMD operations make the solver about **3 times faster**.

## Build

Of course `cargo build` but the solver leverages the `portable_simd` feature of the nightly channel, so it requires nightly Rust to compile. To install it: `rustup install nightly`.

## Benchmarks

The `benches` folder contains a couple [Criterion.rs](https://bheisler.github.io/criterion.rs/book/criterion_rs.html) benchmarks. To run them: `cargo bench`.

Typical results on my machine:

```
solve easy:   ~4.7µs
solve medium: ~120µs
solve hard:   ~200µs
```

To profile and get a [flamegraph](https://github.com/flamegraph-rs/flamegraph), run: `cargo flamegraph --root --bench solve_benchmark -- --bench`. It isn't very useful for a recursive solver that has about two functions, though!

## Resources

Sudoku solutions online: https://www.sudoku-solutions.com/
