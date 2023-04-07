use rust_sudoku_game::{determine_game_mode, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS, GameHandler, load};
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
            // let game_mode = determine_game_mode();

            // let mut game_handler = GameHandler::new(game_mode, 9);
            //
            // game_handler.play();

            let mut gh = load();
            gh.play();
        }).unwrap();

    handler.join().unwrap();
}
