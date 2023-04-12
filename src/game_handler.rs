use crate::{BOARD_SIZE, BoardGenerator, get_trivia_input, pretty_print_board, save, take_user_input_for_cell, UndoHandler, UserInputCommand};
use crate::hint_service::get_hint_command;
use crate::user_input::{get_coordinates_for_hint, get_users_move, get_users_replay_move};
use crate::Trivia;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use colored::Colorize;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameDifficulty {
    //These values are the number of clues that should be present in a 9x9 board
    Easy = 46,
    Medium = 32,
    Hard = 20,
    Trivia = 31
}

#[derive(Serialize, Deserialize)]
pub struct GameHandler
{
    game_difficulty: GameDifficulty,
    undo_handler: UndoHandler,
    initial_generated_board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    board_size: usize,
}

impl GameHandler
{
    pub fn new(game_difficulty: GameDifficulty, board_size: usize) -> GameHandler
    {
        let undo_handler = UndoHandler::new();

        return GameHandler
        {
            game_difficulty,
            undo_handler,
            initial_generated_board: [
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
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

        let mut trivia_addition = 0;
        if self.game_difficulty == GameDifficulty::Trivia
        {
            trivia_addition = self.trivia();
        }

        let board_generator = BoardGenerator::new(self.game_difficulty, trivia_addition);
        board_generator.generate_random_board(&mut sudoku_board);
        pretty_print_board(&sudoku_board);

        self.initial_generated_board = sudoku_board.clone();

        self.game_loop(&mut sudoku_board);
    }

    pub fn set_game_diff(&mut self, game_difficulty: GameDifficulty)
    {
        self.game_difficulty = game_difficulty;
    }

    pub fn load(&mut self)
    {
        let mut sudoku_board = self.initial_generated_board;

        self.undo_handler.re_execute_all_commands(&mut sudoku_board);

        pretty_print_board(&sudoku_board);

        self.game_loop(&mut sudoku_board);
    }

    pub fn replay(&mut self)
    {
        let mut sudoku_board = self.initial_generated_board;

        pretty_print_board(&sudoku_board);
        self.undo_handler.invalidate_redo_stack();

        while !self.is_game_finished(&sudoku_board)
        {
            let users_move = get_users_replay_move();

            match users_move.as_str() {
                "c" => self.undo_handler.redo_last_command_reverse(&mut sudoku_board),
                "u" => self.undo_handler.undo_last_command_reverse(&mut sudoku_board),
                "i" => self.game_loop(&mut sudoku_board),
                "q" => return,
                _ => {}
            }

            pretty_print_board(&sudoku_board);
        }
    }

    fn game_loop(&mut self, mut sudoku_board: &mut [[usize; 9]; 9]) {
        while !self.is_game_finished(&sudoku_board)
        {
            let users_move = get_users_move();

            match users_move.as_str() {
                "c" => self.change_cell(&mut sudoku_board),
                "u" => self.undo(&mut sudoku_board),
                "r" => self.redo(&mut sudoku_board, false),
                "h" => self.hint(&mut sudoku_board),
                "s" => self.save(),
                "q" => return,
                _ => {}
            }
        }
    }

    fn trivia(&self) -> usize
    {
        let mut trivia = Trivia::new();

        println!("Welcome to the trivia mode! You will be given 10 true or false computer science questions, \
        for every one you answer correctly, you will get an extra clue in your sudoku board");
        for i in 0..10 {
            let (question, answer) = trivia.get_trivia_question_and_answer();

            let users_answer = get_trivia_input(question);

            if answer.eq(&users_answer) {
                println!("{}", "Correct answer!".green());
                trivia.increment_correct_answers();
            } else{
                println!("{}", format!("Incorrect, the answer was: {answer}, you entered: {users_answer}").green());
            }

            println!("You have {} questions left. So far you have answered {} correctly", 9 - i, trivia.get_correct_answers());
        }

        return trivia.get_correct_answers();
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

        let x = unwrapped_command.get_x_coordinate();
        let y = unwrapped_command.get_y_coordinate();

        let success_message = format!("Successfully edited coordinates {x},{y}").green();
        println!("{}", success_message);
        pretty_print_board(&sudoku_board);
    }

    fn undo(&mut self, sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize])
    {
        self.undo_handler.undo_last_command(sudoku_board);
        pretty_print_board(&sudoku_board);
    }

    fn redo(&mut self, sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize], is_replay: bool)
    {
        self.undo_handler.redo_last_command(sudoku_board);

        if !is_replay {
            pretty_print_board(&sudoku_board);
        }
    }

    fn hint(&mut self, sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize])
    {
        let (x, y) = get_coordinates_for_hint(sudoku_board.len());
        let command = get_hint_command(sudoku_board, (x, y));

        if !command.is_some()
        {
            return;
        }

        let mut unwrapped_command = command.unwrap();
        unwrapped_command.execute(sudoku_board);

        self.undo_handler.push_command(unwrapped_command);
        self.undo_handler.invalidate_redo_stack();

        let success_message = format!("Hint successfully applied to coordinates {x},{y}").green();
        println!("{}", success_message);

        pretty_print_board(sudoku_board);

    }

    fn save(&self)
    {
        save(self);

        let success_message = "Game saved successfully".green();
        println!("{}", success_message);
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

        let winning_message = "CONGRATULATIONS! You have completed the sudoku, why not try a different game mode?".green();
        println!("{}", winning_message);

        return true;
    }
}

