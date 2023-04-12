use rust_sudoku_game::{determine_game_mode, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS, GameDifficulty, GameHandler, get_game_mode, load, Universe};
use std::thread::Builder;

fn main() {
    println!(
        "Columns: {}, Rows: {}",
        EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS
    );

    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(64 * 1024 * 1024); // 64MB of stack space

    let handler = builder
        .spawn(|| {
            let mut universe = Universe::new();
            universe.big_bang();
        }).unwrap();

    handler.join().unwrap();
}
