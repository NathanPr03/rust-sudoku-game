use std::io::{stdin, stdout, Write};
use rust_sudoku_game::{UserInputCommand, BoardGenerator, EXACT_COVER_MATRIX_COLUMNS, find_solution, GameDifficulty, pretty_print_board, take_user_input_for_cell, GameHandler};
use rust_sudoku_game::EXACT_COVER_MATRIX_ROWS;
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
            let mut game_handler = GameHandler::new(GameDifficulty::Hard, 9);
            game_handler.play();
        })
        .unwrap();

    handler.join().unwrap();
}
