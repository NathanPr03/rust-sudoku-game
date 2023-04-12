use crate::{determine_game_mode, GameHandler, get_users_start_game, load};

pub struct Universe {
    game_handler: GameHandler,
    game_selected: bool
}

impl Universe {
    pub fn new(game_handler: GameHandler) -> Universe
    {
        return Universe {
            game_handler,
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
                "h" => println!("New will create a brand new game, \
                load will load the state of a previous game, \
                replay will start the game from scratch with the same starting board."),
                _ => {}
            }
        }

        println!("Thanks for playing!");
    }

    fn new_game(&mut self)
    {
        let game_diff = determine_game_mode();
        self.game_handler.set_game_diff(game_diff);
        self.game_handler.play();

        self.game_selected = true;
    }

    fn load_game(&mut self)
    {
        self.game_handler = load();
        self.game_handler.load();

        self.game_selected = true;
    }

    fn replay_game(&mut self)
    {
        self.game_handler = load();
        self.game_handler.replay();

        self.game_selected = true;
    }
}