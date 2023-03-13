use crate::{BOARD_SIZE, find_solution};
use rand::Rng;

#[derive(Copy, Clone)]
pub enum GameDifficulty {
    Easy = 46,
    Medium = 32,
    Hard = 20
}

pub struct BoardGenerator {
    game_difficulty: GameDifficulty
}

impl BoardGenerator {
    pub fn new(game_difficulty: GameDifficulty) -> BoardGenerator
    {
        return BoardGenerator {
            game_difficulty
        }
    }

    pub fn generate_random_board(
        &self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    )
    {
        for _clue in 0..self.game_difficulty as usize {
            loop {
                let mut random_num_generator = rand::thread_rng();

                let random_column: usize = random_num_generator.gen_range(0..BOARD_SIZE) as usize;
                let random_row: usize = random_num_generator.gen_range(0..BOARD_SIZE) as usize;
                let random_value: usize = random_num_generator.gen_range(0..BOARD_SIZE) as usize;

                sudoku_board[random_column][random_row] = random_value;

                if find_solution(&mut sudoku_board.clone()) {
                    break;
                }
            }
        }
    }
}