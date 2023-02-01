use std::rc::{Rc};
use crate::node::{Node, OwnedNode, link_left, link_down};

mod ninebyninecovermatrix;
mod fourbyfourcovermatrix;
mod node;

const BOARD_SIZE: u16 = 9;
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
        = ninebyninecovermatrix::nine_by_nine_cover_matrix();

    println!("1st: {}", cover_matrix.len());
    println!("2nd: {}", cover_matrix[1].len());

    cell_constraint(&mut cover_matrix);
    row_constraint(&mut cover_matrix);
    column_constraint(&mut cover_matrix);
    region_constraint(&mut cover_matrix);

    array_of_arrays_to_nodes(&cover_matrix);
    print_board(&mut cover_matrix);
    // clue_to_exact_cover(&_board, &mut cover_matrix);
    // print_board(&mut cover_matrix);
}

fn cell_constraint
(
    cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
) -> ()
{
    let mut row_index: u16 = BOARD_SIZE;

    for column in 0..BOARD_SIZE_SQUARED {
        for row in row_index - BOARD_SIZE..row_index {
            cover_matrix[row as usize][column as usize] = 1;
        }
        row_index = row_index + BOARD_SIZE;
    }
}

fn row_constraint
(
    cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
) -> ()
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
}

fn column_constraint
(
    cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
) -> ()
{
    let board_size_squared_times_two: u16 = (BOARD_SIZE_SQUARED) + (BOARD_SIZE_SQUARED);

    let mut column: u16 = board_size_squared_times_two;
    for row in 0..EXACT_COVER_MATRIX_ROWS {
        if row % BOARD_SIZE_SQUARED == 0 && row > 1{
            column = board_size_squared_times_two;
        }
        cover_matrix[row as usize][column as usize] = 1;

        column += 1;
    }
}

fn region_constraint
(
    mut cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
) -> ()
{
    let board_size_squared_times_three: u16 = (BOARD_SIZE_SQUARED) + (BOARD_SIZE_SQUARED) + (BOARD_SIZE_SQUARED);
    let mut pullback: u16 = board_size_squared_times_three;
    let mut column: u16 = board_size_squared_times_three;

    let sqrt_board_size: u16 = (BOARD_SIZE as f32).sqrt() as u16;

    for row in 0..EXACT_COVER_MATRIX_ROWS {
        if row % (BOARD_SIZE_SQUARED * sqrt_board_size) == 0 && row > 1
        {
            pullback += BOARD_SIZE;
        } else if row % BOARD_SIZE_SQUARED == 0 && row > 1
        {
            pullback -= BOARD_SIZE * (sqrt_board_size - 1);

        } else if row % (BOARD_SIZE * sqrt_board_size) == 0 && row > 1
        {
            pullback += BOARD_SIZE;
        }

        if column % BOARD_SIZE == 0 {
            column = pullback;
        }

        cover_matrix[row as usize][column as usize] = 1;

        column += 1;
    }
}

fn print_board
(
    cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
)
{
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

fn array_of_arrays_to_nodes(
    cover_matrix: &[[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
) -> ()
{
    let mut special_header: OwnedNode = Node::new_root();

    let mut column_nodes: Vec<OwnedNode> = Vec::new();

    for column_index in 0..EXACT_COVER_MATRIX_COLUMNS {
        let column_header: OwnedNode = Node::new_header(Some(column_index as usize));
        link_left(&special_header, &Rc::downgrade(&column_header));
        column_nodes.push(column_header);
    }

    // Only using this so the nodes are always pointed to probably unecessary but fails without
    let mut some_nodes: Vec<OwnedNode> = Vec::new();

    for row_index in 0..EXACT_COVER_MATRIX_ROWS {
        for column_index in 0..EXACT_COVER_MATRIX_COLUMNS {
            if cover_matrix[row_index as usize][column_index as usize] == 1 {
                let header_node: &OwnedNode = &(column_nodes[column_index as usize]);

                let node: OwnedNode = Node::new_inner(header_node, row_index as usize);
                link_down(&header_node, &Rc::downgrade(&node));
                header_node.borrow_mut().inc_count();
                some_nodes.push(node);
            }
        }
    }
}





