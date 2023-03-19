use crate::{find_solution, pretty_print_board};
use rand::Rng;

#[derive(Copy, Clone)]
pub enum GameDifficulty {
    //These values are the percentage of clues that should be present in a Sudoku board
    Easy = 55,
    Medium = 40,
    Hard = 25,
}

pub struct BoardGenerator {
    game_difficulty: GameDifficulty,
    number_of_random_nums_to_insert: usize,
    board_size: usize
}

impl BoardGenerator {
    pub fn new(game_difficulty: GameDifficulty, board_size: usize) -> BoardGenerator
    {
        return BoardGenerator {
            game_difficulty,
            number_of_random_nums_to_insert: 2, //This number seems low but it actually gives us 729 * 721 (525,609) potential boards
            board_size
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

                let random_column: usize = random_num_generator.gen_range(0..self.board_size) as usize;
                let random_row: usize = random_num_generator.gen_range(0..self.board_size) as usize;
                let random_value: usize = random_num_generator.gen_range(0..self.board_size) as usize;

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
        let avoid_rounding = 1000;
        let one_percent = avoid_rounding * (self.board_size * self.board_size) / 100;
        let clues_left = (one_percent * self.game_difficulty as usize) / avoid_rounding;

        let clues_to_remove = self.board_size * self.board_size - clues_left;
        for _clue in 0..(clues_to_remove) {
            loop {
                let mut random_num_generator = rand::thread_rng();

                let random_column: usize = random_num_generator.gen_range(0..self.board_size);
                let random_row: usize = random_num_generator.gen_range(0..self.board_size);

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