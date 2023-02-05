use rust_sudoku_game::{EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS};

use rust_sudoku_game::ArrayMatrix;
use rust_sudoku_game::NodeMatrix;
use rust_sudoku_game::ninebyninecovermatrix;
use rust_sudoku_game::fourbyfourcovermatrix;

fn main() {
    println!("Columns: {}, Rows: {}", EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS);


    //Due to the way arrays work in rust its accessed cover_matrix[row_index][column_index]!!
    let cover_matrix:[[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
        = ninebyninecovermatrix::nine_by_nine_cover_matrix();

    println!("1st: {}", cover_matrix.len());
    println!("2nd: {}", cover_matrix[1].len());

    let mut array_matrix = ArrayMatrix::new(cover_matrix);
    array_matrix.generate_array_matrix();

    let mut nodes_matrix: NodeMatrix = NodeMatrix::new();
    nodes_matrix.arrange_matrix(array_matrix.get_cover_matrix());

    let headers = nodes_matrix.get_column_nodes();
    let first_header = &headers[1];
    dbg!(first_header.borrow_mut().down.upgrade().unwrap());
    array_matrix.print_board();
    // clue_to_exact_cover(&_board, &mut cover_matrix);
    // print_board(&mut cover_matrix);
}

fn board_cell_to_exact_cover_row(board_row: usize, board_column: usize, cell_value: u8) -> usize
{
    let mut exact_cover_row: usize = 0;
    if board_row > 1 {
        exact_cover_row += (board_row - 1) * 16
    }
    if board_column > 1 {
        exact_cover_row += (board_column - 1) * 4
    }
    exact_cover_row += cell_value as usize;

    return exact_cover_row;
}