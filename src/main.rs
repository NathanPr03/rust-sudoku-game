use std::io::{stdin, stdout, Write};
use rust_sudoku_game::{BOARD_SIZE, BoardGenerator, EXACT_COVER_MATRIX_COLUMNS, find_solution, GameDifficulty, pretty_print_board};
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
            let mut sudoku_board = Vec::new();

            print!("Please enter some text: ");
            let _=stdout().flush();
            let mut board_size_raw = String::new();

            stdin().read_line(&mut board_size_raw).expect("failed to readline");

            let real_string = board_size_raw.to_string().replace("\n", "");  // `parse()` works with `&str` and `String`!
            let board_size = real_string.parse::<i32>().unwrap();

            for _i in 0..board_size {
                sudoku_board.push([0, 0, 0, 0, 0, 0, 0, 0, 0]);
            }

            let d = 2;

            // let board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] =[
            //     [3, 2, 1, 4],
            //     [1, 4, 3, 2],
            //     [2, 3, 4, 1],
            //     [4, 1, 2, 0]
            // ];
            // find_solution(&mut board);

            let game_difficulty = GameDifficulty::Hard;
            let board_generator = BoardGenerator::new(game_difficulty);

            board_generator.generate_random_board(&mut sudoku_board);
            pretty_print_board(&sudoku_board);

        })
        .unwrap();

    handler.join().unwrap();
}
