use std::{error::Error, fmt, fs};

const FULL_LINE: &str = "+-----+-----+-----+";
const SEPARATOR: &str = "|";

/// Nicely print a sudoku to stdout.
pub fn pretty_print(sudoku: &[u8; 81]) {
    for line in 0..9 {
        if line % 3 == 0 {
            println!("{}", FULL_LINE);
        }
        print!("{}", SEPARATOR);
        for col in 0..9 {
            let n = sudoku[line * 9 + col];
            match n {
                0 => print!(" "),
                _ => print!("{}", n),
            }
            print!("{}", SEPARATOR);
        }
        println!()
    }
    println!("{}", FULL_LINE);
}

/// Build a sudoku from a file. The file must contain 9 lines with 9 characters each. Each character must
/// either be a digit from 1 to 9 or `.`, which represents an empty tile. The returned sudoku is a
/// line-by-line \[u8; 81] representation: index i contains digit in line i // 9, column i % 9.
pub fn parse_from_file(file_path: &str) -> Result<[u8; 81], Box<dyn Error>> {
    let mut sudoku = [0u8; 81];

    let contents = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = contents.lines().collect();

    if lines.len() != 9 {
        return Err(Box::new(InvalidSudokuFile::new(
            "sudoku file should contain exactly 9 lines",
        )));
    }

    for (j, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() != 9 {
            return Err(Box::new(InvalidSudokuFile::new(
                "sudoku file should contain exactly 9 columns",
            )));
        }
        for (i, c) in chars.iter().enumerate() {
            // '.' means 0
            if *c == '.' {
                continue;
            }
            match c.to_digit(10) {
                Some(n) => {
                    // n must be from 1 to 9
                    if n == 0 {
                        return Err(Box::new(InvalidSudokuFile::new(&format!(
                            "invalid character {} in sudoku file",
                            n
                        ))));
                    }
                    sudoku[j * 9 + i] = n as u8;
                }
                None => {
                    return Err(Box::new(InvalidSudokuFile::new(&format!(
                        "invalid character {} in sudoku file",
                        c
                    ))));
                }
            }
        }
    }

    return Ok(sudoku);
}

#[derive(Debug)]
struct InvalidSudokuFile {
    message: String,
}

impl InvalidSudokuFile {
    fn new(message: &str) -> InvalidSudokuFile {
        InvalidSudokuFile {
            message: message.to_string(),
        }
    }
}

impl Error for InvalidSudokuFile {}

impl fmt::Display for InvalidSudokuFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
