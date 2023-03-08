#![feature(portable_simd)]
use std::simd::{u8x8, SimdPartialEq};

pub fn solve(sudoku: &mut [u8; 81]) -> bool {
    rec_solve(0, sudoku)
}

fn rec_solve(i: usize, sudoku: &mut [u8; 81]) -> bool {
    if i >= 81 {
        return true;
    }
    if sudoku[i] != 0 {
        return rec_solve(i + 1, sudoku);
    }
    for n in 1..10 {
        if !is_tile_valid(n, i, sudoku) {
            continue;
        }
        // try using n in the tile
        sudoku[i] = n;
        if rec_solve(i + 1, sudoku) {
            return true;
        }
    }
    // nothing works: reset the tile and backtrack
    sudoku[i] = 0;
    return false;
}

fn is_tile_valid(n: u8, i: usize, grid: &[u8; 81]) -> bool {
    // check that the same number is not in the same line
    let line = i / 9;
    // use SIMD vectors for faster comparison
    let n_vec = u8x8::from_array([n; 8]);
    let line_vec = u8x8::from_slice(&grid[line * 9..line * 9 + 9]);
    if n_vec.simd_eq(line_vec).any() || n == grid[line * 9 + 8] {
        return false;
    }

    // check that the same number is not in the same column
    let col = i % 9;
    for j in 0..9 {
        if n == grid[j * 9 + col] {
            return false;
        }
    }

    // check that the same number is not in the same square
    let square_line = line / 3;
    let square_col = col / 3;
    for y in square_line * 3..square_line * 3 + 3 {
        for x in square_col * 3..square_col * 3 + 3 {
            if n == grid[y * 9 + x] {
                return false;
            }
        }
    }

    return true;
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
