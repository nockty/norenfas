#![feature(portable_simd)]
use std::simd::{u8x16, SimdPartialEq};

pub mod io;

pub fn solve(sudoku: &mut [u8; 81]) -> bool {
    let grid = sudoku;
    let mut sudoku = Sudoku::new(grid);

    if rec_solve(0, &mut sudoku) {
        for i in 0..81 {
            grid[i] = sudoku.line_grid[i]
        }
        return true;
    }
    return false;
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

/// Sudoku holds line-by-line, column-by-column, and square-by-square representations of the same sudoku.
/// The goal is to be able to use SIMD operations for all kinds of checks. Their length is 88 so that we
/// can use 128-bit SIMD vectors with 16 elements.
struct Sudoku {
    line_grid: [u8; 88],
    col_grid: [u8; 88],
    square_grid: [u8; 88],
}

impl Sudoku {
    fn new(grid: &mut [u8; 81]) -> Sudoku {
        let mut copy = [0u8; 88];
        let mut transposed_col = [0u8; 88];
        let mut transposed_square = [0u8; 88];

        for i in 0..81 {
            copy[i] = grid[i];
            transposed_col[Self::transpose_col(i)] = grid[i];
            transposed_square[Self::transpose_square(i)] = grid[i];
        }

        Sudoku {
            line_grid: copy,
            col_grid: transposed_col,
            square_grid: transposed_square,
        }
    }

    /// transform an index of a line-by-line \[u8; 81] representation of a sudoku to the index of
    /// the column-by-column \[u8; 81] representation of the same sudoku
    fn transpose_col(i: usize) -> usize {
        (i * 9) % 81 + i / 9
    }

    /// transform an index of a line-by-line \[u8; 81] representation of a sudoku to the index of
    /// the square-by-square \[u8; 81] representation of the same sudoku
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

        // Use a SIMD vector with 16 elements. The first 9 contain n because they will be compared to the line,
        // column, and square. The last 7 contain a number that will never equal another number in the sudoku
        // (it can't be 0 because 0 encodes no number so it is present in the sudoku).
        let n_vec = u8x16::from_array([n, n, n, n, n, n, n, n, n, 10, 10, 10, 10, 10, 10, 10]);

        // check that the same number is not in the same line
        let start = (i / 9) * 9;
        let vec = u8x16::from_slice(&self.line_grid[start..start + 16]);
        if n_vec.simd_eq(vec).any() {
            return false;
        }

        // check that the same number is not in the same column
        let start = (i % 9) * 9;
        let vec = u8x16::from_slice(&self.col_grid[start..start + 16]);
        if n_vec.simd_eq(vec).any() {
            return false;
        }

        // check that the same number is not in the same square
        let start = (Sudoku::transpose_square(i) / 9) * 9;
        let vec = u8x16::from_slice(&self.square_grid[start..start + 16]);
        if n_vec.simd_eq(vec).any() {
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
