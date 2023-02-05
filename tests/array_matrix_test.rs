use std::os::unix::thread;
use std::thread::Builder;
use rust_sudoku_game::{ArrayMatrix, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS, ninebyninecovermatrix};
use crate::complete_nine_by_nine_matrix::completed_nine_by_nine_cover_matrix;

mod complete_nine_by_nine_matrix;

#[test]
pub fn test_nine_by_nine_array_matrix_generated_correctly()
{
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(32 * 1024 * 1024); // 32MB of stack space

    let handler = builder.spawn(|| {
        let cover_matrix: [[u32; 324]; 729]
            = ninebyninecovermatrix::nine_by_nine_cover_matrix();

        let mut array_matrix = ArrayMatrix::new(cover_matrix);
        array_matrix.generate_array_matrix();

        let completed_cover_matrix = completed_nine_by_nine_cover_matrix();
        assert_eq!(array_matrix.get_cover_matrix(), &completed_cover_matrix);
    }).unwrap();

    handler.join().unwrap();

}