use std::io::{stdin, stdout, Write};
use rust_sudoku_game::{BoardGenerator, find_solution, GameDifficulty, pretty_print_board};
use std::thread::Builder;

fn main() {
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(64 * 1024 * 1024); // 64MB of stack space

    let handler = builder
        .spawn(|| {
            let mut sudoku_board: Vec<Vec<usize>> = Vec::new();

            print!("Please enter some text: ");
            let _=stdout().flush();
            let mut board_size_raw = String::new();

            stdin().read_line(&mut board_size_raw).expect("failed to readline");

            let real_string = board_size_raw.to_string().replace("\n", "");
            let board_size: usize = real_string.parse::<i32>().unwrap() as usize;

            for _i in 0..board_size {
                let mut one_row = Vec::new();
                for _j in 0..board_size {
                    one_row.push(0);
                }
                sudoku_board.push(one_row);
            }

            let game_difficulty = GameDifficulty::Hard;
            let board_generator = BoardGenerator::new(game_difficulty, board_size);

            board_generator.generate_random_board(&mut sudoku_board);
            print!("here");
            pretty_print_board(&sudoku_board);

        })
        .unwrap();

    handler.join().unwrap();
}
