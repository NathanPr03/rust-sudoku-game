use std::thread::Builder;
use rust_sudoku_game::{BoardGenerator, GameDifficulty};

#[test]
pub fn test_board_generation_is_random()
{
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(64 * 1024 * 1024); // 64MB of stack space

    let handler = builder.spawn(|| {
        let mut sudoku_boards: Vec<Vec<Vec<usize>>> = Vec::new();

        for _i in 0..10 {
            let mut sudoku_board = vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ];

            let board_size = 9;
            let game_difficulty = GameDifficulty::Hard;
            let board_generator = BoardGenerator::new(game_difficulty, board_size);

            board_generator.generate_random_board(&mut sudoku_board);
            sudoku_boards.push(sudoku_board);
        }
        for _i in 0..sudoku_boards.len()
        {
            let current_board = sudoku_boards.pop().unwrap();

            for sudoku_board in sudoku_boards.clone() {
                assert_ne!(sudoku_board, current_board);
            }
        }
    }).unwrap();

    handler.join().unwrap();
}