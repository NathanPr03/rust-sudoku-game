use rust_sudoku_game::{BOARD_SIZE, BoardGenerator, EXACT_COVER_MATRIX_COLUMNS, find_solution, GameDifficulty};
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
            let mut board = [
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
            // let board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] =[
            //     [3, 2, 1, 4],
            //     [1, 4, 3, 2],
            //     [2, 3, 4, 1],
            //     [4, 1, 2, 0]
            // ];
            // find_solution(&mut board);

            let game_difficulty = GameDifficulty::Hard;
            let board_generator = BoardGenerator::new(game_difficulty);

            board_generator.generate_random_board(&mut board);

        })
        .unwrap();

    handler.join().unwrap();
}
