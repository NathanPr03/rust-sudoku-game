use rust_sudoku_game::{BoardGenerator, EXACT_COVER_MATRIX_COLUMNS, find_solution, GameDifficulty};
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
            // let mut board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] = [
            //     [5, 3, 0, 0, 7, 0, 0, 0, 0],
            //     [6, 0, 0, 1, 9, 5, 0, 0, 0],
            //     [0, 9, 8, 0, 0, 0, 0, 6, 0],
            //     [8, 0, 0, 0, 6, 0, 0, 0, 3],
            //     [4, 0, 0, 8, 0, 3, 0, 0, 1],
            //     [7, 0, 0, 0, 2, 0, 0, 0, 6],
            //     [0, 6, 0, 0, 0, 0, 2, 8, 0],
            //     [0, 0, 0, 4, 1, 9, 0, 0, 5],
            //     [0, 0, 0, 0, 8, 0, 0, 7, 9],
            // ];

            // let mut board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] =
            //     [[5, 3, 4, 6, 7, 8, 9, 1, 2],
            //     [6, 7, 2, 1, 9, 5, 3, 4, 8],
            //     [1, 9, 8, 3, 4, 2, 5, 6, 0],
            //     [8, 5, 9, 7, 6, 1, 4, 2, 3],
            //     [4, 2, 6, 8, 5, 3, 7, 9, 1],
            //     [7, 1, 3, 9, 2, 4, 8, 5, 6],
            //     [9, 6, 1, 5, 3, 7, 2, 8, 4],
            //     [2, 8, 7, 4, 1, 9, 6, 3, 5],
            //     [3, 4, 5, 2, 8, 6, 1, 7, 9]];

            let mut board = [
                [1, 2, 3, 0, 0, 0, 0, 0, 0],
                [4, 5, 6, 0, 0, 0, 0, 0, 0],
                [7, 8, 0, 0, 0, 0, 0, 0, 9],
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
            find_solution(&mut board);

            // let gd = GameDifficulty::Easy;
            // let bg = BoardGenerator::new(gd);
            //
            // bg.generate_random_board(&mut board);

        })
        .unwrap();

    handler.join().unwrap();
}
