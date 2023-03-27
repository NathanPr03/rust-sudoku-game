use crate::{BOARD_SIZE, UserInputCommand};

pub fn pretty_print_board(sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]) {
    //TODO: Probably remove this and use one defined in lib.rs
    let sqrt_board_size = ((sudoku_board.len() as f32).sqrt()) as usize;
    println!("-------------------------");
    for (i, row) in sudoku_board.iter().enumerate() {
        if i % sqrt_board_size == 0 && i != 0 {
            println!("|-----------------------|");
        }

        for (j, &num) in row.iter().enumerate() {
            if j % sqrt_board_size == 0 {
                print!("| ");
            }
            print!("{} ", num);
        }
        println!("|");
    }
    println!("-------------------------");
}

pub fn check_if_move_is_valid
(
    sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    command: (usize, usize, usize)
) -> bool
{
    return check_row_constraint(sudoku_board, command)
        && check_col_constraint(sudoku_board, command)
        && check_region_constraint(sudoku_board, command);
}

fn check_row_constraint
(
    sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    command: (usize, usize, usize)
) -> bool {
    let (column, row, value) = command;

    for i in 0..sudoku_board.len() {
        if i == column {
            continue;
        }
        if sudoku_board[row][i] == value {
            return false;
        }
    }
    return true;
}

fn check_col_constraint
(
    sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    command: (usize, usize, usize)
) -> bool {
    let (column, row, value) = command;

    for i in 0..sudoku_board.len() {
        if i == row {
            continue;
        }
        if sudoku_board[i][column] == value {
            return false;
        }
    }
    return true;
}

fn check_region_constraint(
    sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    command: (usize, usize, usize)
) -> bool {
    let (column, row, value) = command;

    let sqrt_board_size = sudoku_board.len()/sudoku_board.len();
    let region_row = (row / sqrt_board_size) * sqrt_board_size;
    let region_col = (column / sqrt_board_size) * sqrt_board_size;

    for r in region_row..region_row+sqrt_board_size {
        for c in region_col..region_col+sqrt_board_size {
            if r == row && c == column {
                continue;
            }
            if sudoku_board[r][c] == value {
                return false;
            }
        }
    }
    return true;
}
