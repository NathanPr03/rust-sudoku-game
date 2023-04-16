use colored::Colorize;
use crate::{BOARD_SIZE, GameDifficulty};

pub fn pretty_print_board_two(sudoku_board: &Vec<Vec<usize>>) {
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

pub fn pretty_print_board(sudoku_board: &Vec<Vec<usize>>) {
    let board_size = sudoku_board.len();
    let sqrt_board_size = (board_size as f32).sqrt() as usize;

    println!("╔═══════╦═══════╦═══════╗");
    for i in 0..board_size {
        if i != 0 && i % sqrt_board_size == 0 {
            println!("╠═══════╬═══════╬═══════╣");
        }
        print!("║ ");
        for j in 0..board_size {
            if j != 0 && j % sqrt_board_size == 0 {
                print!("║ ");
            }
            print!("{} ", sudoku_board[i][j]);
        }
        println!("║ {}", i + 1);
    }
    println!("╚═══════╩═══════╩═══════╝");
    println!("  1 2 3   4 5 6   7 8 9");
}

pub fn calculate_players_score
(
    moves_made: usize,
    hints_used: usize,
    undos_used: usize,
    redos_used: usize,
    game_difficulty: GameDifficulty,
    trivias_answered: usize,
) -> usize
{
    let neg_hint = hints_used * 4;
    let neg_undo = undos_used * 2;
    let neg_redo = redos_used * 2;

    let game_diff_score = match game_difficulty {
        GameDifficulty::Easy => 30,
        GameDifficulty::Trivia => 40,
        GameDifficulty::Medium => 50,
        GameDifficulty::Hard => 90
    };

    let neg_moves = moves_made / 2;

    return game_diff_score + trivias_answered - neg_hint - neg_undo - neg_redo - neg_moves;
}

pub fn check_if_move_is_valid
(
    sudoku_board: &Vec<Vec<usize>>,
    command: (usize, usize, usize)
) -> bool
{
    return check_row_constraint(sudoku_board, command)
        && check_column_constraint(sudoku_board, command)
        && check_region_constraint(sudoku_board, command);
}

fn check_row_constraint
(
    sudoku_board: &Vec<Vec<usize>>,
    command: (usize, usize, usize)
) -> bool {
    let (column, row, value) = command;

    for i in 0..sudoku_board.len() {
        if i == column {
            continue;
        }
        if sudoku_board[row][i] == value && value != 0 {
            println!("{}", "Invalid move, a cell in the same row already has that value".red());
            return false;
        }
    }
    return true;
}

fn check_column_constraint
(
    sudoku_board: &Vec<Vec<usize>>,
    command: (usize, usize, usize)
) -> bool {
    let (column, row, value) = command;

    for i in 0..sudoku_board.len() {
        if i == row {
            continue;
        }
        if sudoku_board[i][column] == value && value != 0 {
            println!("{}", "Invalid move, a cell in the same column already has that value".red());
            return false;
        }
    }
    return true;
}

fn check_region_constraint(
    sudoku_board: &Vec<Vec<usize>>,
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
                println!("{}", "Invalid move, a cell in the same region already has that value".red());
                return false;
            }
        }
    }
    return true;
}
