use std::io::ErrorKind::BrokenPipe;

mod ninebyninecovermatrix;
mod fourbyfourcovermatrix;

const BOARD_SIZE: u16 = 4;
const BOARD_SIZE_SQUARED: u16 = BOARD_SIZE * BOARD_SIZE;

const CONSTRAINTS: [&str; 4] = ["Position", "Row", "Column", "Square"];
const NUM_OF_CONSTRAINTS: u16 = CONSTRAINTS.len() as u16;
const EXACT_COVER_MATRIX_COLUMNS: u16 = BOARD_SIZE_SQUARED * NUM_OF_CONSTRAINTS;
const EXACT_COVER_MATRIX_ROWS: u16 = BOARD_SIZE_SQUARED * BOARD_SIZE;

fn main() {

    println!("Columns: {}, Rows: {}", EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS);

    // let _board: [[u8; BOARD_SIZE as usize]; BOARD_SIZE as usize] =
    // [
    //     [0, 3, 4, 0],
    //     [4, 0, 0, 2],
    //     [1, 0, 0, 3],
    //     [0, 2, 1, 0]
    // ];

    //Due to the way arrays work in rust its accessed cover_matrix[row_index][column_index]!!
    let mut cover_matrix:[[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
        = fourbyfourcovermatrix::four_by_four_cover_matrix();

    cover_matrix = cell_constraint(cover_matrix);
    cover_matrix = row_constraint(cover_matrix);
    cover_matrix = column_constraint(cover_matrix);

    println!("---------------------------------------------------------------------");
    for i in 0..cover_matrix.len() {
        for j in 0..cover_matrix[1].len() {
            if j % (BOARD_SIZE_SQUARED) as usize == 0 {
                print!("|");
            }
            print!("{}", cover_matrix[i][j]);
        }
        print!("|");
        println!();
        println!("---------------------------------------------------------------------")
    }

}
fn cell_constraint
(
    mut cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
) -> [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
{
    let mut row_index: u16 = BOARD_SIZE;

    for column in 0..BOARD_SIZE_SQUARED {
        for row in row_index - BOARD_SIZE..row_index {
            cover_matrix[row as usize][column as usize] = 1;
        }
        row_index = row_index + BOARD_SIZE;
    }

    return cover_matrix;
}

fn row_constraint
(
    mut cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
) -> [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
{
    let mut pullback: u16 = BOARD_SIZE_SQUARED;

    let mut column: u16 = BOARD_SIZE_SQUARED;

    for row in 0..EXACT_COVER_MATRIX_ROWS {
        if row % BOARD_SIZE_SQUARED == 0 && row > 1 {
            pullback += BOARD_SIZE
        }

        if column % BOARD_SIZE == 0 {
            column = pullback
        }

        cover_matrix[row as usize][column as usize] = 1;
        column += 1;
    }

    return cover_matrix;
}

fn column_constraint
(
    mut cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
) -> [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
{
    let board_size_cubed: u16 = (BOARD_SIZE * BOARD_SIZE) + (BOARD_SIZE * BOARD_SIZE);

    let mut column: u16 = board_size_cubed;
    for row in 0..EXACT_COVER_MATRIX_ROWS {
        if row % BOARD_SIZE_SQUARED == 0 && row > 1{
            column = board_size_cubed;
        }
        cover_matrix[row as usize][column as usize] = 1;

        column += 1;
    }
    return cover_matrix
}

fn region_constraint
(
    mut cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
) -> [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
{
    return cover_matrix
}