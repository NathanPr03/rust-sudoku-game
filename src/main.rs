use std::rc::{Rc};
use crate::array_matrix::ArrayMatrix;
use crate::node_matrix::NodeMatrix;
use crate::node::{Node, OwnedNode, link_left, link_down};

mod ninebyninecovermatrix;
mod fourbyfourcovermatrix;
mod node;
mod node_matrix;
mod array_matrix;

const BOARD_SIZE: u16 = 9;
const BOARD_SIZE_SQUARED: u16 = BOARD_SIZE * BOARD_SIZE;

const CONSTRAINTS: [&str; 4] = ["Position", "Row", "Column", "Square"];
const NUM_OF_CONSTRAINTS: u16 = CONSTRAINTS.len() as u16;
const EXACT_COVER_MATRIX_COLUMNS: u16 = BOARD_SIZE_SQUARED * NUM_OF_CONSTRAINTS;
const EXACT_COVER_MATRIX_ROWS: u16 = BOARD_SIZE_SQUARED * BOARD_SIZE;

fn main() {

    println!("Columns: {}, Rows: {}", EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS);


    //Due to the way arrays work in rust its accessed cover_matrix[row_index][column_index]!!
    let mut cover_matrix:[[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
        = ninebyninecovermatrix::nine_by_nine_cover_matrix();

    println!("1st: {}", cover_matrix.len());
    println!("2nd: {}", cover_matrix[1].len());

    let mut array_matrix = ArrayMatrix::new(cover_matrix);
    array_matrix.generate_array_matrix();

    let mut nodes_matrix: NodeMatrix = NodeMatrix::new();
    nodes_matrix.arrange_matrix(&cover_matrix);

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

// This will return a module index, giving us circular links
pub fn get_previous_index(current_index: usize, length: usize) -> usize {
    return if current_index == 0 {
        // 0 indexed
        length - 1
    } else {
        current_index
    };
}

// This will return a module index, giving us circular links
pub fn get_next_index(current_index: usize, length: usize) -> usize {
    return if current_index == length {
        0
    } else {
        current_index
    }
}