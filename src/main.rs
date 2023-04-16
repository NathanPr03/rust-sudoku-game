use rust_sudoku_game::{GameDifficulty, GameHandler, Player, Universe};
use std::thread::Builder;

fn main() {
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(64 * 1024 * 1024); // 64MB of stack space

    let handler = builder
        .spawn(|| {
            let mut universe = Universe::new();
            universe.big_bang();
            //
            // let pl = Player::new("nat".to_string(), GameDifficulty::Medium);
            // let mut gh = GameHandler::new(pl, GameDifficulty::Hard, 16);
            // gh.play();
        }).unwrap();

    handler.join().unwrap();
}
