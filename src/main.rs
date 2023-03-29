use std::io::{stdin, stdout, Write};
use rust_sudoku_game::{UserInputCommand, BoardGenerator, EXACT_COVER_MATRIX_COLUMNS, find_solution, GameDifficulty, pretty_print_board, take_user_input_for_cell, GameHandler, Trivia, get_trivia_input, determine_game_mode};
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
            let game_mode = determine_game_mode();

            let mut game_handler = GameHandler::new(game_mode, 9);

            game_handler.play();
        }).unwrap();

    handler.join().unwrap();
}
