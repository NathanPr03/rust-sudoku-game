use rust_sudoku_game::{Universe};
use std::thread::Builder;
use colored::Colorize;

fn main() {
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(64 * 1024 * 1024); // 64MB of stack space

    let handler = builder
        .spawn(|| {
            println!("{}", "Welcome to SudoGenius, a challenging and addictive sudoku game! \
            The objective of the game is to fill a 9x9 grid with numbers from 1 to 9 (and if you fancy a challenge there are other board sizes and modes!), \
            making sure that each row, column, and 3x3 sub-grid contains all the numbers from 1 to 9 without any repetition. \
            The game starts with some numbers already filled in, and your task is to fill in the remaining numbers to complete the grid.\
             With multiple levels of difficulty, SudoGenius is a great way to exercise your brain and improve your problem-solving skills.\
              Good luck and have fun!".bright_magenta());
            let mut universe = Universe::new();
            universe.big_bang();
        }).unwrap();

    handler.join().unwrap();
}
