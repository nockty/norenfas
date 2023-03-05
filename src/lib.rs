pub fn solve(sudoku: &mut [u8; 81]) {
    rec_solve(0, sudoku);
}

fn rec_solve(i: usize, sudoku: &mut [u8; 81]) -> bool {
    if sudoku[i] != 0 {
        return rec_solve(i + 1, sudoku);
    }
    for n in 1..10 {
        if !is_tile_valid(n, i, sudoku) {
            continue;
        }
        // try using n in the tile
        sudoku[i] = n;
        pretty_print(sudoku);
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
    for m in &grid[line * 9..line * 9 + 9] {
        if n == *m {
            return false;
        }
    }

    // check that the same number is not in the same column
    let col = i % 9;
    for i in 0..9 {
        if n == grid[i * 9 + col] {
            return false;
        }
    }

    // check that the same number is not in the same square
    let square_line = line / 3;
    let square_col = col / 3;
    for x in square_col..square_col + 3 {
        for y in square_line..square_line + 3 {
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
    fn is_tile_valid() {
        assert_eq!(true, true);
    }
}
