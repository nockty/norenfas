#![feature(portable_simd)]
use std::simd::{u8x8, SimdPartialEq};

pub fn solve(sudoku: &mut [u8; 81]) -> bool {
    let mut sudoku = Sudoku::new(sudoku);
    rec_solve(0, &mut sudoku)
}

fn rec_solve(i: usize, sudoku: &mut Sudoku) -> bool {
    if i >= 81 {
        return true;
    }
    if sudoku.get(i) != 0 {
        return rec_solve(i + 1, sudoku);
    }
    for n in 1..10 {
        if !sudoku.is_tile_valid(i, n) {
            continue;
        }
        // try using n in the tile
        sudoku.set(i, n);
        if rec_solve(i + 1, sudoku) {
            return true;
        }
    }
    // nothing works: reset the tile and backtrack
    sudoku.set(i, 0);
    return false;
}

/// Sudoku holds a line-by-line representation and column-by-column representation of the same sudoku. The goal is
/// to be able to use SIMD operations for both the line check and the column check.
struct Sudoku<'a> {
    line_grid: &'a mut [u8; 81],
    col_grid: [u8; 81],
    square_grid: [u8; 81],
}

impl Sudoku<'_> {
    fn new(grid: &mut [u8; 81]) -> Sudoku {
        // create the transposed sudoku (column-by-column representation)
        let mut transposed_col = [0u8; 81];
        for i in 0..81 {
            transposed_col[Self::transpose_col(i)] = grid[i]
        }
        let mut transposed_square = [0u8; 81];
        for i in 0..81 {
            transposed_square[Self::transpose_square(i)] = grid[i]
        }

        Sudoku {
            line_grid: grid,
            col_grid: transposed_col,
            square_grid: transposed_square,
        }
    }

    /// transform an index of a line-by-line \[u8; 81] representation of a sudoku to the index of
    /// the column-by-column \[u8; 81] representation of the same sudoku
    fn transpose_col(i: usize) -> usize {
        (i * 9) % 81 + i / 9
    }

    fn transpose_square(i: usize) -> usize {
        (((i % 9) / 3) % 3) * 26 + i % 3 + i / 3
    }

    fn get(&self, i: usize) -> u8 {
        self.line_grid[i]
    }

    fn set(&mut self, i: usize, n: u8) {
        self.line_grid[i] = n;
        self.col_grid[Self::transpose_col(i)] = n;
        self.square_grid[Self::transpose_square(i)] = n;
    }

    /// check that n would be a valid number at index i in the sudoku
    fn is_tile_valid(&self, i: usize, n: u8) -> bool {
        assert!(i < 81);

        let n_vec = u8x8::from_array([n; 8]);

        // check that the same number is not in the same line
        let start = (i / 9) * 9;
        let vec = u8x8::from_slice(&self.line_grid[start..start + 9]);
        if n_vec.simd_eq(vec).any() || n == self.line_grid[start + 8] {
            return false;
        }

        // check that the same number is not in the same column
        let start = (i % 9) * 9;
        let vec = u8x8::from_slice(&self.col_grid[start..start + 9]);
        if n_vec.simd_eq(vec).any() || n == self.col_grid[start + 8] {
            return false;
        }

        // check that the same number is not in the same square
        let start = (Sudoku::transpose_square(i) / 9) * 9;
        let vec = u8x8::from_slice(&self.square_grid[start..start + 9]);
        if n_vec.simd_eq(vec).any() || n == self.square_grid[start + 8] {
            return false;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves() {
        struct TestCase {
            input: [u8; 81],
            expected: [u8; 81],
        }
        let mut test_cases: Vec<TestCase> = vec![
            TestCase {
                input: [
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
                ],
                expected: [
                    8, 1, 6, 9, 2, 4, 5, 3, 7, //
                    9, 5, 2, 3, 1, 7, 4, 6, 8, //
                    3, 7, 4, 6, 8, 5, 2, 9, 1, //
                    //
                    1, 9, 7, 4, 5, 6, 3, 8, 2, //
                    5, 2, 8, 1, 9, 3, 7, 4, 6, //
                    6, 4, 3, 8, 7, 2, 1, 5, 9, //
                    //
                    4, 3, 9, 2, 6, 1, 8, 7, 5, //
                    2, 6, 5, 7, 3, 8, 9, 1, 4, //
                    7, 8, 1, 5, 4, 9, 6, 2, 3, //
                ],
            },
            TestCase {
                input: [
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
                ],
                expected: [
                    5, 9, 7, 8, 2, 4, 1, 3, 6, //
                    4, 2, 6, 1, 3, 7, 8, 5, 9, //
                    3, 8, 1, 6, 5, 9, 2, 4, 7, //
                    //
                    1, 3, 8, 7, 4, 5, 9, 6, 2, //
                    9, 6, 5, 2, 8, 1, 4, 7, 3, //
                    2, 7, 4, 3, 9, 6, 5, 1, 8, //
                    //
                    7, 4, 9, 5, 6, 2, 3, 8, 1, //
                    8, 1, 2, 4, 7, 3, 6, 9, 5, //
                    6, 5, 3, 9, 1, 8, 7, 2, 4, //
                ],
            },
            TestCase {
                input: [
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
                ],
                expected: [
                    8, 4, 6, 5, 1, 2, 3, 7, 9, //
                    2, 1, 7, 3, 8, 9, 4, 5, 6, //
                    9, 3, 5, 4, 6, 7, 8, 1, 2, //
                    //
                    6, 5, 2, 9, 3, 8, 1, 4, 7, //
                    4, 9, 3, 7, 5, 1, 6, 2, 8, //
                    7, 8, 1, 2, 4, 6, 9, 3, 5, //
                    //
                    3, 6, 8, 1, 2, 5, 7, 9, 4, //
                    1, 2, 9, 6, 7, 4, 5, 8, 3, //
                    5, 7, 4, 8, 9, 3, 2, 6, 1, //
                ],
            },
        ];
        for tc in &mut test_cases {
            let solved = solve(&mut tc.input);
            assert_eq!(true, solved);
            assert_eq!(tc.expected, tc.input);
        }
    }

    #[test]
    fn it_does_not_solve() {
        let mut invalid: [u8; 81] = [
            0, 4, 6, 0, 1, 2, 0, 0, 0, //
            0, 1, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 4, 6, 0, 0, 0, 1, //
            //
            0, 5, 0, 9, 0, 8, 1, 4, 0, //
            0, 0, 3, 0, 0, 0, 0, 0, 8, //
            7, 0, 0, 0, 0, 0, 9, 0, 0, //
            //
            3, 6, 8, 1, 0, 0, 0, 0, 4, //
            0, 0, 0, 0, 7, 0, 5, 0, 0, //
            0, 0, 0, 0, 0, 3, 0, 6, 1, //
        ];
        let solved = solve(&mut invalid);
        assert_eq!(false, solved);
    }
}

const FULL_LINE: &str = "+-----+-----+-----+";

pub fn pretty_print(sudoku: &[u8; 81]) {
    for line in 0..9 {
        if line % 3 == 0 {
            println!("{}", FULL_LINE);
        }
        print!("|");
        for col in 0..9 {
            let n = sudoku[line * 9 + col];
            match n {
                0 => print!(" "),
                _ => print!("{}", n),
            }
            print!("|");
        }
        println!()
    }
    println!("{}", FULL_LINE);
}
