use std::time::Instant;
use std::collections::HashMap;
use colored::Colorize;
use crate::player::Player;
use crate::{determine_game_mode, GameDifficulty, GameHandler, get_users_start_game, load, pretty_print_board};
use crate::user_input::{get_multiple_players_name, get_number_of_players, get_single_players_name};
use crate::util::calculate_players_score;

pub struct Universe {
    players: Vec<Player>,
    game_selected: bool
}

impl Universe {
    pub fn new() -> Universe
    {
        return Universe {
            players: Vec::new(),
            game_selected: false,
        }
    }

    pub fn big_bang(&mut self)
    {
        while self.game_selected == false {
            let users_action = get_users_start_game();

            match users_action.as_str() {
                "n" => self.new_game(),
                "l" => self.load_game(),
                "r" => self.replay_game(),
                "m" => self.multiple_player_game(),
                _ => {}
            }
        }
        println!("Thanks for playing!");
    }

    fn new_game(&mut self)
    {
        let users_name = get_single_players_name();

        let game_diff = determine_game_mode();
        let player = Player::new(users_name, game_diff);
        self.players.push(player.clone());

        let mut board_size = 9;
        if game_diff == GameDifficulty::VeryHard
        {
            board_size = 16;
        }else if game_diff == GameDifficulty::VeryEasy {
            board_size = 4;
        }

        let mut game_handler = GameHandler::new
        (
            player,
            game_diff,
            board_size
        );

        game_handler.play();

        self.game_selected = true;
    }

    fn load_game(&mut self)
    {
        let mut game_handler = load();
        game_handler.load();

        self.game_selected = true;
    }

    fn replay_game(&mut self)
    {
        let mut game_handler = load();
        game_handler.replay();

        self.game_selected = true;
    }

    fn multiple_player_game(&mut self)
    {
        let num_of_players = get_number_of_players();
        let player_names = get_multiple_players_name(num_of_players);
        let game_diff = determine_game_mode();

        let mut games: HashMap<usize, GameHandler> = HashMap::with_capacity(num_of_players);
        let mut sudoku_boards: HashMap<usize, Vec<Vec<usize>>> = HashMap::with_capacity(num_of_players);

        for i in 0..num_of_players
        {
            let player = Player::new(player_names[i].clone(), game_diff);

            let mut game = GameHandler::new(
                player, game_diff, 9
            );

            let mut sudoku_board = game.multiple_player_setup();
            game.multiple_player_play(&mut sudoku_board);

            sudoku_boards.insert(i, sudoku_board);
            games.insert(i, game);
        }

        let mut finished_count = 0;

        while finished_count != num_of_players
        {
            for i in 0..num_of_players {
                let mut sudoku_board = sudoku_boards[&i].clone();
                let mut game = games[&i].clone();

                pretty_print_board(&sudoku_board);

                let game_finished = game.multiple_player_play(&mut sudoku_board);

                games.insert(i, game);

                sudoku_boards.insert(i, sudoku_board);
                if game_finished
                {
                    finished_count += 1;
                }
            }
        }

        for game in games.values()
        {
            self.players.push(game.get_player());
        }

        self.calculate_winner();
        self.game_selected = true;
    }

    fn calculate_winner(&self) -> ()
    {
        let mut scores: Vec<usize> = Vec::new();

        for player in self.players.clone()
        {
            let score = calculate_players_score
            (
                0,
                player.get_hints_used(),
                player.get_undos_used(),
                player.get_redos_used(),
                player.game_difficulty,
            player.get_trivias_answered()
            );

            scores.push(score);
        }

        let mut max_score = 0;
        let mut index_of_max_score = 0;
        for i in 0..scores.len()
        {
            let score = scores[i];
            if score > max_score
            {
                max_score = score;
                index_of_max_score = i;
            }
        }
        let success_message =
            format!("Congratulations {}, you had the highest score of {max_score}", self.players[index_of_max_score].get_name()).green();
        println!("{success_message}");
    }
}