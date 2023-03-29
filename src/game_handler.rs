use crate::{BOARD_SIZE, BoardGenerator, pretty_print_board, take_user_input_for_cell, UndoHandler, UserInputCommand};
use crate::user_input::get_users_move;

#[derive(Copy, Clone)]
pub enum GameDifficulty {
    //These values are the number of clues that should be present in a 9x9 board
    Easy = 46,
    Medium = 32,
    Hard = 20,
}

pub struct GameHandler
{
    board_generator: BoardGenerator,
    undo_handler: UndoHandler,
    board_size: usize
}

impl GameHandler
{
    pub fn new(game_difficulty: GameDifficulty, board_size: usize) -> GameHandler
    {
        let board_generator = BoardGenerator::new(game_difficulty);
        let undo_handler = UndoHandler::new();

        return GameHandler
        {
            board_generator,
            undo_handler,
            board_size,
        }
    }

    pub fn play(&mut self)
    {
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

        self.board_generator.generate_random_board(&mut sudoku_board);
        pretty_print_board(&sudoku_board);

        while !self.is_game_finished(&sudoku_board)
        {
            let users_move = get_users_move();

            match users_move.as_str() {
                "c" => self.change_cell(&mut sudoku_board),
                "u" => self.undo(&mut sudoku_board),
                "r" => self.redo(&mut sudoku_board),
                _ => {}
            }
        }
    }

    fn change_cell(&mut self, mut sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]) {
        let mut command: Option<UserInputCommand> = None;
        while !command.is_some() {
            command = take_user_input_for_cell(self.board_size);
        }

        let mut unwrapped_command = command.unwrap();
        unwrapped_command.execute(&mut sudoku_board);
        self.undo_handler.push_command(unwrapped_command);
        self.undo_handler.invalidate_redo_stack();

        pretty_print_board(&sudoku_board);
    }

    fn undo(&mut self, sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize])
    {
        self.undo_handler.undo_last_command(sudoku_board);
        pretty_print_board(&sudoku_board);
    }

    fn redo(&mut self, sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize])
    {
        self.undo_handler.redo_last_command(sudoku_board);
        pretty_print_board(&sudoku_board);
    }

    fn is_game_finished
    (
        &self,
        sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    )-> bool
    {
        for row in sudoku_board {
            if row.contains(&0) {
                return false;
            }
        }

        return true;
    }
}

