use crate::{BOARD_SIZE, BoardGenerator, get_trivia_input, pretty_print_board, save, take_user_input_for_cell, UndoHandler, UserInputCommand};
use crate::hint_service::get_hint_command;
use crate::user_input::{get_coordinates_for_hint, get_users_move, get_users_replay_move, get_users_two_player_move};
use crate::Trivia;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use colored::Colorize;
use crate::player::Player;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameDifficulty {
    //These values are the number of clues that should be present in a 9x9 board
    Easy = 79,
    Medium = 32,
    Trivia = 31,
    Hard = 20,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameHandler
{
    player: Player,
    game_difficulty: GameDifficulty,
    undo_handler: UndoHandler,
    initial_generated_board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    board_size: usize,
}

impl GameHandler
{
    pub fn new
    (
        player: Player,
        game_difficulty: GameDifficulty,
        board_size: usize,
    ) -> GameHandler
    {
        let undo_handler = UndoHandler::new();

        return GameHandler
        {
            player,
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
        let mut sudoku_board = self.initial_generated_board;

        if self.game_difficulty == GameDifficulty::Trivia
        {
            self.trivia();
        }

        let board_generator = BoardGenerator::new(
            self.game_difficulty,
            self.player.get_trivias_answered()
        );
        board_generator.generate_random_board(&mut sudoku_board);
        pretty_print_board(&sudoku_board);

        self.initial_generated_board = sudoku_board.clone();

        self.game_loop(&mut sudoku_board);
    }

    pub fn multiple_player_setup(&mut self) -> [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    {
        let mut sudoku_board = self.initial_generated_board;

        if self.game_difficulty == GameDifficulty::Trivia
        {
            self.trivia();
        }

        let board_generator = BoardGenerator::new(self.game_difficulty, self.player.get_trivias_answered());
        board_generator.generate_random_board(&mut sudoku_board);
        pretty_print_board(&sudoku_board);

        return sudoku_board;
    }

    pub fn multiple_player_play
    (
        &mut self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    ) -> bool
    {
        let player_name = self.player.get_name();
        println!("It is {player_name}'s turn");

        let users_move = get_users_two_player_move(player_name);

        match users_move.as_str() {
            "c" => self.change_cell(sudoku_board),
            "u" => self.undo(sudoku_board),
            "r" => self.redo(sudoku_board, false),
            "h" => self.hint(sudoku_board),
            "p" => return false,
            _ => {}
        }

        return self.is_game_finished(sudoku_board);
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

    pub fn set_game_diff(&mut self, game_difficulty: GameDifficulty)
    {
        self.game_difficulty = game_difficulty;
    }

    pub fn get_player(&self) -> Player
    {
        return self.player.clone();
    }

    fn game_loop
    (
        &mut self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    ) {
        while !self.is_game_finished(&sudoku_board)
        {
            let users_move = get_users_move();

            match users_move.as_str() {
                "c" => self.change_cell(sudoku_board),
                "u" => self.undo(sudoku_board),
                "r" => self.redo(sudoku_board, false),
                "h" => self.hint(sudoku_board),
                "s" => self.save(),
                "q" => return,
                _ => {}
            }
        }
    }

    fn trivia(&mut self)
    {
        let mut trivia = Trivia::new();

        println!("Welcome to the trivia mode! You will be given 10 true or false computer science questions, \
        for every one you answer correctly, you will get an extra clue in your sudoku board");
        for i in 0..10 {
            let (question, answer) = trivia.get_trivia_question_and_answer();

            let users_answer = get_trivia_input(question);

            if answer.eq(&users_answer) {
                println!("{}", "Correct answer!".green());
                self.player.increment_trivias_answered();
            } else{
                println!("{}", format!("Incorrect, the answer was: {answer}, you entered: {users_answer}").red());
            }

            println!("You have {} questions left. So far you have answered {} correctly", 9 - i, self.player.get_trivias_answered());
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

        self.player.increment_moves_made();
        pretty_print_board(&sudoku_board);
    }

    fn undo(&mut self, sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize])
    {
        self.undo_handler.undo_last_command(sudoku_board);
        self.player.increment_undos();
        pretty_print_board(&sudoku_board);
    }

    fn redo(&mut self, sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize], is_replay: bool)
    {
        self.undo_handler.redo_last_command(sudoku_board);
        self.player.increment_redos();
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

        self.player.increment_hints();
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

