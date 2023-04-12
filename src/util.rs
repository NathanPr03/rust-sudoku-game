use crate::{BOARD_SIZE};

pub fn pretty_print_board_two(sudoku_board: &[[usize; 9]; 9]) {
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

pub fn pretty_print_board(sudoku_board: &[[usize; 9]; 9]) {
    println!("╔═══════╦═══════╦═══════╗");
    for i in 0..9 {
        if i != 0 && i % 3 == 0 {
            println!("╠═══════╬═══════╬═══════╣");
        }
        print!("║ ");
        for j in 0..9 {
            if j != 0 && j % 3 == 0 {
                print!("║ ");
            }
            print!("{} ", sudoku_board[i][j]);
        }
        println!("║ {}", i + 1);
    }
    println!("╚═══════╩═══════╩═══════╝");
    println!("  1 2 3   4 5 6   7 8 9");
}

pub fn check_if_move_is_valid
(
    sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    command: (usize, usize, usize)
) -> bool
{
    return check_row_constraint(sudoku_board, command)
        && check_column_constraint(sudoku_board, command)
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
        if sudoku_board[row][i] == value && value != 0 {
            println!("Invalid move, a cell in the same row already has that value");
            return false;
        }
    }
    return true;
}

fn check_column_constraint
(
    sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    command: (usize, usize, usize)
) -> bool {
    let (column, row, value) = command;

    for i in 0..sudoku_board.len() {
        if i == row {
            continue;
        }
        if sudoku_board[i][column] == value && value != 0 {
            println!("Invalid move, a cell in the same column already has that value");
            return false;
        }
    }
    return true;
}

fn check_region_constraint(
    sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    command: (usize, usize, usize)
) -> bool {
    let (mut column, mut row, value) = command;

    let sqrt_board_size = (sudoku_board.len() as f32).sqrt() as usize;

    // Handle 0 indexing
    column = column + 1;
    row = row + 1;

    let x_coord_top_left_of_region = column - ((column - 1) % sqrt_board_size) - 1;
    let y_coord_top_left_of_region = row - ((row - 1) % sqrt_board_size) - 1;

    for column_iter in x_coord_top_left_of_region..x_coord_top_left_of_region + sqrt_board_size
    {
        for row_iter in y_coord_top_left_of_region..y_coord_top_left_of_region + sqrt_board_size
        {
            if sudoku_board[row_iter][column_iter] == value && value != 0
            {
                println!("Invalid move, a cell in the same region already has that value");
                return false;
            }
        }
    }
    return true;
}
