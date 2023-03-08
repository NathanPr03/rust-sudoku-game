use std::thread::Builder;
use rust_sudoku_game::{BOARD_SIZE, BOARD_SIZE_SQUARED, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS, four_by_four_cover_matrix};

use rust_sudoku_game::ArrayMatrix;
use rust_sudoku_game::NodeMatrix;
use rust_sudoku_game::ninebyninecovermatrix;
use rust_sudoku_game::fourbyfourcovermatrix;
use rust_sudoku_game::ninebyninecovermatrix::nine_by_nine_cover_matrix;

fn main() {
    println!("Columns: {}, Rows: {}", EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS);

    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(64 * 1024 * 1024); // 64MB of stack space

    let handler = builder.spawn(|| {
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

        // let board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] =
        //     [[5, 3, 4, 6, 7, 8, 9, 1, 2],
        //     [6, 7, 2, 1, 9, 5, 3, 4, 8],
        //     [1, 9, 8, 3, 4, 2, 5, 6, 0],
        //     [8, 5, 9, 7, 6, 1, 4, 2, 3],
        //     [4, 2, 6, 8, 5, 3, 7, 9, 1],
        //     [7, 1, 3, 9, 2, 4, 8, 5, 6],
        //     [9, 6, 1, 5, 3, 7, 2, 8, 4],
        //     [2, 8, 7, 4, 1, 9, 6, 3, 5],
        //     [3, 4, 5, 2, 8, 6, 1, 7, 9]];

        // let board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] =[
        //     [3, 1, 0, 0],
        //     [0, 2, 0, 0],
        //     [0, 4, 0, 0],
        //     [0, 0, 0, 4]
        // ];
        // Due to the way arrays work in rust its accessed cover_matrix[row_index][column_index]!!
        let mut cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
        = nine_by_nine_cover_matrix();
        if BOARD_SIZE == 4 {
            // cover_matrix = four_by_four_cover_matrix();
        }else {
            cover_matrix = nine_by_nine_cover_matrix();
        }


        println!("1st: {}", cover_matrix.len());
        println!("2nd: {}", cover_matrix[1].len());

        let mut array_matrix = ArrayMatrix::new(cover_matrix);
        array_matrix.create_sparse_matrix(&mut cover_matrix, &board);
        ArrayMatrix::print_board(&mut cover_matrix);

        // check_matrix_formed_properly(&cover_matrix);

        //
        // let cover_matrix = [
        //     [0,0,1,0,1,1,0],
        //     [1,0,0,1,0,0,1],
        //     [0,1,1,0,0,1,0],
        //     [1,0,0,1,0,0,0],
        //     [0,1,0,0,0,0,1],
        //     [0,0,0,1,1,0,1]
        // ];
        //
        // let cover_matrix = [
        //     [1,1,0,0,1,0,0],
        //     [0,0,1,0,0,1,0],
        //     [0,0,1,0,0,1,1],
        //     [1,0,0,1,0,0,1],
        //     [0,1,1,0,0,1,0],
        //     [0,1,0,0,1,0,0]
        // ];
        
        let mut nodes_matrix: NodeMatrix = NodeMatrix::new();
        nodes_matrix.arrange_matrix(&cover_matrix);
        nodes_matrix.solve(0);

        // nodes_matrix.print_matrix_solution();
    }).unwrap();

    handler.join().unwrap();
}

fn check_matrix_formed_properly(cover_matrix: &[[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize])
{
    for column in 0..EXACT_COVER_MATRIX_COLUMNS {
        let mut one_found = false;
        for i in 0..cover_matrix.len() {
            println!("{}", cover_matrix[i][column as usize]);
            if cover_matrix[i][column as usize] == 1 {
                one_found = true;
            }
        }

        if !one_found {
            println!("No 1's in column: {}", column);
        }
    }
}
