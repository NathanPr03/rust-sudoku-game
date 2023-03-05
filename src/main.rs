use std::thread::Builder;
use rust_sudoku_game::{BOARD_SIZE, BOARD_SIZE_SQUARED, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS};

use rust_sudoku_game::ArrayMatrix;
use rust_sudoku_game::NodeMatrix;
use rust_sudoku_game::ninebyninecovermatrix;
use rust_sudoku_game::fourbyfourcovermatrix;

fn main() {
    println!("Columns: {}, Rows: {}", EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS);

    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(64 * 1024 * 1024); // 64MB of stack space

    let handler = builder.spawn(|| {
        // let board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] =
        //     [[0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0]];

        let board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] =
            [[5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 0],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9]];
        // Due to the way arrays work in rust its accessed cover_matrix[row_index][column_index]!!
        // let mut cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
        //     = ninebyninecovermatrix::nine_by_nine_cover_matrix();
        //
        // println!("1st: {}", cover_matrix.len());
        // println!("2nd: {}", cover_matrix[1].len());
        //
        // let mut array_matrix = ArrayMatrix::new(cover_matrix);
        // array_matrix.create_sparse_matrix(&board, &mut cover_matrix);
        // ArrayMatrix::print_board(&mut cover_matrix);
        
        
        
        let cover_matrix = [
            [0,0,1,0,1,1,0],
            [1,0,0,1,0,0,1],
            [0,1,1,0,0,1,0],
            [1,0,0,1,0,0,0],
            [0,1,0,0,0,0,1],
            [0,0,0,1,1,0,1]
        ];

        let cover_matrix = [
            [1,1,0,0,1,0,0],
            [0,0,1,0,0,1,0],
            [0,0,1,0,0,1,1],
            [1,0,0,1,0,0,1],
            [0,1,1,0,0,1,0],
            [0,1,0,0,1,0,0]
        ];
        
        let mut nodes_matrix: NodeMatrix = NodeMatrix::new();
        nodes_matrix.arrange_matrix(&cover_matrix);
        nodes_matrix.solve(0);
        nodes_matrix.print_matrix_solution();
    }).unwrap();

    handler.join().unwrap();
}
