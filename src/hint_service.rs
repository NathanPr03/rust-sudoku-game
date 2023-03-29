use crate::{BOARD_SIZE, find_solution, pretty_print_board, UserInputCommand};

pub fn get_hint_command
(
    sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    coordinates: (usize, usize)
)-> UserInputCommand
{
    let (x, y) = coordinates;

    //Dont want to edit the real board
    let mut secondary_board = sudoku_board.clone();
    find_solution(&mut secondary_board);

    let value = &secondary_board[y - 1][x - 1];

    return UserInputCommand::new(x, y, *value);
}