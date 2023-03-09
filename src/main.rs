use core::panic;
use std::{env, fs};

use norenfas::{
    io::{parse_from_file, pretty_print},
    solve,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("missing argument file_path");
    }
    let file_path = &args[1];
    let mut sudoku = match parse_from_file(file_path) {
        Err(e) => {
            panic!("error when parsing file {}: {}", file_path, e);
        }
        Ok(sudoku) => sudoku,
    };
    pretty_print(&sudoku);
    println!("Solving the sudoku...");
    solve(&mut sudoku);
    pretty_print(&sudoku);
}
