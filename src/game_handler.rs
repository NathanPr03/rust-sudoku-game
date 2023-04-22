use std::ops::Div;
use crate::{BoardGenerator, get_trivia_input, pretty_print_board, save, take_user_input_for_cell, UndoHandler, UserInputCommand};
use crate::hint_service::get_hint_command;
use crate::user_input::{get_coordinates_for_hint, get_users_move, get_users_replay_move, get_users_two_player_move};
use crate::Trivia;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use colored::Colorize;
use crate::player::Player;
use crate::util::{calculate_players_score, calculate_timer};
use compound_duration;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameDifficulty {
    //These values are the number percentage of clues present in a board
    VeryEasy = 85,
    Easy = 55,
    Medium = 40,
    Trivia = 39,
    Hard = 25,
    VeryHard = 50
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameHandler
{
    player: Player,
    game_difficulty: GameDifficulty,
    undo_handler: UndoHandler,
    initial_generated_board: Vec<Vec<usize>>,
    board_size: usize,
    time_limit: usize
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
            initial_generated_board: Self::create_initial_board(board_size),
            board_size,
            time_limit: calculate_timer(game_difficulty),
        }
    }

    pub fn play(&mut self) -> usize
    {
        let mut sudoku_board = self.initial_generated_board.clone();

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

        let score = calculate_players_score
        (
            0,
            self.player.get_hints_used(),
            self.player.get_undos_used(),
            self.player.get_redos_used(),
            self.player.game_difficulty,
            self.player.get_trivias_answered()
        );

        return score;
    }

    pub fn multiple_player_setup(&mut self) -> Vec<Vec<usize>>
    {
        let mut sudoku_board = self.initial_generated_board.clone();

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
        sudoku_board: &mut Vec<Vec<usize>>
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
        let mut sudoku_board = self.initial_generated_board.clone();

        self.undo_handler.re_execute_all_commands(&mut sudoku_board);

        pretty_print_board(&sudoku_board);

        self.game_loop(&mut sudoku_board);
    }

    pub fn replay(&mut self)
    {
        let mut sudoku_board = self.initial_generated_board.clone();

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
        sudoku_board: &mut Vec<Vec<usize>>
    ) {
        use std::time::Instant;
        let now = Instant::now();

        while !self.is_game_finished(&sudoku_board)
        {
            let elapsed = now.elapsed();

            let duration = std::time::Duration::from_secs(self.time_limit as u64);
            let time_left = duration.checked_sub(elapsed);

            if time_left.is_none() {
                println!("{}", "🚨🚨🚨 ERROR: Time's up! ⏰⏰⏰ \
                Unfortunately, you were not able to complete the Sudoku game in time. 😞😞😞 \
                Don't worry, there's always next time! 🤞🤞🤞 \
                Keep practicing and you'll get better! 💪💪💪".red());

                return;
            }
            let unwrapped_time = time_left.unwrap();
            // let minutes = unwrapped_time.div(60);  // extract minutes from duration
            // let seconds_left = minutes % 60;

            let formatted_message = format!("Time left: {} minutes and {} seconds", "e", "a");
            println!("{}", formatted_message.purple());

            let users_move = get_users_move();
            match users_move.as_str() {
                "c" => self.change_cell(sudoku_board),
                "u" => self.undo(sudoku_board),
                "r" => self.redo(sudoku_board, true),
                "h" => self.hint(sudoku_board),
                "s" => self.save(),
                "q" => return,
                _ => {}
            }
        }
    }

    fn trivia(&mut self)
    {
        let trivia = Trivia::new();

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

    fn change_cell(&mut self, mut sudoku_board: &mut Vec<Vec<usize>>) {
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

    fn undo(&mut self, sudoku_board: &mut Vec<Vec<usize>>)
    {
        self.undo_handler.undo_last_command(sudoku_board);
        self.player.increment_undos();
        pretty_print_board(&sudoku_board);
    }

    fn redo(&mut self, sudoku_board: &mut Vec<Vec<usize>>, not_replay: bool)
    {
        self.undo_handler.redo_last_command(sudoku_board);
        if not_replay {
            self.player.increment_redos();
            pretty_print_board(&sudoku_board);
        }
    }

    fn hint(&mut self, sudoku_board: &mut Vec<Vec<usize>>)
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

    fn create_initial_board(board_size: usize) -> Vec<Vec<usize>>
    {
        let mut board: Vec<Vec<usize>> = Vec::with_capacity(board_size);
        for _i in 0..board_size {
            let mut row_vec: Vec<usize> = Vec::with_capacity(board_size);
            for _j in 0..board_size {
                row_vec.push(0);
            }
            board.push(row_vec);
        }

        return board;
    }

    fn is_game_finished
    (
        &self,
        sudoku_board: &Vec<Vec<usize>>,
    )-> bool
    {
        for row in sudoku_board {
            if row.contains(&0) {
                return false;
            }
        }

        let winning_message = "🎉🎊👏 Congratulations! 👏🎊🎉 \
        You have successfully completed the Sudoku puzzle! 🧩🎉👍 \
        👨‍💻👩‍💻 Your logic and problem-solving skills are on point! 🤓💪 \
        👏👏👏 Well done! 👏👏👏".green();
        println!("{}", winning_message);

        return true;
    }
}

