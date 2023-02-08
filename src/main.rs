use rust_sudoku_game::{BOARD_SIZE, BOARD_SIZE_SQUARED, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS};

use rust_sudoku_game::ArrayMatrix;
use rust_sudoku_game::NodeMatrix;
use rust_sudoku_game::ninebyninecovermatrix;
use rust_sudoku_game::fourbyfourcovermatrix;

fn main() {
    println!("Columns: {}, Rows: {}", EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS);

    // let board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] =
    //     [[0, 0, 0, 0, 0, 0, 0, 5, 0],
    //     [2, 0, 7, 0, 0, 9, 0, 0, 0],
    //     [6, 0, 0, 3, 5, 1, 0, 0, 0],
    //     [5, 0, 0, 0, 0, 0, 0, 1, 0],
    //     [0, 0, 3, 0, 0, 0, 0, 0, 8],
    //     [0, 0, 0, 8, 2, 0, 5, 3, 0],
    //     [0, 0, 0, 0, 7, 0, 8, 0, 4],
    //     [0, 0, 6, 2, 0, 0, 0, 0, 0],
    //     [0, 8, 0, 0, 0, 0, 7, 0, 0]];
    let board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] =
        [[0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0]];

    //Due to the way arrays work in rust its accessed cover_matrix[row_index][column_index]!!
    let mut cover_matrix:[[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
        = ninebyninecovermatrix::nine_by_nine_cover_matrix();

    println!("1st: {}", cover_matrix.len());
    println!("2nd: {}", cover_matrix[1].len());

    let mut array_matrix = ArrayMatrix::new(cover_matrix);
    // array_matrix.create_sparse_matrix(&board, &mut cover_matrix);
    // ArrayMatrix::print_board(&mut cover_matrix);
    array_matrix.generate_array_matrix();
    ArrayMatrix::print_board(array_matrix.get_cover_matrix());
    // array_matrix.generate_array_matrix();
    //
    // let mut nodes_matrix: NodeMatrix = NodeMatrix::new();
    // nodes_matrix.arrange_matrix(array_matrix.get_cover_matrix());
    //
    // let headers = nodes_matrix.get_column_nodes();
    // let first_header = &headers[1];
    // dbg!(first_header.borrow_mut().down.upgrade().unwrap());
    // array_matrix.print_board();
    // clue_to_exact_cover(&_board, &mut cover_matrix);

}
