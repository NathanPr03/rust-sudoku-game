use crate::{BOARD_SIZE, BOARD_SIZE_SQUARED, find_solution, pretty_print_board};
use rand::Rng;

#[derive(Copy, Clone)]
pub enum GameDifficulty {
    //These values are the number of clues that should be present in a 9x9 board
    Easy = 46,
    Medium = 32,
    Hard = 20,
}

pub struct BoardGenerator {
    game_difficulty: GameDifficulty,
    number_of_random_nums_to_insert: usize,
}

impl BoardGenerator {
    pub fn new(game_difficulty: GameDifficulty) -> BoardGenerator
    {
        return BoardGenerator {
            game_difficulty,
            number_of_random_nums_to_insert: 2, //This number seems low but it actually gives us 729 * 721 (525,609) potential boards
        };
    }

    pub fn generate_random_board(
        &self,
        sudoku_board: &mut Vec<Vec<usize>>,
    )
    {
        // This code is extremely performant :)
        use std::time::Instant;
        let now = Instant::now();

        for i in 0..self.number_of_random_nums_to_insert as usize {
            loop {
                let mut random_num_generator = rand::thread_rng();

                let random_column: usize = random_num_generator.gen_range(0..BOARD_SIZE) as usize;
                let random_row: usize = random_num_generator.gen_range(0..BOARD_SIZE) as usize;
                let random_value: usize = random_num_generator.gen_range(0..BOARD_SIZE) as usize;

                //  If the cell has already been filled we dont want to fill it again, will fuck up the matrix
                if sudoku_board[random_column][random_row] != 0
                {
                    continue;
                }

                sudoku_board[random_column][random_row] = random_value;

                // If we are on the first iteration of the loop we dont have to check if the board is still solvable
                if i == 0
                {
                    break;
                }

                // If no solution is found reset cell
                if !find_solution(sudoku_board) {
                    sudoku_board[random_column][random_row] = 0;
                }

                break;
            }
        }

        self.remove_given_numbers_from_sudoku(sudoku_board);

        let elapsed = now.elapsed();
        println!("Board generated in: {:.2?}", elapsed);
    }

    fn remove_given_numbers_from_sudoku(&self, sudoku_board: &mut Vec<Vec<usize>>)
    {
        for _clue in 0..(BOARD_SIZE_SQUARED as usize - self.game_difficulty as usize) {
            loop {
                let mut random_num_generator = rand::thread_rng();

                let random_column: usize = random_num_generator.gen_range(0..BOARD_SIZE) as usize;
                let random_row: usize = random_num_generator.gen_range(0..BOARD_SIZE) as usize;

                if sudoku_board[random_column][random_row] == 0
                {
                    continue;
                }

                sudoku_board[random_column][random_row] = 0;

                break;
            }
        }
    }
}