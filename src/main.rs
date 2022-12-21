mod ninebyninecovermatrix;
mod fourbyfourcovermatrix;

const BOARD_SIZE: u16 = 9;

const CONSTRAINTS: [&str; 4] = ["Position", "Row", "Column", "Square"];
const NUM_OF_CONSTRAINTS: u16 = CONSTRAINTS.len() as u16;
const EXACT_COVER_MATRIX_COLUMNS: u16 = BOARD_SIZE * BOARD_SIZE * NUM_OF_CONSTRAINTS;
const EXACT_COVER_MATRIX_ROWS: u16 = BOARD_SIZE * BOARD_SIZE * BOARD_SIZE;

fn main() {

    println!("Columns: {}, Rows: {}", EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS);

    // let _board: [[u8; BOARD_SIZE as usize]; BOARD_SIZE as usize] =
    // [
    //     [0, 3, 4, 0],
    //     [4, 0, 0, 2],
    //     [1, 0, 0, 3],
    //     [0, 2, 1, 0]
    // ];

    let mut cover_matrix:[[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
        = ninebyninecovermatrix::nine_by_nine_cover_matrix();

    cover_matrix = cell_constraint(cover_matrix);
}

// number in the following function is the number that would be going into a..
// ..given box in the sudoku. In a 9x9 sudoku this could be 1..9
// I THINK
fn _index_in_cover_matrix(row: u16, column: u16, number: u16) -> u32 {
    const SIZE: u16 = 4;
    // What is this maths?
    let row_index: u16 = (row - 1) * SIZE * SIZE;
    let column_index: u16 = (column - 1) * SIZE;
    let num_index: u16 = number - 1;

    return (row_index + column_index + num_index) as u32;
}

fn cell_constraint
(
    mut cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
) -> [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
{
    let mut row_index: u16 = BOARD_SIZE;

    for column in 0..BOARD_SIZE * BOARD_SIZE {
        for row in row_index - BOARD_SIZE..row_index {
            cover_matrix[row as usize][column as usize] = 1;
        }
        row_index = row_index + BOARD_SIZE;
    }

    for i in 0..cover_matrix.len() {
        for j in 0..cover_matrix[1].len() {
            print!("{}", cover_matrix[i][j]);
        }
        println!();
        println!("----------------------------------------------------------------")
    }

    return cover_matrix;
}