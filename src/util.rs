use colored::Colorize;
use crate::{GameDifficulty};

pub fn pretty_print_board(sudoku_board: &Vec<Vec<usize>>) {
    let board_size = sudoku_board.len();
    let sqrt_board_size = (board_size as f32).sqrt() as usize;

    let border = "═".repeat(board_size * 3 + sqrt_board_size + sqrt_board_size - 1);
    let mut row_separator = "╠".to_owned();

    for _i in 0..sqrt_board_size-1 {
        row_separator += "═".repeat(sqrt_board_size * 2 + sqrt_board_size + 1).as_str();
        row_separator += "╬";
    }
    row_separator += "═".repeat(sqrt_board_size * 2 + sqrt_board_size + 1).as_str();
    row_separator += "╣";

    let mut column_labels = String::new();
    for i in 1..=board_size {
        column_labels += format!(" {:2}", i).as_str();
        if i % sqrt_board_size == 0 && i != board_size {
            column_labels += "  ";
        }
    }

    println!("╔{}╗", border);
    for i in 0..board_size {
        if i % sqrt_board_size == 0 && i != 0 {
            println!("{}", row_separator);
        }
        let mut row = "║".to_owned();
        for j in 0..board_size {
            if j % sqrt_board_size == 0 && j != 0 {
                row += " ║";
            }
            row += format!(" {:2}", sudoku_board[i][j]).as_str();
        }
        row += format!(" ║{:2}", i + 1).as_str();
        println!("{}", row);
    }
    println!("╚{}╝", border);
    println!("  {}", column_labels);
}

pub fn calculate_timer(game_difficulty: GameDifficulty) -> usize
{
    // Time given to complete a game, units in seconds
    return match game_difficulty {
        GameDifficulty::VeryEasy => 2,
        GameDifficulty::Easy => 1800,
        GameDifficulty::Medium => 1200,
        GameDifficulty::Hard => 600,
        GameDifficulty::VeryHard => 3600,
        GameDifficulty::Trivia => 1200,
    }
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
        GameDifficulty::VeryEasy => 10,
        GameDifficulty::Easy => 30,
        GameDifficulty::Medium => 50,
        GameDifficulty::Hard => 80,
        GameDifficulty::VeryHard => 100,
        GameDifficulty::Trivia => 40,
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
            if sudoku_board[row_iter][column_iter] == value && value != 0 && column-1 != column_iter && row-1 != row_iter
            {
                println!("{}", "Invalid move, a cell in the same region already has that value".red());
                return false;
            }
        }
    }
    return true;
}
