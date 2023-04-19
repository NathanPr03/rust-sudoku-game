use crate::{find_solution, UserInputCommand};
use colored::Colorize;

// Give auto
pub fn get_hint_command
(
    sudoku_board: &mut Vec<Vec<usize>>,
    coordinates: (usize, usize)
)-> Option<UserInputCommand>
{
    let (x, y) = coordinates;

    //Dont want to edit the real board
    let mut secondary_board = sudoku_board.clone();
    let is_possible = find_solution(&mut secondary_board);

    if !is_possible {
        println!("{}", "Can not get a hint as this bord isn't solvable. Have a look at your previous moves first ;)".red());

        return None;
    }
    let value = &secondary_board[y - 1][x - 1];

    return Some(UserInputCommand::new(x, y, *value));
}