use std::io::{stdin, stdout, Write};
§use rust_sudoku_game::{UserInputCommand, BoardGenerator, EXACT_COVER_MATRIX_COLUMNS, find_solution, GameDifficulty, pretty_print_board, take_user_input_for_cell};
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
            let mut sudoku_board = [
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ];
            let game_difficulty = GameDifficulty::Hard;
            let board_generator = BoardGenerator::new(game_difficulty);

            board_generator.generate_random_board(&mut sudoku_board);
            pretty_print_board(&sudoku_board);

            let mut command: Option<UserInputCommand> = None;
            while !command.is_some() {
                command = take_user_input_for_cell(9);
            }

            let mut unwrapped_command = command.unwrap();
            unwrapped_command.execute(&mut sudoku_board);
            pretty_print_board(&sudoku_board);

            dbg!(unwrapped_command);


            unwrapped_command.undo(&mut sudoku_board);
            pretty_print_board(&sudoku_board);
        })
        .unwrap();

    handler.join().unwrap();
}
