use crate::{ArrayMatrix, BOARD_SIZE, BOARD_SIZE_SQUARED, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS, NodeMatrix, StrongNode};
use crate::ninebyninecovermatrix::nine_by_nine_cover_matrix;

pub fn find_solution
(
    sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
) {
    use std::time::Instant;
    let now = Instant::now();

    // Due to the way arrays work in rust its accessed cover_matrix[row_index][column_index]!!
    let mut cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
        = nine_by_nine_cover_matrix();
    if BOARD_SIZE == 4 {
        // = four_by_four_cover_matrix();
    } else {
        // = nine_by_nine_cover_matrix();
    }

    let mut array_matrix = ArrayMatrix::new(cover_matrix);
    array_matrix.create_sparse_matrix(&mut cover_matrix, &sudoku_board);
    ArrayMatrix::print_board(&mut cover_matrix);

    check_matrix_formed_properly(&cover_matrix);

    let mut nodes_matrix: NodeMatrix = NodeMatrix::new();
    nodes_matrix.arrange_matrix(&cover_matrix);
    nodes_matrix.search(0);

    convert_matrix_to_sudoku_grid(sudoku_board, nodes_matrix.actual_solution);
    pretty_print_board(&sudoku_board);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn convert_matrix_to_sudoku_grid
(
    sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    solution: Vec<StrongNode>
)
{
    for node in solution.clone() {
        let exact_cover_row_index = node.borrow().get_row().unwrap();
        let row = (exact_cover_row_index) / BOARD_SIZE_SQUARED as usize;
        let column = ((exact_cover_row_index) % BOARD_SIZE_SQUARED as usize) / BOARD_SIZE as usize;
        let mut value = (exact_cover_row_index) % BOARD_SIZE as usize;
        value = value + 1;
        if value == 0 {
            value = BOARD_SIZE as usize;
        }

        sudoku_board[row][column] = value;
    }
}

fn pretty_print_board(sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize])
{
    for i in 0..BOARD_SIZE * 2 + 1 {
        print!("-");
    }
    println!();
    for i in 0..sudoku_board.len() {
        for j in 0..sudoku_board[1].len() {
            print!("|");
            print!("{}", sudoku_board[i][j]);
        }
        print!("|");
        println!();
        for i in 0..BOARD_SIZE * 2 + 1 {
            print!("-");
        }
        println!();
    }
}

fn check_matrix_formed_properly(cover_matrix: &[[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize])
{
    for column in 0..EXACT_COVER_MATRIX_COLUMNS {
        let mut one_found = false;
        for i in 0..cover_matrix.len() {
            if cover_matrix[i][column as usize] == 1 {
                one_found = true;
            }
        }

        if !one_found {
            panic!("No 1's in column: {}", column);
        }
    }
}