use std::{
    error::Error,
    fmt::{self, Display},
    fs,
};

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
