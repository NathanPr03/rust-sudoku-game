use rust_sudoku_game::{Universe};
use std::thread::Builder;

fn main() {
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(64 * 1024 * 1024); // 64MB of stack space

    let handler = builder
        .spawn(|| {
            let mut universe = Universe::new();
            universe.big_bang();
        }).unwrap();

    handler.join().unwrap();
}
