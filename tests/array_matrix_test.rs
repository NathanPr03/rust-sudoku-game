use std::thread::Builder;
use rust_sudoku_game::{ArrayMatrix, BOARD_SIZE, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS, ninebyninecovermatrix};
use crate::complete_nine_by_nine_matrix::completed_nine_by_nine_cover_matrix;

mod complete_nine_by_nine_matrix;

#[test]
pub fn test_nine_by_nine_array_matrix_generated_correctly_for_empty_board()
{
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(32 * 1024 * 1024); // 32MB of stack space

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

        let mut cover_matrix: [[u32; 324]; 729]
            = ninebyninecovermatrix::nine_by_nine_cover_matrix();

        let mut array_matrix = ArrayMatrix::new(cover_matrix);
        array_matrix.create_sparse_matrix(&board, &mut cover_matrix);

        let completed_cover_matrix = completed_nine_by_nine_cover_matrix();
        assert_eq!(&cover_matrix, &completed_cover_matrix);
    }).unwrap();

    handler.join().unwrap();

}