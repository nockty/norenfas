use norenfas::{pretty_print, solve};

fn main() {
    let mut sudoku: [u8; 81] = [
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
    ];
    pretty_print(&sudoku);
    println!("Solving the sudoku...");
    solve(&mut sudoku);
    pretty_print(&sudoku);
}
