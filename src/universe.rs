use std::collections::HashMap;
use crate::{BOARD_SIZE, determine_game_mode, game_handler, GameDifficulty, GameHandler, get_game_mode, get_users_start_game, load, player, pretty_print_board};
use crate::player::Player;
use crate::user_input::{get_multiple_players_name, get_single_players_name};

pub struct Universe {
    players: Vec<Player>,
    game_selected: bool
}

impl Universe {
    pub fn new() -> Universe
    {
        return Universe {
            players: vec![],
            game_selected: false,
        }
    }

    pub fn begin_game(&mut self)
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

        let player = Player::new(users_name);

        let game_diff = determine_game_mode();
        let mut game_handler = GameHandler::new
        (
            player,
            game_diff,
            9
        );

        game_handler.play();

        self.game_selected = true;
    }

    fn load_game(&mut self)
    {
        let mut  game_handler = load();
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
        let num_of_players = 2;
        let player_names = get_multiple_players_name(num_of_players);
        let mut games: HashMap<usize, GameHandler> = HashMap::with_capacity(num_of_players);
        let mut sudoku_boards: HashMap<usize, [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]> = HashMap::with_capacity(num_of_players);
        let game_diff = determine_game_mode();
        let mut both_games_finished = false;

        for i in 0..num_of_players
        {
            let player = Player::new(player_names[i].clone());

            let mut game = GameHandler::new(
                player, game_diff, 9
            );

            let mut sudoku_board = game.multiple_player_setup();
            game.multiple_player_play(&mut sudoku_board);

            sudoku_boards.insert(i, sudoku_board);
            games.insert(i, game);
        }

        while !both_games_finished
        {
            for i in 0..num_of_players {
                let mut sudoku_board = sudoku_boards[&i];
                let mut game = games[&i].clone();

                pretty_print_board(&sudoku_board);
                let game_finished = game.multiple_player_play(&mut sudoku_board);

                games.insert(i, game);

                sudoku_boards.insert(i, sudoku_board);
                both_games_finished = game_finished; // TODO: This is wrong and will need to change
            }
        }
    }
}