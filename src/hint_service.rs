use crate::{BOARD_SIZE, find_solution, pretty_print_board, UserInputCommand};

pub fn get_hint_command
(
    sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    coordinates: (usize, usize)
)-> Option<UserInputCommand>
{
    let (x, y) = coordinates;

    //Dont want to edit the real board
    let mut secondary_board = sudoku_board.clone();
    let is_possible = find_solution(&mut secondary_board);

    if !is_possible {
        println!("Can not get a hint as this bord isn't solvable. Have a look at your previous moves first ;)");

        return None;
    }
    let value = &secondary_board[y - 1][x - 1];

    return Some(UserInputCommand::new(x, y, *value));
}